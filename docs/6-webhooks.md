# Webhooks: Production Guide

PylonID delivers verification results asynchronously via signed webhooks.

---

## Overview

After a user completes verification in their EUDI wallet, PylonID sends a signed HTTP POST to your `callbackUrl`.

```
1. User consents in EUDI wallet
2. Wallet sends VP token to PylonID
3. PylonID validates SD-JWT-VC cryptographic proof
4. PylonID POSTs result to your callbackUrl
5. Your app returns HTTP 200
6. Webhook marked as delivered
```

---

## Webhook Format

**Headers:**
```
Content-Type: application/json
X-Pylon-Signature: sha256=a1b2c3d4e5f6...
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

### Possible Status Values

| Status | Meaning |
|--------|---------|
| `verified` | User proved age ≥ minimum |
| `rejected` | User denied consent or credential didn't meet policy |
| `expired` | User didn't respond before timeout |
| `error` | Technical failure |

---

## Signature Validation (CRITICAL)

**Always validate webhook signatures.** Without validation, anyone can forge a webhook to your endpoint.

PylonID signs the raw request body with HMAC-SHA256:

```
X-Pylon-Signature: sha256={hex(HMAC-SHA256(secret, raw_body))}
```

### Validation Steps

1. Get raw request body (bytes, **before** JSON parsing)
2. Compute `HMAC-SHA256(your_webhook_secret, raw_body)`
3. Format as `sha256={hex}`
4. Compare to `X-Pylon-Signature` header using constant-time comparison

---

### Node.js

```javascript
const crypto = require('crypto');

function validateSignature(body, signature, secret) {
  const computed = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(computed));
}

app.post('/webhooks/pylon', express.raw({ type: 'application/json' }), (req, res) => {
  const signature = req.headers['x-pylon-signature'];

  if (!validateSignature(req.body, signature, process.env.PYLON_WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const payload = JSON.parse(req.body);
  console.log(`${payload.verificationId}: ${payload.status}`);
  res.status(200).json({ received: true });
});
```

---

### Python (Flask)

```python
import hmac, hashlib, os
from flask import Flask, request

app = Flask(__name__)

def validate_signature(body, signature, secret):
    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature, computed)

@app.route('/webhooks/pylon', methods=['POST'])
def pylon_webhook():
    signature = request.headers.get('X-Pylon-Signature', '')
    body = request.get_data()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(body, signature, secret):
        return {'error': 'Invalid signature'}, 401

    data = request.json
    print(f"{data['verificationId']}: {data['status']}")
    return {'received': True}, 200
```

---

### Python (FastAPI)

```python
import hmac, hashlib, os
from fastapi import FastAPI, Request, HTTPException

app = FastAPI()

def validate_signature(body: bytes, signature: str, secret: str) -> bool:
    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature, computed)

@app.post('/webhooks/pylon')
async def pylon_webhook(request: Request):
    signature = request.headers.get('X-Pylon-Signature', '')
    body = await request.body()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(body, signature, secret):
        raise HTTPException(status_code=401, detail='Invalid signature')

    data = await request.json()
    print(f"{data['verificationId']}: {data['status']}")
    return {'received': True}
```

---

### Go

```go
import (
    "crypto/hmac"
    "crypto/sha256"
    "encoding/hex"
    "io"
    "net/http"
    "os"
)

func validateSignature(body []byte, signature, secret string) bool {
    h := hmac.New(sha256.New, []byte(secret))
    h.Write(body)
    computed := "sha256=" + hex.EncodeToString(h.Sum(nil))
    return hmac.Equal([]byte(signature), []byte(computed))
}

func webhookHandler(w http.ResponseWriter, r *http.Request) {
    signature := r.Header.Get("X-Pylon-Signature")
    body, _ := io.ReadAll(r.Body)

    if !validateSignature(body, signature, os.Getenv("PYLON_WEBHOOK_SECRET")) {
        http.Error(w, "Invalid signature", http.StatusUnauthorized)
        return
    }

    w.WriteHeader(http.StatusOK)
    w.Write([]byte(`{"received":true}`))
}
```

---

### Java (Spring Boot)

```java
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.nio.charset.StandardCharsets;

@PostMapping("/webhooks/pylon")
public ResponseEntity<?> handleWebhook(
    @RequestHeader("X-Pylon-Signature") String signature,
    @RequestBody String body
) {
    String secret = System.getenv("PYLON_WEBHOOK_SECRET");

    try {
        Mac mac = Mac.getInstance("HmacSHA256");
        mac.init(new SecretKeySpec(secret.getBytes(StandardCharsets.UTF_8), "HmacSHA256"));
        byte[] hash = mac.doFinal(body.getBytes(StandardCharsets.UTF_8));
        StringBuilder sb = new StringBuilder("sha256=");
        for (byte b : hash) sb.append(String.format("%02x", b));
        String computed = sb.toString();

        if (!computed.equals(signature)) {
            return ResponseEntity.status(401).build();
        }
    } catch (Exception e) {
        return ResponseEntity.status(500).build();
    }

    return ResponseEntity.ok(Map.of("received", true));
}
```

---

### Rust (Axum)

```rust
use axum::{http::{HeaderMap, StatusCode}, Json};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn validate_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let computed = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));
    subtle::ConstantTimeEq::ct_eq(computed.as_bytes(), signature.as_bytes()).into()
}

async fn webhook_handler(headers: HeaderMap, body: String) -> Result<Json<serde_json::Value>, StatusCode> {
    let signature = headers.get("x-pylon-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("PYLON_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !validate_signature(body.as_bytes(), signature, &secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(Json(serde_json::json!({"received": true})))
}
```

---

## Retry Policy

If your endpoint doesn't return HTTP 2xx within 30 seconds, PylonID retries with exponential backoff:

| Attempt | Delay |
|---------|-------|
| 1 | Immediate |
| 2 | 30 seconds |
| 3 | 5 minutes |
| 4 | 1 hour |
| 5 | 24 hours |

After 5 failed attempts, the webhook is marked as failed.

**Best practice:** Return 200 immediately, process asynchronously:

```javascript
app.post('/webhooks/pylon', (req, res) => {
  // Validate signature...
  res.status(200).json({ received: true }); // Return immediately
  queue.add('processVerification', JSON.parse(req.body)); // Background
});
```

---

## Endpoint Requirements

- ✅ **HTTPS only** (HTTP rejected)
- ✅ **Publicly accessible** (not localhost)
- ✅ **Returns HTTP 2xx** within 30 seconds
- ✅ **Valid TLS certificate** (Let's Encrypt works)

### Testing Locally

Use ngrok or similar to expose a local endpoint:

```bash
ngrok http 3000
# Use the HTTPS URL as your callbackUrl
```

---

## Common Issues

### Signature validation always fails

```javascript
// ❌ WRONG: Using parsed JSON body
const body = JSON.stringify(req.body);

// ✅ RIGHT: Using raw body bytes
app.post('/webhook', express.raw({ type: 'application/json' }), (req, res) => {
  const body = req.body; // Buffer, not parsed object
});
```

**Always use the raw request body for signature validation.** JSON parsing and re-serializing can change whitespace, key order, or encoding.

### Webhook never fires

1. Is `callbackUrl` HTTPS? (HTTP is rejected)
2. Is it publicly reachable? Test with `curl` from another machine
3. Does it return HTTP 2xx?
4. Check verification status via `GET /v1/status/:id` — is it still `pending`?

### Webhook times out

Return HTTP 200 immediately and process in the background. Don't do database writes, API calls, or heavy computation before responding.

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
