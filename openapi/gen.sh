#!/bin/bash

java -jar openapi-generator-cli.jar generate \
  -i ./example-openapi.yaml \
  -g rust-axum \
  -o ../api \
  #-t ./my-templates/rust-axum \
  #--skip-validate-spec

# added dependency for ../api/src/bin/main.rs
sed -i '/^\[dependencies\]/a clap = { version = "4.0", features = ["derive"] }' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a urlencoding = "2.1.3"' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a rand = "0.8.5"' ../api/Cargo.toml