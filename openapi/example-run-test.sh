#!/bin/bash

TOKEN="$1"

echo "test api hello"
./curl-client/my-api.sh --host "http://127.0.0.1:3000" hello
echo
echo "test api testauth"
./curl-client/my-api.sh \
  --host "http://127.0.0.1:3000" \
  testauth \ -- -H "Authorization: Bearer $TOKEN"
echo "test api testauth proxy"
./curl-client/my-api.sh \
  --host https://localhost:8080/api \
  testauth \
  -- -H "Authorization: Bearer $TOKEN" -k
