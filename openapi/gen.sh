#!/bin/bash

java -jar openapi-generator-cli.jar generate \
  -i ./example-openapi.yaml \
  -g rust-axum \
  -o ../api