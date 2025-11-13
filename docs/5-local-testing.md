# Local Testing with Emulator

**Status:** ✅ Production Ready

## Quick Start

The local emulator lets you test age verification without needing a real EUDI wallet.

### Prerequisites

```bash
# Ensure Rust 1.75+ is installed
rustc --version

# Build the emulator
cd ~/webstack/sites/pylon/pylon-cli
cargo build --release
```

### Run the Emulator

```bash
# Start emulator on localhost:7777
./target/release/pylon-cli
```

**Output:**
```
✨ PYLON Emulator Starting...
  🌐 Fake API: http://localhost:7777
  👤 Fake Wallet: http://localhost:7777
  📝 Ready for testing!
```

***

## Test Workflow

### Step 1: Create Age Verification

```bash
curl -X POST http://localhost:7777/v1/verify/age \\
  -H "Content-Type: application/json" \\
  -d '{
    "policy": {"minAge": 18},
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

Save the `verificationId` for later.

### Step 2: Open Fake Wallet UI

In your browser, visit:
```
http://localhost:7777/scan/ver_local_ABC123
```

You'll see a fake wallet interface with **Accept** and **Reject** buttons.

### Step 3: Accept Verification

Click the **Accept** button. The emulator will:
1. Create a test credential presentation
2. Validate the age logic
3. Fire a webhook to your callback URL
4. Mark the verification as completed

### Step 4: Receive Webhook

Your webhook endpoint at `http://localhost:3000/webhook` receives:

```json
{
  "verificationId": "ver_local_ABC123",
  "type": "age",
  "result": "verified",
  "attributes": {
    "ageOver18": true
  },
  "evidence": {
    "issuer": "LOCAL_TEST",
    "credentialType": "SD-JWT VC",
    "proofHash": "sha256:test123...",
    "issuedAt": "2025-01-15T14:30:00Z"
  },
  "audit": {
    "traceId": "trace_ver_local_ABC123"
  }
}
```

***

## Test with Your App

### Express.js Example

```javascript
const express = require('express');
const app = express();
app.use(express.json());

// Step 1: Create verification request
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
  res.redirect(data.walletUrl); // Redirect to fake wallet UI
});

// Step 2: Receive webhook
app.post('/webhook', (req, res) => {
  const { verificationId, result } = req.body;
  console.log(`✅ Verification ${verificationId}: ${result}`);
  res.status(200).json({ received: true });
});

app.listen(3000, () => console.log('App running on :3000'));
```

**Test it by running:**
```bash
# In terminal 1, start your app
node app.js

# In terminal 2, start PYLON emulator
./target/release/pylon-cli

# In terminal 3, trigger verification
curl http://localhost:3000/start
```

Then:
- Browser opens fake wallet at `http://localhost:7777/scan/...`
- Click **Accept**
- See console log: ✅ Verification ver_local_ABC123: verified

***

## Testing Webhook Retries

The emulator doesn't retry, but production API does. To test:
```bash
# Start a failing webhook server that returns 500
python3 -c "
from http.server import HTTPServer, BaseHTTPRequestHandler

class FailHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        self.send_response(500)
        self.end_headers()

HTTPServer(('', 4000), FailHandler).serve_forever()
"

# Use callbackUrl pointing to failing server
curl -X POST http://localhost:7777/v1/verify/age \\
  -d '{"policy":{"minAge":18},"callbackUrl":"http://localhost:4000/webhook"}'
```

Production retries with exponential backoff from 1s to 32s.

***

## Emulator Features

| Feature | Behavior |
|---------|----------|
| **Age Validation** | Checks `minAge` against mock credential |
| **Webhook Firing** | Sends POST to callback URL immediately |
| **State** | In-memory, clears on restart |
| **Retry** | None (fires once immediately) |
| **Signature** | No signature validation |

***

## Production Differences

| Aspect | Emulator | Production |
|--------|----------|------------|
| **URL** | `http://localhost:7777` | `https://pylonid.eu` |
| **Wallet** | Fake HTML UI | Real German EUDI wallet |
| **Signatures** | Mocked | Real OID4VP signature verification |
| **Retry** | None | Exponential backoff retries |
| **Storage** | In-memory | PostgreSQL persistent |
| **Auth** | None | API key required (Q1 2026) |

***

## Troubleshooting

### Port 7777 Already in Use

```bash
lsof -i :7777   # Find process using port
kill -9 <PID>   # Kill blocking process
```

### Webhook Not Firing

Test webhook endpoint locally:

```bash
curl -X POST http://localhost:3000/webhook \\
  -H "Content-Type: application/json" \\
  -d '{"test":true}'
```

### Verification Not Found

Make sure verification ID matches format:

```
ver_local_XXXXXXXX
```

***

## Next Steps

1. Integrate official SDKs (Go, JS, Python, Rust, Java)
2. Test error handling by sending invalid payloads
3. Test webhooks for proper app behavior
4. Move to production with real URLs

See [API Reference](./3-api-reference.md) for full API docs.

***

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email support@pylonid.eu
]
