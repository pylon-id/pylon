# SDKs

Official SDKs are planned for Q4 2026. Until then, integrate directly via HTTP — the API is intentionally simple.

| Language | Package | Status |
|----------|---------|--------|
| [Go](./go.md) | `github.com/pylon-id/sdk-go` | Planned (Q4 2026) |
| [JavaScript/TypeScript](./javascript.md) | `@pylon-id/sdk` | Planned (Q4 2026) |
| [Python](./python.md) | `pylon-id` | Planned (Q4 2026) |
| [Java](./java.md) | `com.pylonid:sdk` | Planned (Q4 2026) |
| [Rust](./rust.md) | `pylon-sdk` | Planned (Q4 2026) |

Each SDK page includes complete working examples for direct HTTP integration — calling the API, handling webhooks, and validating signatures.

---

## Integration Pattern

Every language follows the same pattern:

1. Call `POST /v1/verify/age` with your API key, policy, and callback URL
2. Get `walletUrl` from the response — display as QR code
3. Customer scans with EUDI wallet and consents
4. PylonID POSTs result to your `callbackUrl`
5. Validate the `X-Pylon-Signature` header (HMAC-SHA256)
6. Process the verification result

---

## Webhook Signature

Every webhook includes an `X-Pylon-Signature` header:

```
X-Pylon-Signature: sha256=a1b2c3d4e5f6...
```

This is `HMAC-SHA256(your_webhook_secret, raw_request_body)` formatted as `sha256={hex}`.

**Always validate signatures** and **always use the raw request body** (before JSON parsing). See each SDK page for language-specific examples, or the [Webhooks guide](../6-webhooks.md) for full details.

---

## Environment Variables

```bash
export PYLON_API_KEY="pyl_..."
export PYLON_WEBHOOK_SECRET="your-webhook-secret"
```

---

**Questions?** See [API Reference](../3-api-reference.md) or email [hello@pylonid.eu](mailto:hello@pylonid.eu)
