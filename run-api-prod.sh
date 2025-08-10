#!/bin/bash
set -o allexport
source .env
set +o allexport

# export RUST_LOG=info
export RUST_LOG=DEBUG

cd ./api

cargo run --release \
  -- \
  --auth0-jwks "$AUTH0_JWKS" \
  --auth0-audience "$AUTH0_AUDIENCE"
