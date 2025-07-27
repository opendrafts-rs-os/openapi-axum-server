openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout ./key.pem -out ./cert.pem \
  -subj "/C=PL/ST=State/L=City/O=Org/OU=Unit/CN=localhost"
