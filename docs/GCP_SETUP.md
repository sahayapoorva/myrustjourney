# Zero-Compute Static Hosting on Google Cloud Storage

This blog runs with **zero compute** (no Cloud Run, no VMs, no Docker). Rust compiles `content/` into pure static HTML/CSS files in `dist/`, which are stored in a **Google Cloud Storage** bucket in `us-central1` (**Always Free**: 5 GB storage + 100 GB egress/month).

## 1. One-Time Bucket Setup (`learnonethingatatime`)

```bash
export GCP_PROJECT_ID="learnonethingatatime"
export GCS_BUCKET="learnonethingatatime"

gcloud storage buckets create "gs://${GCS_BUCKET}" \
  --project="${GCP_PROJECT_ID}" \
  --location="us-central1" \
  --uniform-bucket-level-access

gcloud storage buckets update "gs://${GCS_BUCKET}" \
  --web-main-page-suffix="index.html" \
  --web-error-page="404.html"

gcloud storage buckets add-iam-policy-binding "gs://${GCS_BUCKET}" \
  --member="allUsers" \
  --role="roles/storage.objectViewer"
```

## 2. Build & Sync Static Files

```bash
GCS_BUCKET="learnonethingatatime" ./scripts/deploy.sh
```

Your blog is immediately live over HTTPS at:
`https://storage.googleapis.com/<GCS_BUCKET>/index.html`

## 3. Adding a Custom Domain + CloudFront-Style CDN ($0/month)

If you register a custom domain (e.g. `blog.yourdomain.com`):

1. **Free CDN + Free HTTPS (Recommended — $0/month)**:
   - Create a GCS bucket named exactly `blog.yourdomain.com` (after verifying domain ownership in Google Search Console) and sync `dist/` to it.
   - Add your domain to **Cloudflare (Free plan)** and point a proxied `CNAME` record for `blog` -> `c.storage.googleapis.com` with SSL set to **Flexible** (or **Full** via Cloudflare Worker/Transform Rule to `storage.googleapis.com/blog.yourdomain.com`).
   - Result: global CDN caching + custom HTTPS domain + GCS object store, **$0 compute and $0 load balancer fees**.

2. **Native GCP Cloud CDN**:
   - Note that Google Cloud's native Cloud CDN requires an External Application Load Balancer (~$18/month base fee), which exceeds $10/month credits.
