#!/bin/bash
set -o allexport
source .env
set +o allexport

cargo run \
  -- \
  --auth0-domain "$EXAMPLE_AUTH0_DOMAIN" \
  --auth0-client-id "$EXAMPLE_CLIENT_ID" \
  --auth0-client-secret "$EXAMPLE_CLIENT_SECRET" \
  --auth0-redirect-uri "$EXAMPLE_REDIRECT_URI" \
  --auth0-response-type "$EXAMPLE_RESPONSE_TYPE" \
  --auth0-scope "$EXAMPLE_SCOPE"