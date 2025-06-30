#!/bin/bash
set -o allexport
source .env
set +o allexport

# trunk serve --proxy-backend=http://localhost:3000/ --proxy-rewrite=/api/


trunk serve \
    --address 127.0.0.1 \
    --port 8080 \
    --tls-cert-path ./certs/cert.pem \
    --tls-key-path ./certs/key.pem

# trunk serve --host localhost --tls-key-path ./certs/key.pem --tls-cert-path ./certs/cert.pem
