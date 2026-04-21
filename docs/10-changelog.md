# Changelog

## v2.0.0 (2026-04-21) — EUDI Compliance

Phase 5 complete. Full OpenID4VP age verification with real EUDI wallet support.

### Added
- ✅ OpenID4VP authorization request objects (signed ES256 JWTs)
- ✅ Wallet URL generation (`eudi-openid4vp://` deep link scheme)
- ✅ Request URI endpoint (`GET /v1/oid4vp/request/:id`)
- ✅ VP Token response endpoint (`POST /v1/oid4vp/response`)
- ✅ Verifier metadata discovery (`GET /.well-known/openid-credential-verifier`)
- ✅ SD-JWT-VC parser (issuer JWT + disclosures + key binding JWT)
- ✅ ES256 signature verification against PID Issuer JWKS
- ✅ Key Binding JWT verification (nonce, audience, freshness)
- ✅ Selective disclosure claim reconstruction
- ✅ Integrated EUDI PID Issuer (Keycloak 26 + reference issuer)
- ✅ JWKS fetching and caching (TTL-based)
- ✅ API key authentication (`POST /v1/auth/signup`, `POST /v1/auth/rotate`)
- ✅ Stable verifier and issuer signing keys across restarts

### Changed
- Wallet URL format changed from `https://wallet.pylonid.eu/...` to `eudi-openid4vp://authorize?request_uri=...`
- Webhook signature format is `sha256={hex}` (HMAC-SHA256 of raw body)
- Response from `POST /v1/verify/age` now includes `requestUri` and real wallet deep link
- All endpoints served from `pylonid.eu` (no separate `api.pylonid.eu`)

### Infrastructure
- Caddy reverse proxy routing (API + website + EUDI stack)
- Keycloak 26 with pid-issuer-realm
- EUDI PID Issuer with PKCS12 keystore for stable signing keys
- PostgreSQL 16 with AES-256-GCM encryption at rest
- Docker Compose deployment (4 containers)

### Known Limitations
- No credential revocation checking
- No `presentation_submission` validation
- Database schema may change between versions
- No official SDKs — use direct HTTP

---

## v1.0.0 (2025-11-06) — Public Beta Launch

### Added
- ✅ Age verification API (`POST /v1/verify/age`)
- ✅ Webhook delivery with exponential backoff retries
- ✅ HMAC-SHA256 webhook signatures
- ✅ PostgreSQL persistence
- ✅ Health check endpoint
- ✅ Local emulator (pylon-cli)

### Known Limitations
- Signature validation was structural only (mock credentials)
- No API key authentication
- No rate limiting enforcement

### Infrastructure
- PostgreSQL (self-hosted, Germany)
- Docker deployment with Caddy reverse proxy
- Webhook retry: exponential backoff

---

## Release Cycle

Check your version:
```bash
curl https://pylonid.eu/health
```

Breaking changes announced in advance via GitHub releases.

---

**Updates:** Watch [github.com/pylon-id/pylon](https://github.com/pylon-id/pylon) for releases.
