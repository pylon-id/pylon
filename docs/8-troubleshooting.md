# Troubleshooting & FAQ

---

## Verification Issues

### Wallet doesn't open when scanning QR code

- **Desktop browser?** EUDI wallet deep links only work on mobile
- **Wallet installed?** User needs an EUDI wallet app
- **QR code content:** Should contain the full `eudi-openid4vp://authorize?request_uri=...` URL
- **Try direct link:** On mobile, open the `walletUrl` directly instead of scanning

### Verification stays "pending"

- User hasn't scanned the QR code yet
- User scanned but hasn't consented
- Verification expired (check `expiresAt` in response)
- Start a new verification if expired

### Verification times out

Verifications expire after the time shown in `expiresAt`. If the user doesn't respond:
- Show a "Try again" button
- Create a new verification request
- Don't charge or penalize the user

---

## Authentication Issues

### 401 Unauthorized

```bash
# Is your API key set?
echo $PYLON_API_KEY

# Is it in the right header format?
curl -H "Authorization: Bearer $PYLON_API_KEY" https://pylonid.eu/health
```

**Common causes:**
- Missing `Bearer` prefix in Authorization header
- Extra whitespace or newline in API key
- Key was rotated (old key invalidated)
- Key not yet created — run `POST /v1/auth/signup` first

### 400 Bad Request

- Check JSON syntax (valid JSON?)
- `minAge` must be between 1 and 150
- `callbackUrl` must be HTTPS (not HTTP)

### 429 Too Many Requests

Rate limiting is active. Back off and retry with exponential delay:

```javascript
async function withRetry(fn, maxAttempts = 3) {
  for (let i = 0; i < maxAttempts; i++) {
    try {
      return await fn();
    } catch (err) {
      if (err.status !== 429 || i === maxAttempts - 1) throw err;
      await new Promise(r => setTimeout(r, Math.pow(2, i) * 1000));
    }
  }
}
```

---

## Webhook Issues

### Webhook never fires

1. **Is `callbackUrl` HTTPS?** HTTP is rejected.
2. **Is it publicly reachable?** Test: `curl -X POST https://yourapp.com/webhook`
3. **Does it return HTTP 2xx?** Non-2xx triggers retries.
4. **Is verification still pending?** Check: `GET /v1/status/:id`
5. **Using localhost?** Use ngrok to expose local endpoints during development.

### Signature validation fails

**Most common cause:** validating parsed JSON instead of raw body bytes.

```javascript
// ❌ WRONG
app.post('/webhook', express.json(), (req, res) => {
  validate(JSON.stringify(req.body), signature); // Re-serialized — different bytes
});

// ✅ RIGHT
app.post('/webhook', express.raw({ type: 'application/json' }), (req, res) => {
  validate(req.body, signature); // Original bytes
});
```

**Also check:**
- Correct webhook secret (no extra whitespace)
- Header name is `X-Pylon-Signature` (case-insensitive in most frameworks)
- Signature format is `sha256={hex}` — compare the whole string

### Webhook times out

Your handler takes too long. Return 200 immediately:

```javascript
app.post('/webhook', (req, res) => {
  res.status(200).json({ received: true }); // Return first
  queue.add('process', req.body);            // Process later
});
```

---

## Local Emulator Issues

### Build fails

```bash
# Ensure Rust 1.75+
rustc --version

# Clean and rebuild
cd pylon_cli
cargo clean
cargo build --release
```

### Port 7777 in use

```bash
lsof -i :7777
kill -9 <PID>
```

### Emulator requests fail

```bash
# Is it running?
curl http://localhost:7777/health
```

If connection refused, the emulator isn't running. Start it from the `pylon_cli` directory.

---

## Credential Issues

### "Credential invalid" in webhook

The wallet sent an invalid or tampered credential. Ask the user to:
- Update their wallet app
- Re-issue the credential from their government provider
- Try again

### "Policy mismatch" in webhook

The user's credential doesn't meet the policy (e.g., user is 17, policy requires 18). Show a clear message: "You must be at least 18."

---

## Getting Help

| Channel | Use for |
|---------|---------|
| [hello@pylonid.eu](mailto:hello@pylonid.eu) | General questions |
| [security@pylonid.eu](mailto:security@pylonid.eu) | Security issues |
| [GitHub Issues](https://github.com/pylon-id/pylon/issues) | Bug reports |
| [GitHub Discussions](https://github.com/pylon-id/pylon/discussions) | Questions and feature requests |
| [pylonid.eu/status](https://pylonid.eu/status.html) | Service status |
