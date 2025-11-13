# Core Concepts

PYLON abstracts three standards: OID4VP, SD-JWT VC, and ISO 18013-5. You don't need to know them to use PYLON, but understanding the fundamentals helps.

## Verifiable Credentials (VCs)

A **Verifiable Credential** is a digitally signed claim about an attribute.

**Example:** Government issues a credential: "Anna Müller, born 1990, age > 18"

Properties:
- **Issued by a trusted entity** (e.g., Austrian government)
- **Cryptographically signed** (can't be forged)
- **Expires** (e.g., 5 years from issuance)
- **Selective disclosure** (reveal only what's needed—e.g., just "age > 18", not full birthdate)

---

## OID4VP: Presentation Protocol

**OID4VP** = OpenID Connect for Verifiable Presentations

It's the protocol that says:
1. Your app requests a presentation: "Prove your age >= 18"
2. Wallet responds with a **presentation** (selective disclosure proof)
3. Your app verifies the cryptographic proof

**In PYLON:** You call `POST /v1/verify/age`. PYLON handles OID4VP handshakes internally, verifying the selective disclosure SD-JWT proof.

---

## SD-JWT VC: Self-Issued Credentials

**SD-JWT** = Selective Disclosure JSON Web Token

It's a JSON Web Token that allows the wallet holder to selectively reveal claims.

**Example:**
- Token contains: name, birthdate, address
- Wallet user chooses: reveal only "age > 18", hiding other claims
- Server receives cryptographic proof with only necessary claims

**In PYLON:** The server verifies SD-JWT signature and claims integrity automatically.

---

## ISO 18013-5/7: Mobile Document Standard

**ISO 18013** = International standard for mobile digital identity documents

Used by EUDI Wallets to manage government-issued IDs supporting offline, QR code scanning, and interoperability.

**In PYLON:** Currently supports ISO 18013-5. ISO 18013-7 support is planned.

---

## Verification Flow

The flow when a user verifies age through PYLON:

```
1. Your app calls: POST /v1/verify/age
   ↓
2. PYLON generates OID4VP request
   ↓
3. User scans QR code with EUDI wallet app
   ↓
4. Wallet prompts: "Verify age >= 18?"
   ↓
5. User accepts
   ↓
6. Wallet sends SD-JWT selective disclosure proof
   ↓
7. PYLON validates cryptographic proof
   ↓
8. PYLON verifies issuer trust & credential validity
   ↓
9. PYLON checks compliance to policy (min age)
   ↓
10. PYLON sends webhook with verification result
```

The entire cryptography is handled server-side by PYLON.

---

## Wallet Ecosystems

Supported wallet types:

### 1. Government EUDI Wallets

Issued by member states: Austria, Germany, Italy, Poland.

### 2. Commercial EUDI Wallets

Third-party wallets like Lissi, Verimi.

### 3. Upcoming Mobile Device Wallets

Native support planned in iOS/Android OS.

See [Wallet Interoperability](./9-interoperability.md) for details.

---

## Importance of PYLON

- Based on open standards
- Neutral to wallet vendor
- Future-proof for mandatory EUDI compliance
- eIDAS 2.0 compliant from launch

---

## Next Steps

- Try the [Quickstart](./1-quickstart.md)
- Deploy with [Sandbox Guide](./4-sandbox-guide.md)
- Deep dive in [API Reference](./3-api-reference.md)
- Ensure production reliability with [Webhooks](./6-webhooks.md)
- Use [Local Emulator](./5-local-testing.md) for offline dev

---

## Questions?

Refer to [Troubleshooting](./8-troubleshooting.md) or contact [support@pylonid.eu](mailto:support@pylonid.eu).
