# Security & Compliance

PylonID is designed and developed independently by a sole developer. This document describes implemented security measures, compliance posture, and your responsibilities.

---

## Data Sovereignty

- All data processing occurs within the EU (self-hosted on EU infrastructure)
- No external sub-processors or third-party data processors
- No data leaves the deployment — PylonID doesn't phone home

---

## Encryption

### At Rest
- **AES-256-GCM** encryption for sensitive data in PostgreSQL
- Master encryption key provided via environment variable
- API keys hashed with bcrypt before storage

### In Transit
- **TLS 1.3** minimum for all external connections
- EUDI wallet communication over HTTPS
- Webhook delivery over HTTPS only

### Cryptographic Standards
- **ES256** (ECDSA P-256 + SHA-256) for authorization request signing
- **ES256** for SD-JWT-VC issuer signature verification
- **HMAC-SHA256** for webhook payload signing
- **JWKS** for public key distribution and rotation

---

## Authentication & Authorization

- API key authentication for all SMB endpoints
- Keys generated with cryptographic randomness
- Keys hashed with bcrypt — PylonID cannot retrieve your plaintext key
- Key rotation via `POST /v1/auth/rotate` (immediate invalidation of old key)

---

## Webhook Security

- Every webhook signed with HMAC-SHA256
- Signature in `X-Pylon-Signature: sha256={hex}` header
- Constant-time signature comparison (timing-attack resistant)
- HTTPS-only delivery (HTTP callback URLs rejected)

---

## Credential Verification

PylonID validates every presented credential:

1. **Issuer trust** — credential must come from a configured PID Issuer
2. **Signature** — ES256 signature verified against issuer's JWKS
3. **Credential type** — must be `urn:eudi:pid:1`
4. **Key binding** — wallet proves possession of credential private key
5. **Freshness** — Key Binding JWT must be within 5-minute window
6. **Nonce** — must match the original authorization request

---

## Data Minimization

- Only requested attributes are disclosed (selective disclosure via SD-JWT-VC)
- Age verification reveals only `age_over_18` (boolean), not birthdate
- Verification records store the result, not the raw credential
- Automatic cleanup of expired records via background worker

---

## Compliance Status

| Standard | Status |
|----------|--------|
| eIDAS 2.0 | Architecturally compliant (OpenID4VP, SD-JWT-VC) |
| GDPR | Designed for compliance (data minimization, EU-only) |
| ISO 27001 | Not certified (planned) |
| SOC 2 | Not audited (planned) |

PylonID is a beta product developed by a solo developer. No formal certifications or third-party audits have been obtained yet. The system is architected to follow EU standards and best practices.

---

## Audit Logging

- Immutable audit trail for verification events
- Anonymized IP logging
- Logs retained for 1 year minimum

---

## Your Responsibilities

- Store API keys securely (secrets manager, not source code)
- Rotate API keys periodically
- Validate webhook signatures on every request
- Use HTTPS-only webhook endpoints
- Implement idempotent webhook processing
- Define data retention policies for verification results
- Update your privacy policy to mention EUDI wallet verification
- Handle GDPR data subject requests for verification data

---

## Incident Response

- Report security issues to [security@pylonid.eu](mailto:security@pylonid.eu)
- Revoke compromised API keys immediately via `POST /v1/auth/rotate`
- PylonID will assist investigations if the platform is affected

---

## Security Checklist

- [ ] API key in secrets manager
- [ ] Webhook signature validation implemented
- [ ] HTTPS webhook endpoint with valid certificate
- [ ] Key rotation process documented
- [ ] Privacy policy updated
- [ ] Data retention policies defined
- [ ] Monitoring and alerting configured

---

**Security contact:** [security@pylonid.eu](mailto:security@pylonid.eu)
**General questions:** [hello@pylonid.eu](mailto:hello@pylonid.eu)
