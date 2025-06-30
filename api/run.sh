#!/bin/bash
set -o allexport
source .env
set +o allexport

# export RUST_LOG=info
export RUST_LOG=DEBUG

#../gui/build.sh

#cargo run --quiet\
#  -- \
#  --auth0-domain "$EXAMPLE_AUTH0_DOMAIN" \
#  --auth0-client-id "$EXAMPLE_CLIENT_ID" \
#  --auth0-client-secret "$EXAMPLE_CLIENT_SECRET" \
#  --auth0-redirect-uri "$EXAMPLE_REDIRECT_URI" \
#  --auth0-response-type "$EXAMPLE_RESPONSE_TYPE" \
#  --auth0-scope "$EXAMPLE_SCOPE" \
#  --auth0_audience "$EXAMPLE_AUDIENCE"

cargo run \
  -- \
  --auth0-domain "$EXAMPLE_AUTH0_DOMAIN" \
  --auth0-client-id "$EXAMPLE_CLIENT_ID" \
  --auth0-client-secret "$EXAMPLE_CLIENT_SECRET" \
  --auth0-redirect-uri "$EXAMPLE_REDIRECT_URI" \
  --auth0-response-type "$EXAMPLE_RESPONSE_TYPE" \
  --auth0-scope "$EXAMPLE_SCOPE" \
  --auth0-audience "$EXAMPLE_AUDIENCE"
