# Quickstart: Verify Age in 10 Minutes

Verify customer ages using EUDI wallets. One API call, one QR code, one webhook.

## Prerequisites

- A running PylonID instance (self-hosted or `pylonid.eu`)
- An API key (see Step 1)
- A webhook endpoint that accepts HTTPS POST requests

---

## Step 1: Get an API Key

```bash
curl -X POST https://pylonid.eu/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com"}'
```

Save the returned API key. You'll need it for all authenticated requests.

---

## Step 2: Start an Age Verification

```bash
curl -X POST https://pylonid.eu/v1/verify/age \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "policy": { "minAge": 18 },
    "callbackUrl": "https://yourapp.com/webhooks/pylon"
  }'
```

**Response:**

```json
{
  "verificationId": "ver_81CA6EC3CA80",
  "status": "pending",
  "walletUrl": "eudi-openid4vp://authorize?request_uri=https%3A%2F%2Fpylonid.eu%2Fv1%2Foid4vp%2Frequest%2Fver_81CA6EC3CA80",
  "requestUri": "https://pylonid.eu/v1/oid4vp/request/ver_81CA6EC3CA80",
  "expiresAt": "2026-04-22T00:15:00Z"
}
```

---

## Step 3: Show the QR Code

Display `walletUrl` as a QR code. On mobile, it opens the EUDI wallet directly via the `eudi-openid4vp://` deep link.

```
Your app                          PylonID                         EUDI Wallet
   │                                 │                                 │
   │  Show QR (walletUrl)            │                                 │
   │─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─>│
   │                                 │   GET /v1/oid4vp/request/:id    │
   │                                 │<────────────────────────────────│
   │                                 │   Signed request JWT            │
   │                                 │────────────────────────────────>│
   │                                 │                                 │
   │                                 │   User consents                 │
   │                                 │                                 │
   │                                 │  POST /v1/oid4vp/response       │
   │                                 │<────────────────────────────────│
   │                                 │  (SD-JWT-VC with age_over_18)   │
   │                                 │                                 │
   │  Webhook: verified/rejected     │                                 │
   │<────────────────────────────────│                                 │
```

---

## Step 4: Receive the Webhook

When the user completes verification, PylonID POSTs to your `callbackUrl`:

```json
{
  "event": "verification.completed",
  "verificationId": "ver_81CA6EC3CA80",
  "status": "verified",
  "result": { "age_over_18": true },
  "timestamp": "2026-04-22T00:05:00Z"
}
```

---

## Step 5: Verify the Webhook Signature

PylonID signs every webhook with HMAC-SHA256. **Always validate before trusting the payload.**

The signature is in the `X-Pylon-Signature` header:

```
X-Pylon-Signature: sha256=a1b2c3d4e5f6...
```

### Node.js

```javascript
const crypto = require('crypto');

app.post('/webhooks/pylon', express.raw({ type: 'application/json' }), (req, res) => {
  const signature = req.headers['x-pylon-signature'];
  const body = req.body; // raw Buffer

  const computed = 'sha256=' + crypto
    .createHmac('sha256', process.env.PYLON_WEBHOOK_SECRET)
    .update(body)
    .digest('hex');

  if (!crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(computed))) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const payload = JSON.parse(body);
  console.log(`Verified: ${payload.verificationId} → ${payload.status}`);
  res.status(200).json({ received: true });
});
```

### Python

```python
import hmac, hashlib, os
from flask import Flask, request

@app.route('/webhooks/pylon', methods=['POST'])
def webhook():
    signature = request.headers.get('X-Pylon-Signature', '')
    body = request.get_data()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()

    if not hmac.compare_digest(signature, computed):
        return {'error': 'Invalid signature'}, 401

    data = request.json
    print(f"Verified: {data['verificationId']} → {data['status']}")
    return {'received': True}, 200
```

### Go

```go
func webhookHandler(w http.ResponseWriter, r *http.Request) {
    signature := r.Header.Get("X-Pylon-Signature")
    body, _ := io.ReadAll(r.Body)

    h := hmac.New(sha256.New, []byte(os.Getenv("PYLON_WEBHOOK_SECRET")))
    h.Write(body)
    computed := "sha256=" + hex.EncodeToString(h.Sum(nil))

    if !hmac.Equal([]byte(signature), []byte(computed)) {
        http.Error(w, "Invalid signature", http.StatusUnauthorized)
        return
    }

    w.WriteHeader(http.StatusOK)
    w.Write([]byte(`{"received":true}`))
}
```

---

## Step 6: Poll Status (Optional)

Instead of (or in addition to) webhooks, you can poll:

```bash
curl https://pylonid.eu/v1/status/ver_81CA6EC3CA80 \
  -H "Authorization: Bearer YOUR_API_KEY"
```

---

## Error Cases

| Status | Meaning | Fix |
|--------|---------|-----|
| 401 | Invalid or missing API key | Check `Authorization: Bearer` header |
| 400 | Invalid request body | Check JSON syntax, `minAge` range, `callbackUrl` is HTTPS |
| 429 | Rate limited | Back off and retry |
| 500 | Server error | Retry; contact hello@pylonid.eu if persistent |

---

## Test Locally

Use the Rust-based emulator for offline development:

```bash
cd pylon_cli
cargo run --release
```

Then point your requests at `http://localhost:7777` instead of `https://pylonid.eu`.

---

## Next Steps

- [Core Concepts](./2-core-concepts.md) — understand OID4VP and SD-JWT-VC
- [API Reference](./3-api-reference.md) — all endpoints
- [Webhooks](./6-webhooks.md) — production reliability
- [Local Emulator](./5-local-testing.md) — offline development
