---
name: daily-learn
description: >-
  Guides the user's daily learning habit and consistent blog writing style in
  `myrustjourney`. Use when the user wants to log what they learned today, draft
  a new blog post from notes or a reference file, split a dense topic into a
  multipart series, update a folder node (`index.md`) or leaf blog post (`.md`)
  under `content/`, build static files, or publish to Google Cloud Storage.
---

# Daily Learn & Blog Writing Workflow (`myrustjourney`)

Turn daily notes or source files into small, concise, beginner-friendly Markdown
blog posts inside the `content/` folder tree, then compile to pure static
HTML/CSS in `dist/` for Google Cloud Storage.

## Golden Rules (Never Skip)

1. **Never assume — always ask when in doubt.**
2. **Always ask for the blog location** before creating any new file (inspect
   `content/` first and propose concrete paths under `content/notes/...`).
3. **Always include today's date** (`date: "YYYY-MM-DD"`) in YAML frontmatter.
4. **Always ask for references** to include at the end of the post (never invent
   references or silently drop them).
5. **Write for someone new to Rust** (or new to the topic): explain jargon
   briefly in plain English (e.g., stack vs. heap, two's complement, `usize`,
   `Option`, unit `()`) without assuming deep prior systems knowledge.
6. **Keep every post small and concise.** If the user's source material is too
   large to fit comfortably in one short post, propose splitting it into a
   **multipart blog series** (confirming the split and file paths first) and
   **always cross-reference the parts** so readers can navigate smoothly
   between them.
7. **Use provided reference files strictly for content** — rewrite and structure
   them into this blog's consistent voice and format rather than copying layout
   or interactive widgets verbatim.

## Content Tree Convention

- `content/index.md` — Root home page (`/`)
- `content/notes/index.md` — Notes overview (`/notes/`)
- `content/notes/<topic>/index.md` — Topic folder node (`/notes/<topic>/`)
- `content/notes/<topic>/<post-slug>.md` — Leaf blog post (`/notes/<topic>/<post-slug>/`)

## Consistent Post Style & Structure

Use this consistent structure across all leaf blog posts:

```markdown
---
title: "Clear, Specific Title"
date: "YYYY-MM-DD"
---

Short 1–2 sentence introduction framing the concept for someone new to Rust.
(For multipart posts, link to the other part(s) right here or at the bottom.)

## Focused Section Heading

Concise explanation in plain English, introducing any unfamiliar term inline.

```rust
// Short, runnable-looking Rust snippet with clear inline comments
```

## Key Takeaways

- 2–4 crisp, memorable takeaways in plain language.

---

**Series navigation** *(multipart posts only)*:
← Previous: [Part 1: ...](/notes/rust/...) | Next: [Part 2: ...](/notes/rust/...) →

## References

- [Reference Title](https://example.com)
```

### Style Guidelines
- Always capitalize the first letter of top navigation and breadcrumb labels
  (e.g., `Home`, `Notes`, `Home / Notes / Rust`, `Dark` / `Light`).
- Always tag fenced code blocks with their language (e.g. ` ```rust `) so
  syntax highlighting works cleanly.
- Use short paragraphs, bullet lists, and compact comparison tables over walls
  of text.
- Highlight subtle quirks/gotchas clearly right next to the concept they affect.
- Use root-anchored internal links like `/notes/rust/data-types-compound` — the
  static site generator (`src/ssg.rs`) automatically rewrites them into relative
  `index.html` links that work on Google Cloud Storage and custom CDN domains.

## Workflow Steps

1. **Inspect `content/`**: Check existing topic folders and recent posts to keep
   style and organization consistent.
2. **Clarify Before Writing**: Ask the user for:
   - Exact target location / filename(s)
   - Any references to include
   - Multipart split confirmation (if the source content is dense)
   - Whether to review locally first or deploy straight to GCS
3. **Optional Feynman Check**: When the user is dumping raw personal thoughts
   (rather than supplying a finished reference file), ask 1–2 brief questions to
   close knowledge gaps before drafting.
4. **Write the Markdown Post(s)**: Follow the Consistent Post Style above using
   today's date (`YYYY-MM-DD`).
5. **Verify & Build Locally**: Always run:
   ```bash
   cargo test
   cargo run -- build
   ```
   Update the gitignored `LINKS.local.md` with the new page URL(s), and offer
   `cargo run -- serve` for local browser review.
6. **Publish (Only When Explicitly Asked)**: Run `./scripts/deploy.sh` (or
   commit and push to `main`) to sync `dist/` to Google Cloud Storage.
