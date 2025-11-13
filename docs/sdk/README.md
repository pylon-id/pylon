# PYLON SDKs

Official SDKs for PYLON. Development in progress.

---

## SDK Status

| Language | Package | Status |
|----------|---------|--------|
| **Go** | `github.com/pylon-id/sdk-go` | 🔄 Planned |
| **JavaScript/TypeScript** | `@pylon-id/sdk` | 🔄 Planned |
| **Python** | `pylon-id` | 🔄 Planned |
| **Rust** | `pylon-sdk` | 🔄 Planned |
| **Java** | `com.pylonid:sdk` | 🔄 Planned |

All SDKs are under development. Direct API integration is currently recommended.

---

## Current Integration Method

Use direct HTTP requests to the PYLON API until SDKs are released.

### API Endpoint

```
{BASE_URL}
```

### Local Testing

```
http://localhost:7777
```

Use the local emulator (`pylon-cli`) for development.

---

## Common Integration Pattern

1. Initialize HTTP client with API key
2. Call `POST /v1/verify/age` with policy and callback URL
3. Get `walletUrl` from response
4. Redirect user to `walletUrl`
5. Receive webhook when verification completes
6. Validate webhook signature (HMAC-SHA256)
7. Process verification result

---

## Environment Variables

```bash
export PYLON_API_KEY=<your-api-key>
export PYLON_WEBHOOK_SECRET=<your-webhook-secret>
```

---

## Webhook Signature Validation

**Critical:** Always validate webhook signatures to prevent spoofed requests.

### Validation Steps

1. Extract `X-Pylon-Signature` header
2. Get raw request body (bytes, before JSON parsing)
3. Retrieve webhook secret from environment
4. Compute HMAC-SHA256 signature
5. Compare using timing-safe comparison

See [Webhooks Guide](../6-webhooks.md) for implementation examples in multiple languages.

---

## Error Handling

Common error codes:

| Code | Meaning | Action |
|------|---------|--------|
| `INVALID_API_KEY` | API key missing or invalid | Check environment variable |
| `INVALID_CALLBACK_URL` | Callback not HTTPS | Use valid HTTPS URL |
| `NETWORK_ERROR` | Network failure | Retry with backoff |
| `UNKNOWN_ERROR` | Server error | Contact support if persists |

---

## Webhook Reliability

PYLON provides at-least-once delivery with:

- Exponential backoff retries (1s → 2s → 4s → 8s → 16s → 32s)
- Timeout: 10 seconds per attempt
- Max retries: 5 attempts
- Idempotency via `X-Pylon-Idempotency-Key` header

Return HTTP 200 to acknowledge receipt.

---

## Support

- **Questions:** See [Troubleshooting](../8-troubleshooting.md)
- **Issues:** [GitHub](https://github.com/y-uno23/pylon)
- **Email:** [support@pylonid.eu](mailto:support@pylonid.eu)

---

## Next Steps

- See [API Reference](../3-api-reference.md) for endpoint documentation
- Try [Local Testing](../5-local-testing.md) with the emulator
- Review [Webhooks Guide](../6-webhooks.md) for integration examples

---

## Roadmap

- **Q1 2026:** Official SDK releases for all listed languages
- **Q2 2026:** Additional language support on request

See [Changelog](../10-changelog.md) for updates.
