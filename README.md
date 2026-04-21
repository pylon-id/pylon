# PylonID — SDK & Emulator

**Documentation, local emulator, and integration examples for the PylonID age verification API.**

PylonID is a hosted API for European Digital Identity (EUDI) wallet verification. This repository contains:

- 📖 **Documentation** — Full integration guides at [docs.pylonid.eu](https://docs.pylonid.eu)
- 🛠️ **Local emulator** (`pylon_cli`) — Test the full flow without a real wallet
- 💻 **Integration examples** — Shell scripts, plus examples in Node.js, Python, Go, Rust, Java in the docs
- 📦 **SDK templates** — Official SDKs coming Q4 2026

---

## Quick Start

### Use the Hosted API

```bash
# 1. Get an API key
curl -X POST https://pylonid.eu/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email":"you@example.com"}'

# 2. Start an age verification
curl -X POST https://pylonid.eu/v1/verify/age \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://yourapp.com/webhook"}'

# 3. Display the returned walletUrl as a QR code
# 4. Customer scans with EUDI wallet, consents
# 5. You receive a signed webhook with the result
```

### Local Testing

```bash
# Rename Cargo files for standalone build
cp Cargo.toml.cli Cargo.toml
cp pylon_cli/Cargo.toml.cli pylon_cli/Cargo.toml

# Build and run emulator
cd pylon_cli
cargo build --release
./target/release/pylon-cli

# In another terminal — create a test verification
curl -X POST http://localhost:7777/v1/verify/age \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"http://localhost:3000/webhook"}'

# Open the returned walletUrl in your browser → click Accept/Reject
```

> **Note:** The `.cli` suffixed Cargo files are standalone builds for the emulator. The emulator has no external dependencies beyond what's in `Cargo.toml.cli`.

---

## How It Works

```
Your app                          PylonID                         EUDI Wallet
   │                                 │                                 │
   │  POST /v1/verify/age            │                                 │
   │────────────────────────────────>│                                 │
   │  { walletUrl, verificationId }  │                                 │
   │<────────────────────────────────│                                 │
   │                                 │                                 │
   │  Show QR code (walletUrl)       │                                 │
   │─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ >│
   │                                 │   Wallet fetches request JWT    │
   │                                 │<────────────────────────────────│
   │                                 │   Signed authorization request  │
   │                                 │────────────────────────────────>│
   │                                 │                                 │
   │                                 │   User consents                 │
   │                                 │                                 │
   │                                 │   Wallet sends VP token         │
   │                                 │<────────────────────────────────│
   │                                 │                                 │
   │  Webhook: verified/rejected     │                                 │
   │<────────────────────────────────│                                 │
```

---

## API Endpoints

### Authenticated (API key required)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/v1/auth/signup` | Create API key |
| POST | `/v1/auth/rotate` | Rotate API key |
| POST | `/v1/verify/age` | Start age verification |
| GET | `/v1/status/:id` | Poll verification status |

### Wallet-facing (no auth)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/v1/oid4vp/request/:id` | Authorization request JWT |
| POST | `/v1/oid4vp/response` | VP token submission |

### Discovery

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/.well-known/openid-credential-verifier` | Verifier metadata |
| GET | `/health` | Health check |

---

## Webhook

Webhooks are signed with HMAC-SHA256. Verify using the `X-Pylon-Signature` header:

```
X-Pylon-Signature: sha256=a1b2c3d4e5f6...
```

The signature is `sha256={hex(HMAC-SHA256(your_webhook_secret, raw_request_body))}`.

```json
{
  "event": "verification.completed",
  "verificationId": "ver_81CA6EC3CA80",
  "status": "verified",
  "result": {"age_over_18": true},
  "timestamp": "2026-04-21T22:00:00Z"
}
```

See the [Webhooks guide](https://docs.pylonid.eu/6-webhooks.html) for implementation examples in Node.js, Python, Go, Java, and Rust.

---

## Documentation

Full docs at [docs.pylonid.eu](https://docs.pylonid.eu):

- [Quickstart](https://docs.pylonid.eu/1-quickstart.html) — verify your first age in 10 minutes
- [API Reference](https://docs.pylonid.eu/3-api-reference.html) — all endpoints and response formats
- [Webhooks](https://docs.pylonid.eu/6-webhooks.html) — signature validation, retries, examples
- [Local Emulator](https://docs.pylonid.eu/5-local-testing.html) — offline development
- [Core Concepts](https://docs.pylonid.eu/2-core-concepts.html) — OpenID4VP, SD-JWT-VC, ES256
- [Security](https://docs.pylonid.eu/7-security-compliance.html) — encryption, compliance, your responsibilities

---

## Repository Structure

```
docs/              Documentation source (mdBook → docs.pylonid.eu)
pylon_cli/         Local development emulator (Rust)
examples/          API test scripts
Cargo.toml.cli     Workspace config for emulator (rename to Cargo.toml to build)
.env.example       Environment variable reference
LICENSE            MIT
ROADMAP.md         Development roadmap
```

---

## Architecture

PylonID uses:
- **OpenID4VP** — Credential presentation protocol
- **SD-JWT-VC** — Selective disclosure credentials
- **ES256** — ECDSA P-256 signatures
- **AES-256-GCM** — Encryption at rest
- **HMAC-SHA256** — Webhook signing

The emulator mocks the wallet interaction for local testing. The hosted API at pylonid.eu validates real EUDI wallet credentials.

---

## Roadmap

- ✅ Age verification (OpenID4VP, SD-JWT-VC, ES256)
- 🔄 Real wallet E2E testing
- 🔄 KYC attribute verification (Q3 2026)
- 🔄 OAuth/OIDC login (Q4 2026)
- 🔄 Official SDKs (Q4 2026)

See [ROADMAP.md](./ROADMAP.md) for details.

---

## Support

- **Docs:** [docs.pylonid.eu](https://docs.pylonid.eu)
- **Issues:** [GitHub Issues](https://github.com/pylon-id/pylon/issues)
- **Questions:** [GitHub Discussions](https://github.com/pylon-id/pylon/discussions)
- **General:** hello@pylonid.eu
- **Security:** security@pylonid.eu

---

## License

MIT — See [LICENSE](./LICENSE)
