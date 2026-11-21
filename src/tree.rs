use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Folder,
    Leaf,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub path: String,
    pub title: String,
    pub date: Option<String>,
    pub display_date: Option<String>,
    pub order: Option<i32>,
    pub kind: NodeKind,
    pub body_html: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breadcrumb {
    pub title: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    pub label: String,
    pub path: String,
}

#[derive(Debug, Default, Deserialize)]
struct Frontmatter {
    title: Option<String>,
    date: Option<String>,
    order: Option<i32>,
}

struct ParsedDoc {
    title: String,
    date: Option<String>,
    display_date: Option<String>,
    order: Option<i32>,
    body_html: String,
}

impl Node {
    /// Recursively load the folder tree rooted at `content_dir`.
    ///
    /// - Every folder (including the root) is a `NodeKind::Folder` whose blog post
    ///   comes from `index.md` inside that folder.
    /// - Every other `.md` file inside a folder is a `NodeKind::Leaf` blog post.
    pub fn load_tree(content_dir: &Path) -> Result<Self> {
        if !content_dir.is_dir() {
            bail!(
                "Content directory does not exist: {}",
                content_dir.display()
            );
        }
        load_folder(content_dir, "", "home")
    }

    /// Look up any node (folder or leaf) by its URL path (e.g. `"/"`, `"/rust"`, `"/rust/ownership"`).
    pub fn find(&self, url_path: &str) -> Option<&Node> {
        let normalized = normalize_url_path(url_path);
        if self.path == normalized {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find(&normalized) {
                return Some(found);
            }
        }
        None
    }

    /// Return the breadcrumb trail from the root node down to `url_path`.
    pub fn breadcrumbs(&self, url_path: &str) -> Vec<Breadcrumb> {
        let normalized = normalize_url_path(url_path);
        let mut trail = Vec::new();
        self.collect_breadcrumbs(&normalized, &mut trail);
        trail
    }

    fn collect_breadcrumbs(&self, target: &str, trail: &mut Vec<Breadcrumb>) -> bool {
        trail.push(Breadcrumb {
            title: if self.path == "/" {
                "Home".to_string()
            } else {
                capitalize_first(&self.name)
            },
            path: self.path.clone(),
        });

        if self.path == target {
            return true;
        }

        for child in &self.children {
            if child.collect_breadcrumbs(target, trail) {
                return true;
            }
        }

        trail.pop();
        false
    }

    /// Top-level navigation items: `Home` + each top-level folder under `content/`
    /// with the first letter capitalized.
    pub fn top_nav(&self) -> Vec<NavItem> {
        let mut items = vec![NavItem {
            label: "Home".to_string(),
            path: "/".to_string(),
        }];
        for child in &self.children {
            if child.is_folder() {
                items.push(NavItem {
                    label: capitalize_first(&child.name),
                    path: child.path.clone(),
                });
            }
        }
        items
    }

    pub fn is_folder(&self) -> bool {
        self.kind == NodeKind::Folder
    }

    pub fn count_nodes(&self) -> usize {
        1 + self.children.iter().map(Node::count_nodes).sum::<usize>()
    }

    pub fn leaf_count(&self) -> usize {
        match self.kind {
            NodeKind::Leaf => 1,
            NodeKind::Folder => self.children.iter().map(Node::leaf_count).sum(),
        }
    }
}

