# My Rust & AI Journey (`myrustjourney`)

A minimal Rust static site generator for a folder-tree blog hosted directly on
Google Cloud Storage (zero compute).

## Repository Layout
- `src/tree.rs` — Recursive folder/leaf Markdown loader (`index.md` for folder nodes, `*.md` for leaf blog posts)
- `src/ssg.rs` & `src/main.rs` — Static site generator (`cargo run -- build` -> `dist/`, `cargo run -- serve` for local preview)
- `templates/page.html` & `templates/not_found.html` — Minimal Vercel-style responsive blog templates
- `static/style.css` — Monochrome light/dark responsive CSS (mobile, tablet, desktop)
- `content/` — Folder tree of Markdown blog posts
- `.agents/skills/daily-learn/SKILL.md` — Daily learning & publishing workflow

## Rules & Conventions
- **Zero Compute**: Only generate pure static HTML/CSS in `dist/` and upload to Google Cloud Storage.
- **Keep It Minimal**: Only folder nodes (`index.md`) and leaf blog posts (`*.md`).
- **Daily Learning Entries**: Whenever the user shares daily learning notes or wants to publish a post, activate the `daily-learn` skill.
- **Verification**: Always run `cargo test` after editing Rust code or Markdown files under `content/`.
- **No Binaries or Build Output in Git**: Never commit `target/`, `dist/`, compiled binaries, or `.env`.
