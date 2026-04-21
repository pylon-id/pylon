# What is PylonID?

**PylonID** is a turnkey API for verifying attributes from European Digital Identity (EUDI) wallets. Instead of implementing 500+ pages of cryptographic standards, you call one REST endpoint.

## One Endpoint. One QR Code. One Webhook.

```bash
POST /v1/verify/age
{
  "policy": { "minAge": 18 },
  "callbackUrl": "https://yourapp.com/webhooks/pylon"
}
```

Returns a `walletUrl` — display it as a QR code. Customer scans with their EUDI wallet, consents, and you receive a signed webhook with the result. That's it.

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

## Current Status

🟢 **Beta** — OpenID4VP age verification with real EUDI wallet support.

### Working Now
- ✅ Age verification via OpenID4VP
- ✅ SD-JWT-VC parsing and ES256 signature verification
- ✅ Signed authorization request objects
- ✅ Key Binding JWT verification
- ✅ JWKS fetching from PID Issuer
- ✅ HMAC-SHA256 signed webhooks
- ✅ API key management (signup, rotation)
- ✅ Integrated PID Issuer (Keycloak + EUDI reference issuer)

### Planned
- 🔄 KYC attribute verification (Q3 2026)
- 🔄 OAuth/OIDC "Sign in with EUDI" (Q4 2026)
- 🔄 Official SDKs — Go, JS, Python, Rust, Java (Q4 2026)

## Why PylonID?

| | |
|---|---|
| **Time to integrate** | 10 minutes |
| **Learning curve** | REST API, not cryptography |
| **Data sovereignty** | EU-only, self-hosted, no US sub-processors |
| **Lock-in** | None — standards-native (OpenID4VP, SD-JWT-VC) |
| **Deployment** | Self-hosted via Docker Compose |

## eIDAS 2.0 Compliance

The European Digital Identity Regulation mandates:
- **Dec 2026**: Member states must provide EUDI wallets to citizens
- **Dec 2027**: Financial, healthcare, and mobility sectors **must** accept EUDI wallets

PylonID is built for this deadline.

## Next Steps

- **5 minutes**: [Quickstart](./1-quickstart.md) — verify your first age
- **15 minutes**: [Local Emulator](./5-local-testing.md) — test without a wallet
- **30 minutes**: [Self-host](./4-sandbox-guide.md) — deploy your own instance

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