fn load_folder(dir: &Path, url_prefix: &str, name: &str) -> Result<Node> {
    let url_path = if url_prefix.is_empty() {
        "/".to_string()
    } else {
        url_prefix.to_string()
    };

    let index_file = dir.join("index.md");
    let doc = if index_file.is_file() {
        parse_markdown_file(&index_file, name)?
    } else {
        ParsedDoc {
            title: prettify_name(name),
            date: None,
            display_date: None,
            order: None,
            body_html: String::new(),
        }
    };

    let mut children = Vec::new();

    for entry in
        fs::read_dir(dir).with_context(|| format!("Failed to read directory {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if file_name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            let child_url = format!("{}/{}", url_prefix, file_name);
            let child_node = load_folder(&path, &child_url, file_name)?;
            children.push(child_node);
        } else if path.is_file()
            && path.extension().and_then(|e| e.to_str()) == Some("md")
            && !file_name.eq_ignore_ascii_case("index.md")
        {
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", path.display()))?;
            let child_url = format!("{}/{}", url_prefix, stem);
            let leaf_doc = parse_markdown_file(&path, stem)?;

            children.push(Node {
                name: stem.to_string(),
                path: child_url,
                title: leaf_doc.title,
                date: leaf_doc.date,
                display_date: leaf_doc.display_date,
                order: leaf_doc.order,
                kind: NodeKind::Leaf,
                body_html: leaf_doc.body_html,
                children: Vec::new(),
            });
        }
    }

    // Sort: folders first (by order/name), then leaves (newest date first, then order/name).
    children.sort_by(|a, b| {
        let kind_rank = |k: NodeKind| match k {
            NodeKind::Folder => 0u8,
            NodeKind::Leaf => 1u8,
        };
        kind_rank(a.kind)
            .cmp(&kind_rank(b.kind))
            .then_with(|| {
                a.order
                    .unwrap_or(i32::MAX)
                    .cmp(&b.order.unwrap_or(i32::MAX))
            })
            .then_with(|| {
                if a.kind == NodeKind::Leaf {
                    b.date.cmp(&a.date)
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .then_with(|| {
                a.name
                    .to_ascii_lowercase()
                    .cmp(&b.name.to_ascii_lowercase())
            })
    });

    Ok(Node {
        name: name.to_string(),
        path: url_path,
        title: doc.title,
        date: doc.date,
        display_date: doc.display_date,
        order: doc.order,
        kind: NodeKind::Folder,
        body_html: doc.body_html,
        children,
    })
}

fn parse_markdown_file(path: &Path, fallback_name: &str) -> Result<ParsedDoc> {
    let raw =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let normalized = raw.replace("\r\n", "\n");

    let (frontmatter, body_md) = if let Some(rest) = normalized.strip_prefix("---\n") {
        if let Some((yaml, body)) = rest.split_once("\n---\n") {
            let fm: Frontmatter = serde_yaml::from_str(yaml)
                .with_context(|| format!("Invalid YAML frontmatter in {}", path.display()))?;
            (fm, body.trim())
        } else {
            (Frontmatter::default(), normalized.trim())
        }
    } else {
        (Frontmatter::default(), normalized.trim())
    };

    let (derived_title, body_without_h1) = extract_leading_h1(body_md);
    let title = frontmatter
        .title
        .filter(|t| !t.trim().is_empty())
        .or(derived_title)
        .unwrap_or_else(|| prettify_name(fallback_name));

    let render_md = if body_md.starts_with("# ") {
        body_without_h1
    } else {
        body_md
    };

    let display_date = frontmatter.date.as_deref().map(format_display_date);
    let body_html = render_markdown(render_md);

    Ok(ParsedDoc {
        title,
        date: frontmatter.date,
        display_date,
        order: frontmatter.order,
        body_html,
    })
}

fn extract_leading_h1(md: &str) -> (Option<String>, &str) {
    if let Some(rest) = md.strip_prefix("# ") {
        if let Some((heading, remainder)) = rest.split_once('\n') {
            return (Some(heading.trim().to_string()), remainder.trim_start());
        }
        return (Some(rest.trim().to_string()), "");
    }
    (None, md)
}

pub fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

/// Format `"YYYY-MM-DD"` as `"November 21, 2026"` (falls back to input if non-ISO).
pub fn format_display_date(raw: &str) -> String {
    let parts: Vec<&str> = raw.trim().split('-').collect();
    if parts.len() != 3 {
        return raw.to_string();
    }
    let month_name = match parts[1] {
        "01" => "January",
        "02" => "February",
        "03" => "March",
        "04" => "April",
        "05" => "May",
        "06" => "June",
        "07" => "July",
        "08" => "August",
        "09" => "September",
        "10" => "October",
        "11" => "November",
        "12" => "December",
        _ => return raw.to_string(),
    };
    let day = parts[2].trim_start_matches('0');
    let day = if day.is_empty() { "0" } else { day };
    format!("{month_name} {day}, {}", parts[0])
}

pub fn normalize_url_path(raw: &str) -> String {
    let trimmed = raw.trim().trim_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn capitalize_first(input: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let mut chars = lower.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn prettify_name(name: &str) -> String {
    name.split(['-', '_'])
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_nested_folders_and_leaf_blogs() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();

        fs::write(root.join("index.md"), "# Home\n\nWelcome!").unwrap();
        fs::create_dir_all(root.join("rust/web")).unwrap();
        fs::write(
            root.join("rust/index.md"),
            "---\ntitle: \"Rust\"\norder: 1\n---\n\nRust overview.",
        )
        .unwrap();
        fs::write(
            root.join("rust/ownership.md"),
            "---\ntitle: \"Ownership\"\ndate: \"2026-11-21\"\n---\n\nOwnership notes.",
        )
        .unwrap();
        fs::write(root.join("rust/web/index.md"), "# Web Dev\n\nAxum notes.").unwrap();
        fs::write(
            root.join("rust/web/routing.md"),
            "# Routing\n\nRoute guide.",
        )
        .unwrap();

        let tree = Node::load_tree(root).unwrap();
        assert_eq!(tree.title, "Home");
        assert_eq!(tree.count_nodes(), 5);
        assert_eq!(tree.leaf_count(), 2);
        let nav = tree.top_nav();
        assert_eq!(nav.len(), 2);
        assert_eq!(nav[0].label, "Home");
        assert_eq!(nav[1].label, "Rust");

        let rust = tree.find("/rust").unwrap();
        assert!(rust.is_folder());
        assert_eq!(rust.title, "Rust");
        assert!(rust.body_html.contains("Rust overview."));

        let leaf = tree.find("/rust/ownership").unwrap();
        assert!(!leaf.is_folder());
        assert_eq!(leaf.display_date.as_deref(), Some("November 21, 2026"));

        let crumbs = tree.breadcrumbs("/rust/web/routing");
        assert_eq!(crumbs.len(), 4);
        assert_eq!(crumbs[0].path, "/");
        assert_eq!(crumbs[0].title, "Home");
        assert_eq!(crumbs[1].path, "/rust");
        assert_eq!(crumbs[1].title, "Rust");
        assert_eq!(crumbs[2].path, "/rust/web");
        assert_eq!(crumbs[2].title, "Web");
        assert_eq!(crumbs[3].path, "/rust/web/routing");
        assert_eq!(crumbs[3].title, "Routing");
    }

    #[test]
    fn workspace_content_directory_loads_cleanly() {
        let content_dir = Path::new("content");
        if content_dir.is_dir() {
            let tree = Node::load_tree(content_dir).unwrap();
            assert!(tree.count_nodes() >= 1);
        }
    }
}
