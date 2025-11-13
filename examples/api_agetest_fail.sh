#!/bin/bash
set -e

BASE_URL="https://pylonid.eu"

echo "1. Health check"
curl -s ${BASE_URL}/health | jq

echo "2. Signup and get API key"
API_KEY=$(curl -s -X POST ${BASE_URL}/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com"}' | jq -r .api_key)
echo "API_KEY=$API_KEY"

echo "3. Submit age verification request"
VERIF_ID=$(curl -s -X POST ${BASE_URL}/v1/verify/age \
  -H "Authorization: Bearer ${API_KEY}" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://yourcallback.com/webhook"}' | jq -r .verificationId)
echo "Verification ID: $VERIF_ID"

echo "4. Get test JWT"
JWT=$(curl -s -X GET ${BASE_URL}/v1/test/jwt?age_over_18=false \
  -H "Authorization: Bearer ${API_KEY}" | jq -r .jwt)
echo "JWT token received"

echo "5. Submit JWT presentation verification"
curl -s -X POST ${BASE_URL}/v1/presentations/verify \
  -H "Authorization: Bearer ${API_KEY}" \
  -H "Content-Type: application/json" \
  -d "{\"verificationId\":\"${VERIF_ID}\",\"presentation\":\"${JWT}\"}"

echo -e "\n6. Check verification status"
curl -s -X GET ${BASE_URL}/v1/status/${VERIF_ID} \
  -H "Authorization: Bearer ${API_KEY}" | jq

