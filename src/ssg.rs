use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use askama::Template;

use crate::tree::{NavItem, Node};

const STYLESHEET: &str = include_str!("../static/style.css");
const DIAGRAMS_JS: &str = include_str!("../static/diagrams.js");

struct NavLink {
    label: String,
    href: String,
    active: bool,
}

struct CrumbLink {
    title: String,
    href: String,
}

struct ChildEntry {
    title: String,
    href: String,
    meta_label: String,
    is_folder: bool,
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate<'a> {
    site_title: &'a str,
    css_href: String,
    diagrams_href: String,
    top_nav: Vec<NavLink>,
    breadcrumbs: Vec<CrumbLink>,
    title: &'a str,
    is_folder: bool,
    iso_date: Option<String>,
    display_date: Option<String>,
    body_html: String,
    show_children: bool,
    children: Vec<ChildEntry>,
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate<'a> {
    site_title: &'a str,
    css_href: String,
    top_nav: Vec<NavLink>,
}

/// Build the entire folder-tree blog into pure static HTML & CSS files under `out_dir`.
pub fn build_site(content_dir: &Path, out_dir: &Path, site_title: &str) -> Result<usize> {
    let root = Node::load_tree(content_dir)?;

    if out_dir.exists() {
        fs::remove_dir_all(out_dir)
            .with_context(|| format!("Failed to clean {}", out_dir.display()))?;
    }
    fs::create_dir_all(out_dir.join("static"))?;

    // Write static assets
    fs::write(out_dir.join("static/style.css"), STYLESHEET)?;
    fs::write(out_dir.join("static/diagrams.js"), DIAGRAMS_JS)?;

    let raw_top_nav = root.top_nav();
    let mut written = 0usize;
    render_recursive(
        &root,
        &root,
        &raw_top_nav,
        out_dir,
        site_title,
        &mut written,
    )?;

    // Write 404.html for GCS / CDN NotFoundPage
    let not_found_nav: Vec<NavLink> = raw_top_nav
        .iter()
        .map(|item| NavLink {
            label: item.label.clone(),
            href: root_absolute_href(&item.path),
            active: false,
        })
        .collect();
    let not_found_html = NotFoundTemplate {
        site_title,
        css_href: "/static/style.css".to_string(),
        top_nav: not_found_nav,
    }
    .render()?;
    fs::write(out_dir.join("404.html"), not_found_html)?;

    Ok(written)
}

