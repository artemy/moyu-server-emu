#!/usr/bin/env sh

mkdir -p certs

# Certificate Authority
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out certs/ca_private_key.pem
openssl req -new -x509 -days 3650 -key certs/ca_private_key.pem -subj "/C=NL/O=TEST" -out certs/ca_public_certificate.pem

# Server certificate
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out certs/server_private_key.pem
# Server certificate signing request
openssl req -new -key certs/server_private_key.pem -subj "/C=NL/O=TEST/CN=iot-auth.cn-shanghai.aliyuncs.com" -out certs/server_request.csr
# Sign server certificate
openssl x509 -req -in certs/server_request.csr -CA certs/ca_public_certificate.pem -CAkey certs/ca_private_key.pem -days 3650 -CAcreateserial -out certs/server_public_certificate.pem
