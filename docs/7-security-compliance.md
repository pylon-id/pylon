# Security & Compliance

PYLON is a privacy-first project designed and developed independently by a sole developer. This document describes implemented security measures, compliance goals, and your responsibilities.

***

## Data Sovereignty

- All verification data processing is designed to occur within the EU.
- There are currently **no external subprocessors** or third-party data processors involved.
- Efforts are made to comply with EU data privacy regulations.

***

## Compliance Status

- No formal certifications (ISO 27001, SOC 2, TISAX) or official audits have been obtained yet.
- The system is architected to follow standards such as eIDAS 2.0, OID4VP, SD-JWT, and ISO 18013.
- Plans for formal certification and audits are considered future goals.

***

## Security Measures

- Transport encryption with TLS 1.3 minimum
- HMAC-SHA256 webhook signatures with replay protection
- API key-based authentication and planned rotation workflow
- Rate limiting to mitigate abuse
- Minimal attribute storage, retaining only verified flags and audit linkage

***

## Developer & User Responsibilities

- Review suitability and compliance requirements for your use case.
- Store API keys securely and rotate them periodically.
- Use HTTPS-only webhook endpoints and validate all webhook signatures.
- Implement idempotent webhook processing to avoid duplicates.
- Respect data retention and GDPR requirements for personal data.

***

## Audit & Logging

- Immutable audit trails recording verification events with anonymized IPs.
- Logs maintained for at least 1 year available via API.
- Recommendation to log webhook handling, access, and deletions.

***

## Incident Response

- Report suspected breaches immediately to security@pylonid.eu.
- Revoke compromised API keys immediately.
- PYLON will assist investigations and communications if the platform is affected.

***

## Security Checklist Prior to Production

- API keys secured and rotated
- Webhook security enforced with signatures and HTTPS
- Idempotency enforced on webhook processing
- Privacy policy updated to mention PYLON integration
- Data retention policies defined and implemented
- Monitoring and alerting configured

***

## Contact & Support

- Security incidents: security@pylonid.eu
- Compliance queries: compliance@pylonid.eu
- General support: support@pylonid.eu

***

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email [support@pylonid.eu](mailto:support@pylonid.eu)
]
