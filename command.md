```bash
gcloud iam workload-identity-pools create "receive-canister-metrics" \
  --project="hot-or-not-monitoring" \
  --location="global" \
  --display-name="Receive canister metrics"

gcloud iam workload-identity-pools providers create-oidc "github-action" \
  --project="hot-or-not-monitoring" \
  --location="global" \
  --workload-identity-pool="receive-canister-metrics" \
  --display-name="Github Action" \
  --attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.aud=assertion.aud" \
  --issuer-uri="https://token.actions.githubusercontent.com"

gcloud iam service-accounts add-iam-policy-binding "github-action@hot-or-not-monitoring.iam.gserviceaccount.com" \
  --project="hot-or-not-monitoring" \
  --role="roles/iam.workloadIdentityUser" \
  --member="principalSet://iam.googleapis.com/projects/431003668404/locations/global/workloadIdentityPools/receive-canister-metrics/attribute.repository/go-bazzinga/receive_canister_metrics_from_fleet_and_push_to_timeseries_database"
```
