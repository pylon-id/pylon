# Example: KYC Attribute Verification

**Status:** Planned for Q3 2026.

---

## What This Will Do

Verify identity attributes (name, date of birth, address, nationality) from EUDI wallets — without storing raw PII.

```
1. Your app calls POST /v1/verify/kyc
2. User scans QR with EUDI wallet
3. Wallet shows: "Share name, address, date of birth?"
4. User consents → wallet sends selective disclosure proof
5. PylonID validates and fires webhook with verified attributes
```

---

## Why It Matters

- **Privacy:** User controls exactly what's shared (selective disclosure)
- **Compliance:** You receive verified attributes, not raw documents
- **Trust:** Attributes are government-issued and cryptographically signed
- **GDPR:** Easier data retention and deletion — no document copies

---

## Planned API

```bash
curl -X POST https://pylonid.eu/v1/verify/kyc \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "attributes": ["given_name", "family_name", "date_of_birth", "address"],
    "callbackUrl": "https://yourapp.com/webhooks/pylon"
  }'
```

---

## Planned Webhook

```json
{
  "event": "verification.completed",
  "verificationId": "ver_...",
  "status": "verified",
  "result": {
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
  "timestamp": "2026-09-15T10:00:00Z"
}
```

---

**Current:** Use [Age Verification](./age-verification.md) (available now)
**Reference:** [API Reference](../3-api-reference.md) | [Changelog](../10-changelog.md)
