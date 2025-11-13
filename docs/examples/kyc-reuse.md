# Example: KYC Reuse

**Status:** 🔄 Planned. Not yet available.

KYC verification via EUDI wallets is under development. This example shows the planned integration pattern.

---

## Current Status

The `/v1/verify/kyc` endpoint is not yet implemented. Only age verification (`/v1/verify/age`) is currently available.

---

## Planned Flow

```
1. User clicks "Complete KYC"
2. Your app calls POST /v1/verify/kyc (not yet available)
3. PYLON returns walletUrl
4. User scans QR code
5. Wallet shows: "Share name, address?"
6. User taps Accept
7. Wallet sends selective proof
8. PYLON validates and fires webhook
9. Your app gets attributes without storing raw data
```

---

## Why This Matters (When Available)

✅ **Privacy:** User controls what's shared (selective disclosure)  
✅ **Compliance:** Never store raw PII (just verified attributes)  
✅ **Trust:** Attributes come from government wallet  
✅ **GDPR:** Easier data retention/deletion  

---

## Roadmap

- **Q1 2026:** KYC verification endpoint
- **Q2 2026:** Additional attribute support

---

**Current:** Use [Age Verification](./age-verification.md) for now  
**Reference:** [API Reference](../3-api-reference.md) | [Troubleshooting](../8-troubleshooting.md)
