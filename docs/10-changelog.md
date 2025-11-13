# Changelog

## v1.0.0 (2025-11-06) — Public Beta Launch

### Added
- ✅ Age verification API (`POST /v1/verify/age`)
- ✅ Webhook delivery with exponential backoff retries
- ✅ Webhook signature validation (HMAC-SHA256)
- ✅ Idempotency keys for deduplication
- ✅ PostgreSQL persistence (data survives restarts)
- ✅ Health check endpoint
- ✅ Local emulator with mock wallet

### Known Limitations (Beta)
- 🟡 Signature validation is structural only (mock credentials accepted)
- 🟡 Real signature validation launching Nov 25, 2025
- 🟡 No API key authentication (public sandbox)
- 🟡 No rate limiting enforcement
- 🟡 No self-serve dashboard (email signup only)

### Infrastructure
- PostgreSQL database (self-hosted, Germany)
- Docker deployment with Caddy reverse proxy
- Data retention: 30 days (automatic cleanup)
- Webhook retry: 1s → 2s → 4s → 8s → 16s → 32s

### Migration Notes
**If upgrading from v0.1:**
1. Run new migrations: `migrations/20250206_003_webhook_schema_update.sql`
2. Redeploy pylon-server
3. Start cleanup job: `pylon-cleanup` (background process)

***

## Release Cycle

We release updates monthly. Check GitHub for latest version.

Check your version
```bash
curl https://pylonid.eu/health | grep version
```

All breaking changes announced 30 days in advance.
]
