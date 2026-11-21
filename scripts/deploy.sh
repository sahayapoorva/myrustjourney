#!/usr/bin/env bash
set -euo pipefail

# Zero-compute static deployment to Google Cloud Storage (Object Store).
# Usage:
#   GCS_BUCKET=learnonethingatatime ./scripts/deploy.sh

GCS_BUCKET="${GCS_BUCKET:-learnonethingatatime}"

echo "==> Running tests & building static HTML/CSS into dist/..."
cargo test --offline
cargo run --offline -- build

echo "==> Syncing dist/ to gs://${GCS_BUCKET}..."
gcloud storage rsync --recursive --delete-unmatched-destination-objects dist "gs://${GCS_BUCKET}"

echo "==> Setting short cache-control headers so blog updates appear quickly..."
gcloud storage objects update "gs://${GCS_BUCKET}/**/*.html" --cache-control="public, max-age=60"
gcloud storage objects update "gs://${GCS_BUCKET}/static/style.css" --cache-control="public, max-age=300"

echo "==> Live at: https://storage.googleapis.com/${GCS_BUCKET}/index.html"
