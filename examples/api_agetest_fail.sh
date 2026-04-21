#!/bin/bash
# PylonID API — error case tests
set -e

BASE_URL="${PYLON_URL:-https://pylonid.eu}"

echo "1. Missing auth header (expect 401)"
curl -s -w "\nHTTP %{http_code}\n" -X POST ${BASE_URL}/v1/verify/age \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://example.com/webhook"}'

echo ""
echo "2. Invalid API key (expect 401)"
curl -s -w "\nHTTP %{http_code}\n" -X POST ${BASE_URL}/v1/verify/age \
  -H "Authorization: Bearer invalid_key_here" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://example.com/webhook"}'

echo ""
echo "3. HTTP callback URL (expect 400)"
API_KEY=$(curl -s -X POST ${BASE_URL}/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}' | jq -r .api_key)

curl -s -w "\nHTTP %{http_code}\n" -X POST ${BASE_URL}/v1/verify/age \
  -H "Authorization: Bearer ${API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"http://example.com/webhook"}'

echo ""
echo "4. Nonexistent verification (expect 404)"
curl -s -w "\nHTTP %{http_code}\n" ${BASE_URL}/v1/status/ver_doesnotexist \
  -H "Authorization: Bearer ${API_KEY}"
