# What is PYLON?

**PYLON** is a developer-friendly API for European Digital Identity (EUDI) wallet integration. Instead of learning 500+ pages of cryptographic standards (OID4VC, SD-JWT VC, ISO 18013-5), you call simple REST endpoints.

## One Endpoint. Three Outcomes.

```bash
POST /v1/verify/age
{
  "policy": { "minAge": 18 },
  "callbackUrl": "https://app.example.com/webhooks/pylon"
}
```

Returns:
- `verificationId`: Unique ID for this request
- `walletUrl`: QR code for user to scan with EUDI wallet

User scans → Wallet presents credential → PYLON validates → Your webhook fires with result. That's it.

## Core Features

- **Age Verification**: Selective disclosure for age gates
- **KYC Reuse**: Leverage verified attributes without storing PII
- **OIDC Login**: "Sign in with EUDI" via OpenID Connect
- **Qualified Signatures**: ETSI-compliant digital signatures

## Why PYLON?

| Feature | PYLON |
|---------|-------|
| **Time to integrate** | 10 minutes |
| **Learning curve** | Minimal (REST API, not cryptography) |
| **Data sovereignty** | EU-only, no US sub-processors |
| **Lock-in** | None (standards-native, export guaranteed) |
| **Developer DX** | SDKs, emulator, Postman, docs |

## eIDAS 2.0 Compliance

The European Digital Identity Regulation (eIDAS 2.0) mandates:
- **Dec 2026**: Member states provide EUDI Wallet to citizens
- **Dec 2027**: Financial, healthcare, and mobility sectors **must** accept EUDI Wallet

PYLON is built for this deadline. Start integrating now.

## Next Steps

- **5 minutes**: Read [Quickstart](./1-quickstart.md)
- **15 minutes**: Try the [Local Emulator](./5-local-testing.md)
- **30 minutes**: Deploy to [Sandbox](./4-sandbox-guide.md)

---

**Questions?** Check [Troubleshooting](./8-troubleshooting.md) or email support@pylonid.eu
