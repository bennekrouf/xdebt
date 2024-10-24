#!/bin/bash

# Replace these with appropriate values
REPO_NAME="gpecs"  # Replace with the actual repository name you want to analyze
TENANT="mayorana"         # Replace with your tenant if needed

# Prepare the JSON payload for the request
REQUEST_PAYLOAD=$(cat <<EOF
{
  "repo_name": "$REPO_NAME"
}
EOF
)

# Use grpcurl to send the request to your gRPC service
grpcurl -plaintext \
    -rpc-header "tenant:$TENANT" \
    -d "$REQUEST_PAYLOAD" \
    0.0.0.0:50051 \
    analyze.Analyze/AnalyzeRepository
