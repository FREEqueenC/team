#!/bin/bash
# Run this once from Documents/GitHub/team
# All required APIs are already enabled on anw-aetheric-envoy

set -e

PROJECT_ID="anw-aetheric-envoy"
REGION="us-central1"
REPO="FREEqueenC/team"
SA_NAME="partner-tools-deployer"
POOL_NAME="github-pool"
PROVIDER_NAME="github-provider"

echo "=== Step 1: Set active project ==="
gcloud config set project $PROJECT_ID
gcloud config set account ashleighwalker@anwfoundations.com

echo ""
echo "=== Step 2: Create Artifact Registry repo ==="
gcloud artifacts repositories create partner-tools-repo \
  --repository-format=docker \
  --location=$REGION \
  --description="PartnerTools Docker images" \
  --project=$PROJECT_ID 2>/dev/null || echo "Repo already exists, skipping."

echo ""
echo "=== Step 3: Create deployer service account ==="
SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

gcloud iam service-accounts create $SA_NAME \
  --display-name="PartnerTools GitHub Actions Deployer" \
  --project=$PROJECT_ID 2>/dev/null || echo "SA already exists, skipping."

echo "Granting roles..."
gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/run.admin" --quiet

gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/artifactregistry.writer" --quiet

gcloud projects add-iam-policy-binding $PROJECT_ID \
  --member="serviceAccount:${SA_EMAIL}" \
  --role="roles/iam.serviceAccountUser" --quiet

echo ""
echo "=== Step 4: Create Workload Identity Pool ==="
gcloud iam workload-identity-pools create $POOL_NAME \
  --location="global" \
  --display-name="GitHub Actions Pool" \
  --project=$PROJECT_ID 2>/dev/null || echo "Pool already exists, skipping."

echo ""
echo "=== Step 5: Create OIDC Provider ==="
gcloud iam workload-identity-pools providers create-oidc $PROVIDER_NAME \
  --location="global" \
  --workload-identity-pool=$POOL_NAME \
  --display-name="GitHub OIDC" \
  --issuer-uri="https://token.actions.githubusercontent.com" \
  --attribute-mapping="google.subject=assertion.sub,attribute.repository=assertion.repository,attribute.actor=assertion.actor" \
  --attribute-condition="assertion.repository=='${REPO}'" \
  --project=$PROJECT_ID 2>/dev/null || echo "Provider already exists, skipping."

echo ""
echo "=== Step 6: Allow GitHub Actions to impersonate the SA ==="
PROJECT_NUMBER=$(gcloud projects describe $PROJECT_ID --format='value(projectNumber)')
POOL_ID="projects/${PROJECT_NUMBER}/locations/global/workloadIdentityPools/${POOL_NAME}"

gcloud iam service-accounts add-iam-policy-binding $SA_EMAIL \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/${POOL_ID}/attribute.repository/${REPO}" \
  --project=$PROJECT_ID --quiet

echo ""
echo "============================================"
echo "DONE. Paste these into GitHub secrets:"
echo "https://github.com/${REPO}/settings/secrets/actions"
echo "============================================"
echo ""
echo "GCP_WORKLOAD_IDENTITY_PROVIDER:"
gcloud iam workload-identity-pools providers describe $PROVIDER_NAME \
  --location="global" \
  --workload-identity-pool=$POOL_NAME \
  --project=$PROJECT_ID \
  --format='value(name)'

echo ""
echo "GCP_SERVICE_ACCOUNT:"
echo $SA_EMAIL
