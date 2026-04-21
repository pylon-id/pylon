# Wallet Interoperability

PylonID works with any EUDI wallet that implements the required standards.

---

## Required Standards

Any compliant wallet must support:

| Standard | Purpose |
|----------|---------|
| OpenID4VP | Verifiable presentation protocol |
| SD-JWT-VC | Selective disclosure credentials |
| ES256 | Signature algorithm (ECDSA P-256) |

PylonID uses the `eudi-openid4vp://` URI scheme for wallet invocation, as specified in the EUDI Architecture Reference Framework.

---

## Wallet Landscape

### EUDI Reference Wallet

The EU-funded reference implementation. PylonID is built and tested against this wallet.

- [EUDI Wallet Reference Implementation](https://github.com/eu-digital-identity-wallet)

### Government Wallets

EU member states are deploying national EUDI wallets under eIDAS 2.0:

| Country | Status | Notes |
|---------|--------|-------|
| Germany | Pilot | BDr Wallet |
| Austria | Development | Building on eID Austria |
| Italy | Development | IO App integration |
| Others | Planned | Dec 2026 deadline for all member states |

### Commercial Wallets

| Provider | Status |
|----------|--------|
| Lissi | Developing EUDI support |
| Verimi | Developing EUDI support |
| walt.id | Developing EUDI support |

*Interoperability depends on each wallet's OID4VP and SD-JWT-VC compliance. Test with each wallet before production use.*

---

## What PylonID Validates

Regardless of which wallet presents the credential, PylonID verifies:

1. **Credential format** — valid SD-JWT-VC
2. **Credential type** — `urn:eudi:pid:1` (EU Person Identification Data)
3. **Issuer signature** — ES256 verified against issuer JWKS
4. **Key binding** — wallet proves credential ownership
5. **Freshness** — presentation created within 5-minute window
6. **Nonce** — matches the original request (replay protection)
7. **Disclosed claims** — `age_over_18` present and valid

---

## Testing Recommendations

1. **Start with the emulator** — deterministic, no wallet needed
2. **Test with EUDI reference wallet** — the baseline implementation
3. **Test with target wallets** — whichever wallets your users will have
4. **Monitor in production** — track success/failure rates per wallet type

---

## Conformance

- EUDI Wallet Conformance Tests: [github.com/EWC-consortium/ewc-wallet-conformance](https://github.com/EWC-consortium/ewc-wallet-conformance)
- PylonID's verifier metadata is published at `/.well-known/openid-credential-verifier`

---

## Roadmap

- **Q3 2026**: Test against commercial wallets (Lissi, Verimi, walt.id)
- **Q4 2026**: Track and publish wallet compatibility matrix
- **2027**: EU member state wallets mandatory — broad testing

---

**Wallet not working?** Report via [GitHub Issues](https://github.com/pylon-id/pylon/issues) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
