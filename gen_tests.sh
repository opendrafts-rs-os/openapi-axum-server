#!/bin/bash

cd ./openapi

rm -rf ./curl-client

java -jar openapi-generator-cli.jar generate \
  -i ./example-openapi.yaml \
  -g bash \
  -p scriptName=my-api.sh \
  -o ./curl-client \
  -p generateBashScripts=true

  chmod +x ./curl-client/my-api.sh