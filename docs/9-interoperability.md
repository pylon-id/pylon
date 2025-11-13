# Wallet Interoperability

PYLON supports major EUDI wallet standards and guides you through current testing and integration status.

***

## Standards Compliance

All compliant EUDI wallets must support:

| Standard       | Status           | Purpose                       |
|----------------|------------------|-------------------------------|
| OID4VP 1.0     | ✅ Final (Jun 2025) | Verifiable presentations protocol |
| SD-JWT VC      | ✅ Standard       | Selective disclosure credentials |
| ISO 18013-5/7  | ✅ Required       | Mobile document format        |
| FAPI 2 Baseline| ✅ Standard       | OAuth 2.0 security            |

***

## Government Wallets

| Country    | Wallet Name | Status    | Testing Status | Notes                          |
|------------|-------------|-----------|----------------|--------------------------------|
| Austria    | eID Austria | ✅ Live   | ✅ Tested      | Integrating EUDI features       |
| Italy      | IO App      | ✅ Live   | ✅ Tested      | Includes EUDI credentials       |
| Germany    | BDr Wallet  | 🔄 Sandbox| 🔄 Testing     | Sandbox launch expected Nov 2025|
| Greece     | TBD         | 🔄 Dev    | ⏳ Pending     | 2025-2026 rollout               |
| Luxembourg | TBD         | 🔄 Dev    | ⏳ Pending     | 2025-2026 rollout               |
| Poland     | TBD         | 🔄 Dev    | ⏳ Pending     | 2025-2026 rollout               |

***

## Commercial Wallets

| Provider | Status  | Notes                       |
|----------|---------|-----------------------------|
| Lissi    | 🔄 Beta | Testing OID4VP compliance   |
| Verimi   | 🔄 Beta | Testing SD-JWT support      |
| walt.id  | 🔄 Beta | Testing ISO 18013-5         |

*Feature support varies—test in sandbox before production.*

***

## Wallet Service Domain Status

`wallet.pylonid.eu` is currently a **placeholder** with no active wallet service.

Use:

- Local emulator (`http://localhost:7777`) for development
- Sandbox environment with real wallets for integration tests
- Production environment with actual user wallets

until a dedicated wallet service is launched.

***

## Testing Your Integration

Recommended sequence:

1. **Local emulator (pylon-cli):** Instant and deterministic
2. **Sandbox with German wallet (Nov 2025+):** Real wallet, test backend
3. **Production:** Live users, production backend

***

## Known Compatibility Notes

- OID4VP and selective disclosure are consistently supported
- Age > 18 verification supported
- HMAC-SHA256 webhook security standard enforced
- Attribute support and disclosure levels vary by wallet

***

## Issue Resolution

Confirm issues via local emulator and sandbox.

For unresolved issues:

- Contact [support@pylonid.eu](mailto:support@pylonid.eu)
- Use priority support for production users with paid plans

***

## Public Conformance & Contribution

- Conformance tests at [github.com/EWC-consortium/ewc-wallet-conformance](https://github.com/EWC-consortium/ewc-wallet-conformance)
- Quarterly results published
- Contribution via email, GitHub issues, Discord

***

## Roadmap

- Q1 2026: Test against Lissi, Verimi, walt.id
- Q2 2026: Native iOS/Android EUDI wallet support
- Q3 2026: EU member state wallets in testing

***

**Don’t see your wallet?** Email [support@pylonid.eu](mailto:support@pylonid.eu) to request testing.

]
