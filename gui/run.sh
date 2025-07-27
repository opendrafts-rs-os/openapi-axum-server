#!/bin/bash
set -o allexport
source .env
set +o allexport

trunk serve --tls-cert-path ./certs/cert.pem --tls-key-path ./certs/key.pem --proxy-backend=http://127.0.0.1:3000/ --proxy-rewrite=/api/