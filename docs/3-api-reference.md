# API Reference

Base URL: `https://pylonid.eu`

---

## Health Check

**Status:** ✅ Live

### GET /health

Check if API is running.

**Request:**
curl https://pylonid.eu/health

**Response (200 OK):**
```
{
  "status": "ok",
  "service": "pylon-server",
  "version": "1.0.0",
  "environment": "production"
}
```

---

## Verify Age

**Status:** ✅ Beta (signature validation coming Nov 2025)

### POST /v1/verify/age

Create an age verification request.

**Request:**
```
curl -X POST https://pylonid.eu/v1/verify/age \
-H "Content-Type: application/json" \
-d '{
"policy": {
"minAge": 18
},
"callbackUrl": "https://app.example.com/webhooks/pylon"
}'
```

**Request Body:**
```
{
  "policy": {
    "minAge": 18 // or 21, 25, etc.
  },
  "callbackUrl": "https://app.example.com/webhooks/pylon"
}
```

**Response (200 OK):**
```
{
  "verificationId": "ver_abc123xyz",
  "walletUrl": "https://wallet.pylonid.eu/request/ver_abc123xyz",
  "expiresAt": "2025-11-07T08:15:00Z"
}
```

**Next Steps:**
1. Redirect user to `walletUrl`
2. User scans with EUDI wallet and presents credential
3. PYLON validates and POSTs to `callbackUrl`

---

## Get Verification Status

**Status:** ✅ Live

### GET /v1/status/{id}

Check the status of a verification request.

**Request:**
```
curl https://pylonid.eu/v1/status/ver_abc123xyz
```

**Response (200 OK):**
```
{
  "verificationId": "ver_abc123xyz",
  "status": "pending", // or "completed"
  "result": null, // or "verified", "not_verified"
  "createdAt": "2025-11-06T08:00:00Z",
  "expiresAt": "2025-11-07T08:00:00Z"
}
```

---

## Webhook Signature Verification

**Status:** ✅ Live

When a verification completes, PYLON POSTs to your `callbackUrl` with:

**Headers:**
- `X-Pylon-Signature`: sha256=abc123...
- `Idempotency-Key`: webhook_attempt_456
- `Content-Type`: application/json

**Body:**
```
{
  "verificationId": "ver_abc123xyz",
  "status": "completed",
  "result": "verified", // or "not_verified"
  "completedAt": "2025-11-06T08:05:00Z"
}
```

**Verify signature in your webhook handler:**

```
import hmac
import hashlib

def verify_signature(signature_header, body, webhook_secret):
    expected = hmac.new(
        webhook_secret.encode(),
        body,
        hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature_header.replace("sha256=", ""), expected)
```

**Idempotency:**

- The `Idempotency-Key` header is included in webhook requests.
- Your webhook handler should use this key to deduplicate repeated webhook deliveries to avoid processing the same event multiple times.

---

## Error Responses

**401 Unauthorized**
```
{
  "error": "invalid_api_key",
  "message": "API key not found or expired"
}
```

**422 Unprocessable Entity**
```
{
  "error": "invalid_request",
  "message": "callbackUrl must be HTTPS"
}
```

**429 Too Many Requests**
```
{
  "error": "rate_limited",
  "message": "Exceeded 100 requests/second"
}
```

**500 Internal Server Error**
```
{
  "error": "internal_error",
  "message": "Unexpected server error"
}
```

---

## Rate Limits (Beta)

- **Free tier:** 1,000 verifications/month
- **Request rate:** 100 requests/second
- **Webhook timeout:** 30 seconds
- **Webhook retries:** 5 attempts over 60 seconds (exponential backoff)

---

## Roadmap

**✅ v1.0 (Now)**
- Age verification (mock signatures)
- Webhook delivery + retries
- PostgreSQL persistence

**🟡 v1.1 (Nov 2025)**
- Real signature validation (German EUDI Wallet Sandbox)
- API key authentication

**🟡 v2.0 (Q1 2026)**
- KYC attribute verification
- OIDC Login
- Self-serve dashboard
- SLA guarantee (99.95% uptime)

---

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email [support@pylonid.eu](mailto:support@pylonid.eu)
