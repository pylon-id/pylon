#!/bin/bash
# PylonID age verification — end-to-end API test
set -e

BASE_URL="${PYLON_URL:-https://pylonid.eu}"

echo "1. Health check"
curl -s ${BASE_URL}/health | jq

echo "2. Signup and get API key"
API_KEY=$(curl -s -X POST ${BASE_URL}/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}' | jq -r .api_key)
echo "API_KEY=${API_KEY}"

echo "3. Start age verification"
RESULT=$(curl -s -X POST ${BASE_URL}/v1/verify/age \
  -H "Authorization: Bearer ${API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://example.com/webhook"}')
echo "$RESULT" | jq

VERIF_ID=$(echo "$RESULT" | jq -r .verificationId)
echo "Verification ID: ${VERIF_ID}"
echo "Wallet URL: $(echo "$RESULT" | jq -r .walletUrl)"

echo "4. Check status (should be pending)"
curl -s ${BASE_URL}/v1/status/${VERIF_ID} \
  -H "Authorization: Bearer ${API_KEY}" | jq

echo ""
echo "Scan the walletUrl with an EUDI wallet to complete verification."
echo "Then check status again or wait for webhook."
