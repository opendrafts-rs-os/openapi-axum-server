#!/bin/bash
set -o allexport
source .env
set +o allexport

# export RUST_LOG=info
export RUST_LOG=DEBUG

#../gui/build.sh

cargo run \
  -- \
  --auth0-domain "$AUTH0_DOMAIN" \
  --auth0-client-id "$AUTH0_CLIENT_ID" \
  --auth0-client-secret "$AUTH0_CLIENT_SECRET" \
  --auth0-redirect-uri "$AUTH0_REDIRECT_URI" \
  --auth0-scope "$AUTH0_SCOPE" \

  # --auth0-response-type "$EXAMPLE_RESPONSE_TYPE" \