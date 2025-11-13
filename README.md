# PYLON SDK & Emulator

**Official documentation, emulator, and examples for PYLON age verification API.**

PYLON is a hosted API for European Digital Identity (EUDI) wallet verification. This repository contains:

- 📖 **Complete documentation** - Full integration guides
- 🛠️ **Local emulator** (`pylon-cli`) - Test without API keys
- 💻 **Integration examples** - Node.js, Python, Go, Rust, Java
- 📦 **SDK templates** - Official SDKs coming Q1 2026

---

## Quick Start

### 1. Use Hosted API (Recommended)

Sign up for free beta at [pylonid.eu](https://pylonid.eu)

- ✅ 1,000 free verifications/month
- ✅ EU-only data hosting
- ✅ Production SLA (99.9%+)
- ✅ No infrastructure to manage

```bash
curl -X POST https://pylonid.eu/v1/verify/age \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"https://yourapp.com/webhook"}'
``````

### 2. Local Testing (Development)

```
# Install emulator
cd pylon-cli
cargo build --release

# Start local test server
./target/release/pylon-cli

# Test API
curl -X POST http://localhost:7777/v1/verify/age \
  -H "Content-Type: application/json" \
  -d '{"policy":{"minAge":18},"callbackUrl":"http://localhost:3000/webhook"}'
``````

***

## Documentation

Full docs at [docs.pylonid.eu](https://docs.pylonid.eu)

- [Quickstart Guide](https://docs.pylonid.eu/1-quickstart.html)
- [API Reference](https://docs.pylonid.eu/3-api-reference.html)
- [Webhook Guide](https://docs.pylonid.eu/6-webhooks.html)
- [Local Testing](https://docs.pylonid.eu/5-local-testing.html)

***

## Integration Examples

See [`examples/`](./examples/) directory for complete working examples:

- **Node.js + Express** - Age verification with webhooks
- **Python + Flask** - Age verification with webhooks
- **Go** - Direct HTTP integration
- **Rust** - Native integration
- **Java + Spring Boot** - Age verification

***

## Architecture

PYLON uses:
- **OID4VP** - Credential presentation protocol
- **SD-JWT-VC** - Selective disclosure format
- **ISO 18013-5** - Mobile document standard

The emulator stubs these protocols for local testing. Production API validates real EUDI wallets.

***

## Self-Hosting (Advanced)

**Note:** Self-hosting requires significant infrastructure expertise. We recommend using the hosted API.

If you must self-host:
1. This repo contains the emulator only (test mode)
2. Production server requires:
   - Real EUDI wallet integration (OID4VP)
   - Credential signature verification (EdDSA, ECDSA)
   - PostgreSQL database
   - TLS certificates
   - Monitoring/alerting

For production deployment assistance, contact enterprise@pylonid.eu

***

## Roadmap

- **Q4 2025:** Beta testing, monitoring
- **Q1 2026:** Production launch, KYC attributes
- **Q2 2026:** OAuth/OIDC login, official SDKs

***

## Support

- **Documentation:** [docs.pylonid.eu](https://docs.pylonid.eu)
- **Bug reports:** [GitHub Issues](https://github.com/pylon-id/pylon/issues)
- **Feature requests:** Label as `enhancement`
- **Questions:** [GitHub Discussions](https://github.com/pylon-id/pylon/discussions)
- **Security:** security@pylonid.eu

***

## License

MIT - See [LICENSE](./LICENSE)

---

**Hosted by:** [pylonid.eu](https://pylonid.eu) | **Status:** [pylonid.eu/status](https://pylonid.eu/status.html)
