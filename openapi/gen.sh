#!/bin/bash

java -jar openapi-generator-cli.jar generate \
  -i ./example-openapi.yaml \
  -g rust-axum \
  -o ../api

# added dependency for ../api/src/bin/main.rs
sed -i '/^\[dependencies\]/a clap = { version = "4.0", features = ["derive"] }' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a urlencoding = "2.1.3"' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a rand = "0.8.5"' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a reqwest = { version = "0.12.15", features = ["json"] }' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a jsonwebtoken = "9"' ../api/Cargo.toml