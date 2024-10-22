
#!/bin/bash

SERVER_ADDRESS="0.0.0.0:50051"
SERVICE="analyze.Analyze"
METHOD="AnalyzeRepository"
REQUEST_FILE="request.json"

# Number of times to run the test
RUNS=1

# Example request JSON (you can modify this with your repository names)
cat <<EOF > $REQUEST_FILE
{
  "repo_name": "gpecs"
}
EOF

# Loop to call the gRPC server multiple times
for (( i=1; i<=$RUNS; i++ ))
do
    echo "Running gRPC call #$i"
    grpcurl -d "$(cat $REQUEST_FILE)" -plaintext $SERVER_ADDRESS $SERVICE/$METHOD
    echo ""
done

# Clean up
rm $REQUEST_FILE

