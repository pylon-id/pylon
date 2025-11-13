# Sandbox vs Production

Guide to choosing the right environment and migrating when ready.

***

## Environments

### Sandbox
- **URL:** `https://pylonid.eu`
- **Purpose:** Testing before going live
- **Data:** Reset on demand (non-persistent)
- **Wallets:** Simulated and German EUDI Wallet Sandbox (expected end Nov 2025)
- **Rate limit:** 10,000 requests/month (free)
- **Cost:** Free

### Production
- **URL:** `https://api.pylonid.eu`
- **Purpose:** Live applications
- **Data:** Persistent (production data)
- **Wallets:** Real government EUDI wallets (Austria, Germany, Italy, etc.)
- **Rate limit:** Depends on your plan
- **Cost:** Pay-as-you-go starting €0.10/request

***

## When to Move to Production

✅ **Ready for production when:**
- ✅ Local testing with emulator is passing
- ✅ Sandbox testing with real wallet is passing
- ✅ Webhook signature validation working
- ✅ Error handling implemented
- ✅ Rate limiting handled
- ✅ API key rotation strategy in place

❌ **NOT ready if:**
- ❌ Still debugging locally (use emulator first)
- ❌ Webhook URL not HTTPS
- ❌ No error handling
- ❌ No retry logic

***

## Testing Progression

```
1. Local Emulator (no network)
   ↓
2. Sandbox (simulated wallet)
   ↓
3. Sandbox + German Wallet (real wallet, sandbox backend)
   ↓
4. Production (real users, real wallets)
```

***

## Migration Checklist

### Step 1: Prepare

- [ ] Generate production API key in dashboard
- [ ] Update code to use production URL
- [ ] Test with production credentials locally
- [ ] Enable production logging
- [ ] Set up monitoring and alerting

### Step 2: Soft Launch

- [ ] Deploy to production with feature flag (off by default)
- [ ] Start with 1% of users
- [ ] Monitor error rates (<0.1% target)
- [ ] Monitor webhook delivery (>99.9% target)

### Step 3: Ramp Up

- [ ] 5% of users → observe for 24h
- [ ] 25% of users → observe for 24h
- [ ] 100% of users → full rollout

### Step 4: Monitor

- [ ] Daily checks for 1 week
- [ ] Weekly checks thereafter
- [ ] Alert on error rate >1%
- [ ] Alert on webhook delivery <99%

***

## API Key Management

### Sandbox API Keys
- Used for testing
- Separate from production keys
- Safe for example commits (not for real use)
- Regenerate if exposed

### Production API Keys
- **Never commit to repo** use environment variables only
- Rotate every 90 days
- Revoke immediately if compromised
- One key per environment per application

### Best Practices

```bash
# ✅ Use environment variable for key retrieval securely
export PYLON_API_KEY=$(aws secretsmanager get-secret-value --secret-id pylon-api-key | jq -r '.SecretString')

# ❌ Avoid hardcoded keys
const apiKey = "pk_live_abc123xyz";  # Bad

# ❌ Avoid committing keys to repo
git add .env  # Bad
```

***

## Environment Config Example

```javascript
// Node.js sample
const pylon = new PylonClient({
  apiKey: process.env.PYLON_API_KEY,
  baseUrl: process.env.PYLON_ENV === 'production'
    ? 'https://api.pylonid.eu'
    : 'https://pylonid.eu',  // same domain, env distinguished via API keys/session
});
```

```python
# Python sample
import os

client = Client(
    api_key=os.getenv('PYLON_API_KEY'),
    base_url=os.getenv('PYLON_BASE_URL', 'https://pylonid.eu'),
)
```

***

## Webhook URL Requirements

Webhook URLs must be:

- HTTPS (no HTTP allowed)
- Publicly accessible (not localhost)
- Respond within 10 seconds (HTTP 200)
- Use valid SSL certificates

### Testing Webhooks Locally

Use tools like ngrok to expose local endpoints:

```bash
ngrok http 3000
# Forwards https://abc123.ngrok.io → localhost:3000

# Use this URL as webhook endpoint in your app
```

***

## Data Retention

### Sandbox
- Verification and webhook logs kept for 7 days
- Manual reset allowed via dashboard

### Production
- Verification data kept for 90 days (configurable)
- Webhook logs kept 30 days
- Data deletion compliant with GDPR on request

***

## Support

- Sandbox issues via support@pylonid.eu or GitHub issues
- Production issues via priority support
- Incident reporting to security@pylonid.eu

***

## Questions?

See [Troubleshooting](./8-troubleshooting.md) or email support@pylonid.eu
]
