#!/bin/bash

set -e

# Generate a transfer_public request
cd request_generator && cargo test test_transfer_public_local
# Generate a transfer_public authorization
cd ../authorization_generator && cargo test test_transfer_public_local
# Generate a fee request
cd ../request_generator && cargo test test_fee_public_local
# Generate a transfer_public transaction and verify its correctness
cd ../authorization_generator && cargo test test_transfer_public_local