fn render_recursive(
    root: &Node,
    node: &Node,
    raw_top_nav: &[NavItem],
    out_dir: &Path,
    site_title: &str,
    written: &mut usize,
) -> Result<()> {
    let root_prefix = relative_root_prefix(&node.path);

    let top_nav: Vec<NavLink> = raw_top_nav
        .iter()
        .map(|item| {
            let active = node.path == item.path
                || (item.path != "/" && node.path.starts_with(&format!("{}/", item.path)));
            NavLink {
                label: item.label.clone(),
                href: rel_href(&root_prefix, &item.path),
                active,
            }
        })
        .collect();

    let breadcrumbs: Vec<CrumbLink> = root
        .breadcrumbs(&node.path)
        .into_iter()
        .map(|c| CrumbLink {
            title: c.title,
            href: rel_href(&root_prefix, &c.path),
        })
        .collect();

    let show_children = node.is_folder() && node.path != "/" && !node.children.is_empty();
    let children: Vec<ChildEntry> = if show_children {
        node.children
            .iter()
            .map(|child| {
                let meta_label = if child.is_folder() {
                    "folder".to_string()
                } else if let Some(d) = &child.display_date {
                    d.clone()
                } else {
                    "post".to_string()
                };
                ChildEntry {
                    title: child.title.clone(),
                    href: rel_href(&root_prefix, &child.path),
                    meta_label,
                    is_folder: child.is_folder(),
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    let body_html = rewrite_internal_links(&node.body_html, &root_prefix);

    let html = PageTemplate {
        site_title,
        css_href: format!("{root_prefix}static/style.css"),
        diagrams_href: format!("{root_prefix}static/diagrams.js"),
        top_nav,
        breadcrumbs,
        title: &node.title,
        is_folder: node.is_folder(),
        iso_date: node.date.clone(),
        display_date: node.display_date.clone(),
        body_html,
        show_children,
        children,
    }
    .render()?;

    let target_file = if node.path == "/" {
        out_dir.join("index.html")
    } else {
        let rel = node.path.trim_start_matches('/');
        let dir = out_dir.join(rel);
        fs::create_dir_all(&dir)?;
        dir.join("index.html")
    };

    fs::write(&target_file, html)
        .with_context(|| format!("Failed to write {}", target_file.display()))?;
    *written += 1;

    for child in &node.children {
        render_recursive(root, child, raw_top_nav, out_dir, site_title, written)?;
    }

    Ok(())
}

/// Depth prefix from `url_path`'s `index.html` back to the site root.
fn relative_root_prefix(url_path: &str) -> String {
    let trimmed = url_path.trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        let depth = trimmed.split('/').count();
        "../".repeat(depth)
    }
}

fn rel_href(root_prefix: &str, target_path: &str) -> String {
    let trimmed = target_path.trim_matches('/');
    if trimmed.is_empty() {
        format!("{root_prefix}index.html")
    } else {
        format!("{root_prefix}{trimmed}/index.html")
    }
}

fn root_absolute_href(target_path: &str) -> String {
    let trimmed = target_path.trim_matches('/');
    if trimmed.is_empty() {
        "/index.html".to_string()
    } else {
        format!("/{trimmed}/index.html")
    }
}

/// Rewrite Markdown-rendered internal links like `href="/notes/rust"` to relative
/// `index.html` URLs so links inside prose also work seamlessly on pure GCS object storage.
fn rewrite_internal_links(html: &str, root_prefix: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(idx) = rest.find("href=\"/") {
        result.push_str(&rest[..idx]);
        let after_prefix = &rest[idx + 6..];
        let Some(end_quote) = after_prefix.find('"') else {
            result.push_str(&rest[idx..]);
            return result;
        };

        let raw_href = &after_prefix[..end_quote];
        if raw_href.starts_with("//") {
            result.push_str("href=\"");
            result.push_str(raw_href);
            result.push('"');
        } else if raw_href.starts_with("/static/") {
            result.push_str("href=\"");
            result.push_str(root_prefix);
            result.push_str(raw_href.trim_start_matches('/'));
            result.push('"');
        } else {
            let (path_part, anchor) = match raw_href.split_once('#') {
                Some((p, a)) => (p, Some(a)),
                None => (raw_href, None),
            };
            result.push_str("href=\"");
            result.push_str(&rel_href(root_prefix, path_part));
            if let Some(a) = anchor {
                result.push('#');
                result.push_str(a);
            }
            result.push('"');
        }

        rest = &after_prefix[end_quote + 1..];
    }

    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_complete_static_site_with_relative_links() {
        let src = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();

        fs::write(
            src.path().join("index.md"),
            "# Home\n\nRead [Rust](/notes/rust) or [The Book](https://doc.rust-lang.org/stable/book/).",
        )
        .unwrap();
        fs::create_dir_all(src.path().join("notes/rust")).unwrap();
        fs::write(src.path().join("notes/index.md"), "# Notes\n\nAll notes.").unwrap();
        fs::write(
            src.path().join("notes/rust/index.md"),
            "# Rust\n\nRust notes.",
        )
        .unwrap();
        fs::write(
            src.path().join("notes/rust/shadowing.md"),
            "---\ntitle: \"Shadowing\"\ndate: \"2026-11-21\"\n---\n\nShadowing body.",
        )
        .unwrap();

        let count = build_site(src.path(), out.path(), "Apoorva Sahay").unwrap();
        assert_eq!(count, 4);

        let home = fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(home.contains("href=\"static/style.css\""));
        assert!(home.contains("href=\"notes/rust/index.html\""));
        assert!(home.contains("https://doc.rust-lang.org/stable/book/"));

        let leaf = fs::read_to_string(out.path().join("notes/rust/shadowing/index.html")).unwrap();
        assert!(leaf.contains("href=\"../../../static/style.css\""));
        assert!(leaf.contains("href=\"../../../index.html\""));
        assert!(leaf.contains("Shadowing body."));

        let css = fs::read_to_string(out.path().join("static/style.css")).unwrap();
        assert!(css.contains(".shell"));
        assert!(out.path().join("404.html").is_file());
        assert!(!out.path().join("stats/index.html").exists());
    }

    #[test]
    fn builds_real_workspace_content_cleanly() {
        let content_dir = Path::new("content");
        if content_dir.is_dir() {
            let out = tempfile::tempdir().unwrap();
            let count = build_site(content_dir, out.path(), "Apoorva Sahay").unwrap();
            assert!(count >= 4);
            assert!(out.path().join("index.html").is_file());
            assert!(out.path().join("notes/index.html").is_file());
            assert!(out.path().join("notes/rust/shadowing/index.html").is_file());
            assert!(out.path().join("static/style.css").is_file());
            assert!(out.path().join("static/diagrams.js").is_file());
        }
    }
}
