//! Minimal Rust static site generator for a folder-tree blog.
//!
//! Compiles `content/` Markdown nodes (folders via `index.md`, leaf posts via `*.md`)
//! into pure static HTML & CSS in `dist/` for zero-compute hosting on Google Cloud Storage.

pub mod ssg;
pub mod tree;
