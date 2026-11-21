# Learning One Thing at a Time (`myrustjourney`)

A minimal **Rust static site generator** for a folder-tree blog hosted with **zero compute** in **Google Cloud Storage**.

## Folder Structure

Every folder under `content/` is a node (`index.md`), and every other `.md` file is a leaf blog post:

```text
content/
├── index.md
└── notes/
    ├── index.md
    ├── rust/
    │   ├── index.md
    │   └── shadowing.md
    └── ai/
        ├── index.md
        └── getting-started-with-gemini.md
```

## Build & Preview Locally

```bash
# Run tests
cargo test

# Generate pure static HTML/CSS in dist/
cargo run -- build

# Preview dist/ locally at http://127.0.0.1:8080/
cargo run -- serve
```

## Deploy to Google Cloud Storage (Zero Compute)

```bash
GCS_BUCKET=learnonethingatatime ./scripts/deploy.sh
```

See [docs/GCP_SETUP.md](docs/GCP_SETUP.md) for bucket setup and connecting a free CDN custom domain.
