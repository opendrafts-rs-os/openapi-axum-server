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
sed -i '/^\[dependencies\]/a tower-http = { version = "0.6.2", features = ["fs"] }' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a tracing-subscriber = { version = "0.3", default-features = false, features = ["fmt", "env-filter"] }' ../api/Cargo.toml
sed -i '/^\[dependencies\]/a josekit = "0.10"' ../api/Cargo.toml

cd ../api
grep -qxF ".env" .gitignore || echo ".env" >> .gitignore

