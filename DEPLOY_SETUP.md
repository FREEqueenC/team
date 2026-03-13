# PartnerTools Cloud Run Deployment — Setup Guide

This document walks through the one-time setup needed to make
`deploy-team-cloudrun.yml` work. You only do this once; after that,
every `git push` to `main` auto-deploys.

---

## Prerequisites

- `gcloud` CLI installed and authenticated (`gcloud auth login`)
- Your GCP project ID for anwfoundations.com
- Admin access to the GitHub repo (`model.earth/team` or wherever it lives)

---

## Step 1 — Enable Required GCP APIs

```bash
gcloud services enable \
  run.googleapis.com \
  artifactregistry.googleapis.com \
  iam.googleapis.com \
  iamcredentials.googleapis.com \
  cloudresourcemanager.googleapis.com \
  --project=YOUR_PROJECT_ID
```

---

## Step 2 — Create Artifact Registry repo (one-time)

```bash
gcloud artifacts repositories create partner-tools-repo \
  --repository-format=docker \
  --location=us-central1 \
  --description="PartnerTools Docker images" \
  --project=YOUR_PROJECT_ID
```

---

## Step 3 — Create a deployer Service Account

```bash
gcloud iam service-accounts create partner-tools-deployer \
  --display-name="PartnerTools GitHub Actions Deployer" \
  --project=YOUR_PROJECT_ID

SA_EMAIL="partner-tools-deployer@YOUR_PROJECT_ID.iam.gserviceaccount.com"

# Grant only what's needed
gcloud projects add-iam-policy-binding YOUR_PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/run.admin"

gcloud projects add-iam-policy-binding YOUR_PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/artifactregistry.writer"

gcloud projects add-iam-policy-binding YOUR_PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/iam.serviceAccountUser"
```

---

## Step 4 — Set up Workload Identity Federation (OIDC, no keys)

This is the "no service account key" approach GitHub recommends.

```bash
PROJECT_ID="YOUR_PROJECT_ID"
PROJECT_NUMBER=$(gcloud projects describe $PROJECT_ID --format='value(projectNumber)')
REPO="YOUR_GITHUB_ORG/team"   # e.g. modelearth/team

# Create the pool
gcloud iam workload-identity-pools create "github-pool" \
  --location="global" \
  --display-name="GitHub Actions Pool" \
  --project=$PROJECT_ID

# Create the OIDC provider inside the pool
gcloud iam workload-identity-pools providers create-oidc "github-provider" \
  --location="global" \
  --workload-identity-pool="github-pool" \
  --display-name="GitHub OIDC" \
  --issuer-uri="https://token.actions.githubusercontent.com" \
  --attribute-mapping="google.subject=assertion.sub,attribute.repository=assertion.repository,attribute.actor=assertion.actor" \
  --attribute-condition="assertion.repository=='${REPO}'" \
  --project=$PROJECT_ID

# Allow the pool/provider to impersonate the deployer SA
POOL_ID="projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/github-pool"
SA_EMAIL="partner-tools-deployer@${PROJECT_ID}.iam.gserviceaccount.com"

gcloud iam service-accounts add-iam-policy-binding $SA_EMAIL \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/${POOL_ID}/attribute.repository/${REPO}" \
  --project=$PROJECT_ID

# Print the two values you need for GitHub secrets
echo ""
echo "=== COPY THESE INTO GITHUB SECRETS ==="
echo ""
echo "GCP_WORKLOAD_IDENTITY_PROVIDER:"
gcloud iam workload-identity-pools providers describe github-provider \
  --location="global" \
  --workload-identity-pool="github-pool" \
  --project=$PROJECT_ID \
  --format='value(name)'
echo ""
echo "GCP_SERVICE_ACCOUNT:"
echo $SA_EMAIL
```

---

## Step 5 — Add GitHub Repository Secrets

Go to: **GitHub repo → Settings → Secrets and variables → Actions → New repository secret**

### Required for OIDC auth (from Step 4 output):
| Secret name | Value |
|---|---|
| `GCP_WORKLOAD_IDENTITY_PROVIDER` | Full provider resource name printed by Step 4 |
| `GCP_SERVICE_ACCOUNT` | `partner-tools-deployer@YOUR_PROJECT_ID.iam.gserviceaccount.com` |

### Required for the app:
| Secret name | Value |
|---|---|
| `GCP_PROJECT_ID` | Your GCP project ID |
| `GCP_ORG_ID` | Your GCP org ID (from `gcloud organizations list`) |
| `GCP_USER_EMAIL` | Your Google account email |
| `GCP_BILLING_ID` | Your billing account ID |
| `GEMINI_API_KEY` | Gemini API key from AI Studio |
| `PARTNER_TOOLS_DATABASE_URL` | Full PostgreSQL URL e.g. `postgresql://user:pass@host/db` |
| `COMMONS_DB_HOST` | PostgreSQL host |
| `COMMONS_DB_PORT` | PostgreSQL port (usually `5432`) |
| `COMMONS_DB_NAME` | Database name |
| `COMMONS_DB_USER` | Database user |
| `COMMONS_DB_PASSWORD` | Database password |
| `COMMONS_DB_SSL_MODE` | `require` or `disable` |
| `EXIOBASE_DB_HOST` | Exiobase DB host |
| `EXIOBASE_DB_NAME` | Exiobase DB name |
| `EXIOBASE_DB_USER` | Exiobase DB user |
| `EXIOBASE_DB_PASSWORD` | Exiobase DB password |
| `EXIOBASE_DB_SSL_MODE` | `require` or `disable` |
| `PARTNER_TOOLS_GH_TOKEN` | GitHub Personal Access Token |
| `GCP_SERVICE_KEY` | Google service account JSON (base64 or raw JSON) |

> **Tip:** For the database secrets, if you don't have a Cloud SQL instance yet,
> you can start with a free-tier PostgreSQL on Render.com or Neon.tech —
> just set `COMMONS_DB_SSL_MODE=require` and plug in their connection string.

---

## Step 6 — Trigger the deploy

```bash
cd /path/to/team-repo
git add .github/workflows/deploy-team-cloudrun.yml DEPLOY_SETUP.md
git commit -m "feat: add Cloud Run deploy workflow via GitHub OIDC"
git push origin main
```

That push will trigger the workflow. Watch it at:
**GitHub repo → Actions → Deploy PartnerTools to Cloud Run**

---

## What the workflow does

```
push to main
    │
    ▼
checkout code
    │
    ▼
authenticate to GCP (OIDC — no keys stored anywhere)
    │
    ▼
docker build → push to Artifact Registry
    │
    ▼
gcloud run deploy (Cloud Run managed, port 8081, 0 min instances)
    │
    ▼
print live URL  ← share this with ModelEarth contributors
```

---

## Cost estimate (anwfoundations.com / small traffic)

| Resource | Cost |
|---|---|
| Cloud Run (0 min instances, ~1 req/min) | ~$0/mo (free tier covers it) |
| Artifact Registry storage (<1 GB) | ~$0.10/mo |
| Cloud Build (if used) | Free 120 min/day |
| **Total** | **~$0–$2/mo** |

Zero-to-one is basically free on Cloud Run.
