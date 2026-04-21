# Integration & Testing

How to set up your environment, test your integration, and go to production.

---

## Setup

### 1. Get an API Key

```bash
curl -X POST https://pylonid.eu/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com"}'
```

Store the returned API key securely. It cannot be retrieved again.

### 2. Configure Your Environment

```bash
# Your app needs these two values
export PYLON_API_KEY="pyl_..."           # From signup
export PYLON_WEBHOOK_SECRET="..."         # Provided with your API key
```

Store in your secrets manager (AWS Secrets Manager, Vault, etc.) — never in source code.

### 3. Set Up a Webhook Endpoint

Your app needs an HTTPS endpoint that:
- Accepts POST requests
- Returns HTTP 200 within 30 seconds
- Validates the `X-Pylon-Signature` header

See [Webhooks](./6-webhooks.md) for implementation examples in Node.js, Python, Go, Java, and Rust.

---

## Testing Progression

```
1. Local emulator (no network, no wallet needed)
   ↓
2. Live API with test verifications
   ↓
3. Real EUDI wallet end-to-end test
   ↓
4. Production (real users)
```

### Stage 1: Local Emulator

The Rust-based emulator runs the full API locally with a mock wallet:

```bash
cd pylon_cli
cargo run --release
# Emulator runs on http://localhost:7777
```

Point your app at `http://localhost:7777` instead of `https://pylonid.eu`. The emulator auto-completes verifications and fires webhooks immediately.

See [Local Emulator](./5-local-testing.md) for details.

### Stage 2: Live API

Switch your app to `https://pylonid.eu` with your real API key. Create verifications and confirm:
- You receive the `walletUrl` and `verificationId`
- Your webhook endpoint is reachable
- Signature validation works
- Status polling works via `GET /v1/status/:id`

### Stage 3: Real Wallet Test

Scan the QR code with an actual EUDI wallet app. Verify the full flow end-to-end:
- Wallet opens and shows consent screen
- User accepts → webhook fires with `verified`
- User denies → webhook fires with `rejected`

### Stage 4: Production

Once all stages pass, you're ready for real users.

---

## Pre-Production Checklist

### Security
- [ ] API key stored in secrets manager (not in code or git)
- [ ] Webhook endpoint uses HTTPS with valid TLS certificate
- [ ] Webhook signature validation implemented and tested
- [ ] API key rotation process documented

### Reliability
- [ ] Webhook handler returns 200 immediately, processes async
- [ ] Status polling implemented as webhook backup
- [ ] Error handling for all API response codes (400, 401, 429, 500)
- [ ] Retry logic for transient network failures

### Monitoring
- [ ] Webhook delivery success rate tracked
- [ ] API error rate tracked
- [ ] Alerts configured for failures

### Compliance
- [ ] Privacy policy updated to mention age verification via EUDI wallet
- [ ] Data retention policy defined for verification results
- [ ] GDPR data subject request process covers verification data

---

## API Key Management

```bash
# Create a new key
curl -X POST https://pylonid.eu/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com"}'

# Rotate (invalidates old key immediately)
curl -X POST https://pylonid.eu/v1/auth/rotate \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{}'
```

Best practices:
- Rotate keys periodically
- Revoke immediately if compromised
- Use separate keys per environment if running multiple instances
- Never commit keys to source control

---

## Webhook Testing with ngrok

To test webhooks during development without deploying:

```bash
ngrok http 3000
# Returns: https://abc123.ngrok.io → localhost:3000
```

Use the ngrok HTTPS URL as your `callbackUrl`:

```bash
curl -X POST https://pylonid.eu/v1/verify/age \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "policy": { "minAge": 18 },
    "callbackUrl": "https://abc123.ngrok.io/webhooks/pylon"
  }'
```

---

## Self-Hosting

PylonID can also be self-hosted on your own infrastructure. See the [repository README](https://github.com/pylon-id/pylon) for Docker Compose deployment instructions.

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
