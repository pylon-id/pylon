# Local Testing with Emulator

Test the full age verification flow offline, without a real EUDI wallet.

---

## Quick Start

```bash
cd pylon_cli
cargo build --release
./target/release/pylon-cli
```

The emulator starts on `http://localhost:7777` and provides the same API as the production server.

---

## Test Workflow

### Step 1: Create a Verification

```bash
curl -X POST http://localhost:7777/v1/verify/age \
  -H "Content-Type: application/json" \
  -d '{
    "policy": { "minAge": 18 },
    "callbackUrl": "http://localhost:3000/webhook"
  }'
```

**Response:**
```json
{
  "verificationId": "ver_local_ABC123",
  "status": "pending",
  "walletUrl": "http://localhost:7777/scan/ver_local_ABC123"
}
```

### Step 2: Simulate Wallet Response

Open `http://localhost:7777/scan/ver_local_ABC123` in your browser. You'll see a simple UI with **Accept** and **Reject** buttons.

- **Accept** → emulator fires webhook with `"status": "verified"`
- **Reject** → emulator fires webhook with `"status": "rejected"`

### Step 3: Receive Webhook

Your webhook endpoint at `http://localhost:3000/webhook` receives:

```json
{
  "event": "verification.completed",
  "verificationId": "ver_local_ABC123",
  "status": "verified",
  "result": { "age_over_18": true },
  "timestamp": "2026-04-22T00:05:00Z"
}
```

---

## Integration Example (Express.js)

```javascript
const express = require('express');
const app = express();
app.use(express.json());

// Start verification
app.get('/start', async (req, res) => {
  const resp = await fetch('http://localhost:7777/v1/verify/age', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      policy: { minAge: 18 },
      callbackUrl: 'http://localhost:3000/webhook'
    })
  });
  const data = await resp.json();
  res.json(data); // Return walletUrl to display as QR
});

// Receive webhook
app.post('/webhook', (req, res) => {
  console.log(`${req.body.verificationId}: ${req.body.status}`);
  res.status(200).json({ received: true });
});

app.listen(3000, () => console.log('App on :3000'));
```

**Test it:**
```bash
# Terminal 1: Start your app
node app.js

# Terminal 2: Start emulator
cd pylon_cli && cargo run --release

# Terminal 3: Trigger verification
curl http://localhost:3000/start
# Open the returned walletUrl in browser, click Accept
```

---

## Emulator vs Production

| Aspect | Emulator | Production |
|--------|----------|------------|
| URL | `http://localhost:7777` | `https://pylonid.eu` |
| Wallet | Browser UI (Accept/Reject) | Real EUDI wallet app |
| Signatures | No SD-JWT-VC validation | Full ES256 + JWKS verification |
| Webhooks | Fires immediately, no retries | Retries with exponential backoff |
| Storage | In-memory (clears on restart) | PostgreSQL |
| Auth | No API key required | API key required |
| Webhook signing | No signature | HMAC-SHA256 signed |

---

## Prerequisites

```bash
# Rust 1.75+
rustc --version

# Build
cd pylon_cli
cargo build --release
```

---

## Troubleshooting

### Port 7777 already in use

```bash
lsof -i :7777
kill -9 <PID>
```

### Webhook not firing

Test that your endpoint is reachable:
```bash
curl -X POST http://localhost:3000/webhook \
  -H "Content-Type: application/json" \
  -d '{"test": true}'
```

### Connection refused

Emulator not running. Start it with `cargo run --release` from the `pylon_cli` directory.

---

## Next Steps

Once your integration works locally:
1. Switch to `https://pylonid.eu` with a real API key
2. Add webhook signature validation (emulator doesn't sign, production does)
3. Test with a real EUDI wallet

See [Integration & Testing](./4-sandbox-guide.md) for the full testing progression.

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
