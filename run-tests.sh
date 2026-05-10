#!/bin/bash

# Start Solana test validator in background
solana-test-validator &

# Store PID if you want to stop it later
VALIDATOR_PID=$!

echo "Waiting for validator to start..."

# Wait a few seconds
sleep 5

# Run anchor tests
anchor test --skip-local-validator

# Optional: stop validator after tests
kill $VALIDATOR_PID