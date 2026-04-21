# Core Concepts

PylonID abstracts three standards: OpenID4VP, SD-JWT-VC, and ES256 signatures. You don't need to know them to use PylonID, but understanding the fundamentals helps.

---

## Verifiable Credentials (VCs)

A **Verifiable Credential** is a digitally signed claim about a person, issued by a trusted authority.

**Example:** A government issues a credential: "Anna Müller, born 1990, age over 18: true"

Properties:
- **Issued by a trusted entity** (e.g., Austrian government)
- **Cryptographically signed** (can't be forged)
- **Has an expiry** (e.g., 5 years from issuance)
- **Supports selective disclosure** (reveal only "age over 18", not full birthdate)

---

## SD-JWT-VC: Selective Disclosure

**SD-JWT-VC** = Selective Disclosure JSON Web Token Verifiable Credential

A JWT where individual claims can be selectively revealed by the holder.

**How it works:**
1. Issuer creates a JWT with hashed claim placeholders (`_sd` array)
2. Each real claim value is in a separate **disclosure** (base64-encoded)
3. Holder chooses which disclosures to reveal
4. Verifier receives the JWT + selected disclosures, reconstructs only the revealed claims

**In PylonID:** When a wallet responds with a VP token, PylonID parses the SD-JWT-VC, matches disclosure hashes against `_sd` arrays, verifies the issuer signature, and extracts the revealed claims (e.g., `age_over_18`).

---

## OpenID4VP: Presentation Protocol

**OpenID4VP** = OpenID for Verifiable Presentations

The protocol for requesting and receiving verifiable credentials from wallets.

**The flow:**

```
1. PylonID creates a signed authorization request JWT containing:
   - presentation_definition: "I need age_over_18 from a PID credential"
   - response_uri: where the wallet should send the response
   - nonce: replay protection

2. Wallet fetches this request via request_uri

3. User sees: "PylonID wants to verify your age. Share age_over_18?"

4. User consents → wallet creates VP token:
   - SD-JWT-VC with only age_over_18 disclosed
   - Key Binding JWT (proves wallet holds the private key)

5. Wallet POSTs VP token to PylonID's response_uri

6. PylonID validates everything and fires webhook
```

**In PylonID:** You call `POST /v1/verify/age`. PylonID handles the entire OID4VP handshake — request signing, nonce management, token validation, issuer verification.

---

## ES256 Signatures

**ES256** = ECDSA using P-256 curve and SHA-256

Used throughout the EUDI ecosystem:
- **PID Issuer** signs credentials with ES256
- **PylonID** signs authorization requests with ES256
- **Wallet** signs Key Binding JWTs with ES256

PylonID verifies issuer signatures by fetching the issuer's public keys via JWKS (JSON Web Key Set).

---

## Key Binding JWT

When a wallet presents a credential, it includes a **Key Binding JWT** — a short-lived token proving the wallet actually holds the private key associated with the credential.

PylonID verifies:
- **Nonce** matches the authorization request
- **Audience** matches PylonID's client_id
- **Freshness** — issued within the last 5 minutes
- **Signature** — valid ES256 against the `cnf.jwk` in the credential

---

## The Verification Flow (Complete)

```
POST /v1/verify/age
  → PylonID creates verification record with nonce
  → Returns eudi-openid4vp:// wallet URL

Wallet scans QR code
  → GET /v1/oid4vp/request/:id
  → Receives signed ES256 JWT with presentation_definition

User consents in wallet
  → POST /v1/oid4vp/response
  → Sends vp_token (SD-JWT-VC) + state

PylonID validates:
  1. Parse SD-JWT → issuer JWT + disclosures + key binding JWT
  2. Check issuer URL and VCT (urn:eudi:pid:1)
  3. Fetch issuer JWKS → verify ES256 signature on issuer JWT
  4. Verify Key Binding JWT (nonce, audience, freshness, signature)
  5. Match disclosure hashes against _sd arrays
  6. Reconstruct revealed claims → extract age_over_18
  7. Update verification status → fire webhook
```

---

## Trust Model

PylonID trusts credentials based on:

1. **Issuer identity** — credential must come from a configured trusted PID Issuer
2. **Cryptographic signature** — issuer's ES256 signature must verify against their published JWKS
3. **Credential type** — must be `urn:eudi:pid:1` (EU Person Identification Data)
4. **Freshness** — Key Binding JWT must be recent (5-minute window)
5. **Binding** — wallet must prove possession of the credential's private key

---

## Wallet Ecosystems

### Government EUDI Wallets
Issued by EU member states under eIDAS 2.0. Austria, Germany, Italy, and others are deploying wallets.

### Commercial EUDI Wallets
Third-party wallets (Lissi, Verimi, walt.id) implementing the same standards.

### Mobile OS Integration
Native EUDI wallet support planned for iOS and Android.

All compliant wallets speak the same protocol — PylonID works with any of them.

See [Wallet Interoperability](./9-interoperability.md) for details.

---

## Next Steps

- [Quickstart](./1-quickstart.md) — verify your first age
- [API Reference](./3-api-reference.md) — all endpoints
- [Webhooks](./6-webhooks.md) — production webhook handling
- [Security](./7-security-compliance.md) — encryption and compliance

---

**Questions?** See [Troubleshooting](./8-troubleshooting.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
