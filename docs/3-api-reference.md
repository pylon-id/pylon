# API Reference

Base URL: `https://pylonid.eu`

All authenticated endpoints require:
```
Authorization: Bearer YOUR_API_KEY
```

---

## Authentication

### POST /v1/auth/signup

Create a new API key.

**Request:**
```bash
curl -X POST https://pylonid.eu/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com"}'
```

**Response (200):**
```json
{
  "apiKey": "pyl_...",
  "email": "you@example.com"
}
```

Store this key securely — it cannot be retrieved again.

---

### POST /v1/auth/rotate

Rotate your API key. Requires current key for authentication.

**Request:**
```bash
curl -X POST https://pylonid.eu/v1/auth/rotate \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{}'
```

**Response (200):**
```json
{
  "apiKey": "pyl_...",
  "previous": "revoked"
}
```

The old key is immediately invalidated.

---

## Age Verification

### POST /v1/verify/age

Start an age verification request. Returns a wallet URL for the user to scan.

**Request:**
```bash
curl -X POST https://pylonid.eu/v1/verify/age \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "policy": { "minAge": 18 },
    "callbackUrl": "https://yourapp.com/webhooks/pylon"
  }'
```

**Request Body:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `policy.minAge` | integer | Yes | Minimum age (1–150) |
| `callbackUrl` | string | Yes | HTTPS URL for webhook delivery |

**Response (200):**
```json
{
  "verificationId": "ver_81CA6EC3CA80",
  "status": "pending",
  "walletUrl": "eudi-openid4vp://authorize?request_uri=https%3A%2F%2Fpylonid.eu%2Fv1%2Foid4vp%2Frequest%2Fver_81CA6EC3CA80",
  "requestUri": "https://pylonid.eu/v1/oid4vp/request/ver_81CA6EC3CA80",
  "expiresAt": "2026-04-22T00:15:00Z"
}
```

| Field | Description |
|-------|-------------|
| `verificationId` | Unique ID for this verification |
| `walletUrl` | Deep link for EUDI wallet — display as QR code |
| `requestUri` | HTTPS URL the wallet fetches the signed request from |
| `expiresAt` | Verification expires after this time |

---

### GET /v1/status/:id

Check the status of a verification.

**Request:**
```bash
curl https://pylonid.eu/v1/status/ver_81CA6EC3CA80 \
  -H "Authorization: Bearer YOUR_API_KEY"
```

**Response (200):**
```json
{
  "verificationId": "ver_81CA6EC3CA80",
  "status": "pending",
  "result": null,
  "createdAt": "2026-04-22T00:00:00Z",
  "expiresAt": "2026-04-22T00:15:00Z"
}
```

**Status values:** `pending`, `verified`, `rejected`, `expired`

---

## Wallet Endpoints

These are called by the EUDI wallet, not by your application. No authentication required.

### GET /v1/oid4vp/request/:id

The wallet fetches the signed authorization request JWT from this endpoint.

**Response:** Signed ES256 JWT containing:
- `presentation_definition` requesting `age_over_18` from `urn:eudi:pid:1`
- `response_uri` — where the wallet sends the VP token
- `nonce` and `state` for replay protection
- `client_id` — PylonID verifier identifier

---

### POST /v1/oid4vp/response

The wallet submits the Verifiable Presentation token here.

**Request body:** `application/x-www-form-urlencoded`

| Field | Description |
|-------|-------------|
| `vp_token` | SD-JWT-VC with selective disclosures and key binding JWT |
| `state` | Matches the state from the authorization request |
| `presentation_submission` | JSON describing which credential satisfies which input descriptor |

**Processing:**
1. Parse SD-JWT-VC (issuer JWT + disclosures + key binding JWT)
2. Fetch issuer JWKS and verify ES256 signature
3. Verify key binding JWT (nonce, audience, freshness)
4. Reconstruct claims from selective disclosures
5. Extract `age_over_18`
6. Update verification status
7. Fire webhook to SMB callback URL

---

## Discovery

### GET /.well-known/openid-credential-verifier

Verifier metadata and public key for wallet trust establishment.

**Response (200):**
```json
{
  "issuer": "https://pylonid.eu",
  "jwks": {
    "keys": [
      {
        "kty": "EC",
        "crv": "P-256",
        "kid": "pylonid-verifier-1",
        "use": "sig",
        "x": "...",
        "y": "..."
      }
    ]
  }
}
```

---

### GET /health

Service health check.

**Response (200):**
```json
{
  "status": "ok",
  "service": "pylon-server",
  "version": "1.0.0"
}
```

---

## Webhook Delivery

When a verification completes, PylonID POSTs to your `callbackUrl`.

**Headers:**
```
Content-Type: application/json
X-Pylon-Signature: sha256=a1b2c3d4...
```

**Body:**
```json
{
  "event": "verification.completed",
  "verificationId": "ver_81CA6EC3CA80",
  "status": "verified",
  "result": { "age_over_18": true },
  "timestamp": "2026-04-22T00:05:00Z"
}
```

**Signature verification:**
```python
computed = "sha256=" + HMAC-SHA256(webhook_secret, raw_body).hex()
assert computed == headers["X-Pylon-Signature"]
```

See [Webhooks](./6-webhooks.md) for retry policy, idempotency, and implementation examples.

---

## Error Responses

**401 Unauthorized:**
```json
{ "error": "invalid_api_key", "message": "API key not found or expired" }
```

**400 Bad Request:**
```json
{ "error": "invalid_request", "message": "callbackUrl must be HTTPS" }
```

**429 Too Many Requests:**
```json
{ "error": "rate_limited", "message": "Too many requests" }
```

**500 Internal Server Error:**
```json
{ "error": "internal_error", "message": "Unexpected server error" }
```

---

## Rate Limits

- Request rate limiting is enforced per API key
- Webhook timeout: 30 seconds
- Webhook retries: exponential backoff

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
