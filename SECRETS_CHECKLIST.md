# GitHub Secrets Checklist for PartnerTools Deploy

Go to: GitHub → ModelEarth/team → Settings → Secrets and variables → Actions

## ✅ Already have these values (from your local env files)

| GitHub Secret Name | Value source | Status |
|---|---|---|
| `GEMINI_API_KEY` | `~/.env` → `GEMINI_API_KEY` | ✅ Ready to paste |
| `GCP_PROJECT_ID` | `anw-aetheric-envoy` (hardcoded in workflow now) | ✅ Not needed as secret |
| `GCP_USER_EMAIL` | Your Google account email | ✅ You know this |
| `PARTNER_TOOLS_DATABASE_URL` | `~/Documents/nexum-api.env` → `DATABASE_URL` | ✅ Neon.tech URL ready |
| `COMMONS_DB_HOST` | Parse from nexum-api.env DATABASE_URL | ✅ `ep-young-star-ahh8mgel-pooler.c-3.us-east-1.aws.neon.tech` |
| `COMMONS_DB_PORT` | `5432` | ✅ Standard Postgres |
| `COMMONS_DB_NAME` | Parse from nexum-api.env | ✅ `neondb` |
| `COMMONS_DB_USER` | Parse from nexum-api.env | ✅ `neondb_owner` |
| `COMMONS_DB_PASSWORD` | Parse from nexum-api.env | ✅ In nexum-api.env |
| `COMMONS_DB_SSL_MODE` | `require` | ✅ Neon always requires SSL |

## ⚠️ Still need to set up

| GitHub Secret Name | What it is | How to get it |
|---|---|---|
| `GCP_WORKLOAD_IDENTITY_PROVIDER` | OIDC provider resource name | Run `scripts/02_gcp_github_oidc.sh` (see DEPLOY_SETUP.md Step 4) |
| `GCP_SERVICE_ACCOUNT` | Deployer SA email | Created in DEPLOY_SETUP.md Step 3 |
| `GCP_ORG_ID` | GCP org ID | `gcloud organizations list` |
| `GCP_BILLING_ID` | Billing account ID | GCP Console → Billing |
| `PARTNER_TOOLS_GH_TOKEN` | GitHub PAT | GitHub → Settings → Developer settings → PAT |
| `GCP_SERVICE_KEY` | GCP service account JSON | From GCP Console (for Meetup integration) |

## 🔵 Exiobase DB — skip for now

The Exiobase secrets (`EXIOBASE_*`) are for the trade-flow data pipeline.
You can set them to empty strings or dummy values to get the API started,
then add real values later when you need that feature:
- `EXIOBASE_DB_HOST` = `localhost`
- `EXIOBASE_DB_NAME` = `exiobase`
- `EXIOBASE_DB_USER` = `user`
- `EXIOBASE_DB_PASSWORD` = `password`
- `EXIOBASE_DB_SSL_MODE` = `disable`

## Neon DB connection string breakdown

From `nexum-api.env`:
```
postgres://neondb_owner:<password>@ep-young-star-ahh8mgel-pooler.c-3.us-east-1.aws.neon.tech/neondb?sslmode=require
```

Parsed:
- HOST: `ep-young-star-ahh8mgel-pooler.c-3.us-east-1.aws.neon.tech`
- PORT: `5432`
- NAME: `neondb`
- USER: `neondb_owner`
- PASSWORD: (in nexum-api.env file)
- SSL_MODE: `require`

> **Note:** This Neon DB might be from the nexum project — check if you want to
> create a separate Neon DB for PartnerTools or reuse this one. Creating a new
> Neon project is free and takes 60 seconds at neon.tech.
