# Webhooks: Production Guide

PYLON delivers asynchronous verification results via webhooks. This guide covers production reliability, security, and best practices.

***

## Overview

After a user completes a verification in their EUDI wallet, PYLON sends a **signed HTTP POST** to your webhook endpoint with the result.

### Webhook Lifecycle

```
1. User completes verification (age, KYC, signature)
2. PYLON validates cryptographic proof
3. PYLON POSTs result to your callbackUrl
4. Your app receives and processes webhook
5. Your app returns HTTP 200
6. Webhook marked as delivered
```

***

## Setup

### 1. Register Webhook Endpoint

In PYLON Dashboard:
1. Go **Settings > Webhooks**
2. Add endpoint: `https://app.example.com/api/webhooks/pylon`
3. Copy **Webhook Secret** (save securely)
4. Click **"Send Test"** to verify connectivity

### Requirements

- ✅ **HTTPS only** (HTTP rejected)
- ✅ **Publicly accessible** (curl must work from internet)
- ✅ **Returns HTTP 200** within 10 seconds
- ✅ **Valid SSL certificate** (Let's Encrypt OK)

### 2. Store Webhook Secret

```bash
# ✅ Good: Environment variable
export PYLON_WEBHOOK_SECRET="whsec_abc123xyz..."

# ❌ Bad: Hardcoded in code
const secret = "whsec_abc123xyz...";  # Don't do this!
```

***

## Webhook Request Format

PYLON sends:

```
POST https://app.example.com/api/webhooks/pylon
Content-Type: application/json
X-PYLON-Signature: t=1678886400,v1=abcdef1234567890...
X-Pylon-Idempotency-Key: idem_123xyz789...

{
  "verificationId": "ver_abc123xyz",
  "type": "age",
  "result": "verified",
  "attributes": { ... },
  "evidence": { ... },
  "audit": { ... }
}
```

### Headers

| Header | Purpose | Example |
|--------|---------|---------|
| `X-PYLON-Signature` | HMAC-SHA256 signature for verification | `t=1678886400,v1=abcd...` |
| `X-Pylon-Idempotency-Key` | Unique key for retry deduplication | `idem_123xyz789...` |

***

## Webhook Responses

### Age Verification Result

```json
{
  "verificationId": "ver_abc123xyz",
  "type": "age",
  "result": "verified",
  "attributes": {
    "ageOver18": true
  },
  "evidence": {
    "issuer": "AT_GOV",
    "issuerName": "Republik Österreich",
    "credentialType": "SD-JWT VC",
    "proofHash": "sha256:abc123...",
    "issuedAt": "2025-01-12T10:23:11Z",
    "expiresAt": "2026-01-12T10:23:11Z"
  },
  "audit": {
    "traceId": "trace_xyz987",
    "timestamp": "2025-01-15T14:30:00Z"
  }
}
```

### KYC Verification Result

```json
{
  "verificationId": "ver_def456uvw",
  "type": "kyc",
  "result": "verified",
  "attributes": {
    "given_name": "Anna",
    "family_name": "Müller",
    "date_of_birth": "1990-05-15",
    "address": {
      "street_address": "Schulstr. 12",
      "postal_code": "10115",
      "locality": "Berlin",
      "country": "DE"
    }
  },
  "evidence": {
    "issuer": "DE_GOV",
    "credentialType": "SD-JWT VC",
    "issuedAt": "2025-01-10T09:15:22Z"
  },
  "audit": {
    "traceId": "trace_abc123",
    "timestamp": "2025-01-15T14:30:00Z"
  }
}
```

### Failed Verification

```json
{
  "verificationId": "ver_ghi789jkl",
  "type": "age",
  "result": "not_verified",
  "reason": "user_denied",
  "audit": {
    "traceId": "trace_failed123",
    "timestamp": "2025-01-15T14:31:00Z"
  }
}
```

**Possible reasons:**
- `user_denied` — User tapped "Reject" in wallet
- `user_cancelled` — User closed wallet without responding
- `credential_invalid` — Wallet sent invalid/expired credential
- `policy_mismatch` — Credential doesn't meet policy (e.g., user is 17, policy requires 18)
- `timeout` — User didn't respond within 15 minutes
- `error` — Technical error (check traceId in logs)

***

## Signature Validation (CRITICAL)

**ALWAYS validate webhook signatures.** Without validation, anyone can fake a webhook.

### Signature Format

```
X-PYLON-Signature: t=1678886400,v1=abcdef1234567890abcdef1234567890
```

- `t` = Unix timestamp when PYLON sent the webhook
- `v1` = HMAC-SHA256(secret, "t.body") in hex

### Validation Algorithm

```
1. Extract t and v1 from header
2. Get raw request body (bytes, before JSON parsing)
3. Construct signed message: "t.{rawBody}"
4. Compute HMAC-SHA256(secret, signed_message)
5. Compare using timing-safe comparison
```

### Node.js Example

```javascript
import crypto from 'crypto';

function validatePylonWebhook(signature, body, secret) {
  const [t, v1] = signature.split(',');
  const tValue = t.replace('t=', '');
  
  const signedMessage = `${tValue}.${body}`;
  const computed = crypto
    .createHmac('sha256', secret)
    .update(signedMessage)
    .digest('hex');

  return crypto.timingSafeEqual(
    Buffer.from(v1.replace('v1=', '')),
    Buffer.from(computed)
  );
}

// Express middleware
app.post('/webhooks/pylon', express.raw({ type: 'application/json' }), (req, res) => {
  const signature = req.headers['x-pylon-signature'];
  
  if (!validatePylonWebhook(signature, req.body.toString(), process.env.PYLON_WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const payload = JSON.parse(req.body);
  handleWebhook(payload);

  res.status(200).json({ received: true });
});
```

### Python Example

```python
import hmac
import hashlib

def validate_pylon_webhook(signature, body, secret):
  t, v1 = signature.split(',')
  t = t.replace('t=', '')
  v1 = v1.replace('v1=', '')
  
  signed_message = f"{t}.{body}"
  computed = hmac.new(
    secret.encode(),
    signed_message.encode(),
    hashlib.sha256
  ).hexdigest()

  return hmac.compare_digest(v1, computed)

# Flask endpoint
@app.route('/webhooks/pylon', methods=['POST'])
def pylon_webhook():
  signature = request.headers.get('X-PYLON-Signature')
  body = request.get_data()
  secret = os.getenv('PYLON_WEBHOOK_SECRET')

  if not validate_pylon_webhook(signature, body.decode(), secret):
    return {'error': 'Invalid signature'}, 401

  payload = request.json
  handle_webhook(payload)
  
  return {'received': True}, 200
```

### Go Example

```go
import (
  "crypto/hmac"
  "crypto/sha256"
  "encoding/hex"
  "strings"
)

func validatePylonWebhook(signature string, body []byte, secret string) bool {
  parts := strings.Split(signature, ",")
  if len(parts) != 2 {
    return false
  }

  t := strings.TrimPrefix(parts[0], "t=")
  v1 := strings.TrimPrefix(parts[1], "v1=")

  signedMessage := t + "." + string(body)
  
  h := hmac.New(sha256.New, []byte(secret))
  h.Write([]byte(signedMessage))
  computed := hex.EncodeToString(h.Sum(nil))

  return hmac.Equal([]byte(v1), []byte(computed))
}
```

***

## Idempotency & Deduplication

Every webhook retry includes the same `X-Pylon-Idempotency-Key`. Use this to prevent duplicate processing.

**Why this matters:** If your handler crashes after processing but before returning HTTP 200, PYLON retries. Without deduplication, you might grant access multiple times.

### Node.js Example

```javascript
app.post('/webhooks/pylon', async (req, res) => {
  const idempotencyKey = req.headers['x-pylon-idempotency-key'];

  // Check if already processed
  const existing = await db.webhooks.findOne({ idempotencyKey });
  if (existing) {
    console.log(`Already processed: ${idempotencyKey}`);
    return res.status(200).json({ status: 'already_processed' });
  }

  // Validate signature
  if (!validatePylonWebhook(...)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  // Return 200 immediately (webhook delivered)
  res.status(200).json({ received: true });

  // Record processing
  await db.webhooks.insertOne({
    idempotencyKey,
    verificationId: req.body.verificationId,
    result: req.body.result,
    processedAt: new Date(),
  });

  // Process asynchronously
  queue.add('processWebhook', req.body);
});
```

### With TTL (Auto-Cleanup)

```javascript
// Store idempotency keys with 24-hour TTL
await db.webhooks.insertOne(
  {
    idempotencyKey,
    verificationId: req.body.verificationId,
    result: req.body.result,
    processedAt: new Date(),
  },
  {
    // MongoDB: auto-delete after 24 hours
    expireAfterSeconds: 86400,
  }
);
```

***

## Retry Policy

If your webhook doesn't return HTTP 200 within 10 seconds, PYLON retries:

| Attempt | Delay  | Total Time    |
|---------|--------|---------------|
| 1       | 0s     | 0s            |
| 2       | 30s    | 30s           |
| 3       | 5m     | 5m 30s        |
| 4       | 1h     | 1h 5m 30s     |
| 5       | 24h    | 25h 5m 30s    |

After the 5th attempt, webhook is marked as failed.

### Best Practice: Process Async

```javascript
// Return 200 immediately
res.status(200).json({ received: true });

// Process in background
setTimeout(() => {
  // Do expensive work here (db updates, API calls)
  processWebhook(payload);
}, 0);
```

This prevents retries during processing.

***

## Common Issues

### "My webhook never fires"

**Checklist:**
1. Is callback URL HTTPS? (HTTP rejected)
2. Is it publicly accessible? `curl https://app.example.com/webhook`
3. Does it return HTTP 200 within 10 seconds?
4. Check PYLON Dashboard > Webhooks > Delivery Log for error details

### "Signature validation always fails"

**Debugging:**

```bash
# 1. Print raw request body (before JSON.parse)
console.log(typeof req.body);  # Should be Buffer or string

# 2. Print signature header
console.log(req.headers['x-pylon-signature']);

# 3. Print secret
console.log(process.env.PYLON_WEBHOOK_SECRET);

# 4. Manual verify
const signed = `${t}.${body}`;
const manual = crypto.createHmac('sha256', secret).update(signed).digest('hex');
console.log('Expected:', v1);
console.log('Computed:', manual);
console.log('Match:', v1 === manual);
```

**Mistake:** Always use raw body bytes for signature validation; JSON parsing alters bytes.

### "Webhook times out"

- Return HTTP 200 immediately and process asynchronously to prevent retries.

```javascript
res.status(200).json({ received: true });
queue.add('process', payload);  # Background job
```

### "Webhook URL keeps rejecting connections"

- Check SSL certificate validity: `openssl s_client -connect app.example.com:443`
- Check firewall rules and port accessibility
- Verify your service is up: `curl https://app.example.com/webhook`

***

## Monitoring

### What to Track

```javascript
app.post('/webhooks/pylon', (req, res) => {
  const start = Date.now();

  // ... process your webhook ...

  const duration = Date.now() - start;
  logger.info({
    verificationId: req.body.verificationId,
    result: req.body.result,
    duration,
    status: res.statusCode,
  });
});
```

### Metrics

- Delivery rate: % of webhooks returned HTTP 200
- Latency: time to process webhook
- Error rate: % of webhooks that failed or timed out
- Duplicate rate: % of duplicate idempotency keys received

### Alerts

- Alert if delivery rate < 99%
- Alert if latency > 5s
- Alert if error rate > 0.5%

***

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email support@pylonid.eu
]
