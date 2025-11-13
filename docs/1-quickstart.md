# Quickstart: Verify Age in 10 Minutes

Verify attributes from EUDI wallets in minutes. This guide walks you through your first age verification request.

## Prerequisites

- Create a free account at `https://dashboard.pylonid.eu`
- Generate an API key in the dashboard
- Add a webhook endpoint (e.g., `https://app.example.com/webhooks/pylon`)
- (Optional) Install local emulator: `npm install -g pylon-cli && pylon dev`

## Base URLs

| Environment | URL |
|---|---|
| **Sandbox** | `https://sandbox.api.pylonid.eu` |
| **Production** | `https://api.pylonid.eu` |

All requests require Bearer token authentication:

```
Authorization: Bearer <YOUR_PYLON_API_KEY>
```

---

## Step 1: Create a Verification Request

Call `POST /v1/verify/age` with your policy and callback URL:

```
curl -X POST https://sandbox.api.pylonid.eu/v1/verify/age \\
  -H "Authorization: Bearer $PYLON_API_KEY" \\
  -H "Content-Type: application/json" \\
  -d '{
    "policy": {
      "minAge": 18,
      "evidence": ["national_eid", "mdoc_id"]
    },
    "callbackUrl": "https://app.example.com/webhooks/pylon"
  }'
```

---

## Step 2: Parse the Response

PYLON returns a verification ID and wallet URL:

```
{
  "verificationId": "ver_123abc789",
  "status": "pending",
  "walletUrl": "https://pylon.link/123abc789"
}
```

---

## Step 3: Redirect the User

Display the `walletUrl` as a QR code or direct link. On mobile, the EUDI wallet app opens automatically.

**Your app should:**
1. Display QR code (or link) to `walletUrl`
2. User scans with EUDI wallet app
3. Wallet shows: "Verify age >= 18?" with Accept/Deny buttons
4. User taps Accept/Deny
5. Wallet sends cryptographic proof back to PYLON

---

## Step 4: Receive the Webhook

Once the user completes the action, PYLON POSTs the result to your `callbackUrl`:

```
{
  "verificationId": "ver_123abc789",
  "type": "age",
  "result": "verified",
  "attributes": {
    "ageOver18": true
  },
  "evidence": {
    "issuer": "AT_GOV",
    "credentialType": "SD-JWT VC",
    "proofHash": "abc123def456...",
    "issuedAt": "2025-01-12T10:23:11Z"
  },
  "audit": {
    "traceId": "trace_xyz987"
  }
}
```

**Critical:** Always verify the webhook signature (see [Webhooks](./6-webhooks.md)).

---

## Verify the Webhook Signature

PYLON signs every webhook with HMAC-SHA256. Verify the `X-PYLON-Signature` header:

### Node.js

```
const crypto = require('crypto');

function verifyWebhookSignature(payload, signature, secret) {
  const computed = crypto
    .createHmac('sha256', secret)
    .update(payload)
    .digest('hex');
  
  return crypto.timingSafeEqual(
    Buffer.from(signature),
    Buffer.from(computed)
  );
}

app.post('/webhooks/pylon', (req, res) => {
  const signature = req.headers['x-pylon-signature'].split('v1=');[1]
  const secret = process.env.PYLON_WEBHOOK_SECRET;
  
  if (!verifyWebhookSignature(JSON.stringify(req.body), signature, secret)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }
  
  console.log(`Verified: ${req.body.verificationId}`);
  res.status(200).json({ received: true });
});
```

### Python

```
import hmac
import hashlib
from flask import Flask, request, jsonify

def verify_webhook_signature(payload, signature, secret):
  computed = hmac.new(
    secret.encode(),
    payload.encode() if isinstance(payload, str) else payload,
    hashlib.sha256
  ).hexdigest()
  
  return hmac.compare_digest(signature, computed)

@app.route('/webhooks/pylon', methods=['POST'])
def webhook():
  signature = request.headers.get('X-Pylon-Signature', '').split('v1=')[1]
  secret = os.getenv('PYLON_WEBHOOK_SECRET')
  
  if not verify_webhook_signature(request.get_data(), signature, secret):
    return {'error': 'Invalid signature'}, 401
  
  data = request.json
  print(f"Verified: {data['verificationId']}")
  return {'received': True}, 200
```

### Go

```
package main

import (
  "crypto/hmac"
  "crypto/sha256"
  "encoding/hex"
  "net/http"
)

func verifyWebhookSignature(payload []byte, signature, secret string) bool {
  h := hmac.New(sha256.New, []byte(secret))
  h.Write(payload)
  computed := hex.EncodeToString(h.Sum(nil))
  return hmac.Equal([]byte(signature), []byte(computed))
}

func webhookHandler(w http.ResponseWriter, r *http.Request) {
  signature := r.Header.Get("X-Pylon-Signature")[3:] // Remove "v1="
  body, _ := io.ReadAll(r.Body)
  
  if !verifyWebhookSignature(body, signature, os.Getenv("PYLON_WEBHOOK_SECRET")) {
    http.Error(w, "Invalid signature", http.StatusUnauthorized)
    return
  }
  
  w.Header().Set("Content-Type", "application/json")
  w.WriteHeader(http.StatusOK)
  w.Write([]byte(`{"received":true}`))
}
```

---

## Test Locally Without a Real Wallet

Use the local emulator to test without internet or a real EUDI wallet:

```
npm install -g pylon-cli
pylon dev --wallet=mock --age=20
```

In another terminal:

```
curl -X POST http://localhost:8000/v1/verify/age \\
  -H "Authorization: Bearer test_key_local" \\
  -H "Content-Type: application/json" \\
  -d '{
    "policy": {"minAge": 18},
    "callbackUrl": "http://localhost:3000/webhook"
  }'
```

The emulator auto-completes the flow and triggers your webhook instantly.

---

## Error Cases

| Error | Meaning | Fix |
|-------|---------|-----|
| 401 Unauthorized | Invalid API key | Check `$PYLON_API_KEY` export |
| 400 Bad Request | Invalid policy | Check JSON syntax and `minAge` (0-150) |
| 429 Too Many Requests | Rate limited | Wait 60s and retry; check dashboard for usage |

---

## Next Steps

- ✅ Successfully verified age in 10 minutes
- 📖 Read [Core Concepts](./2-core-concepts.md) to understand OID4VP + SD-JWT
- 📚 Read [API Reference](./3-api-reference.md) for all endpoints
- 🔒 Read [Webhooks](./6-webhooks.md) for production reliability
- 🧪 Try [Local Emulator](./5-local-testing.md) for offline testing

---

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email [support@pylonid.eu](mailto:support@pylonid.eu)
