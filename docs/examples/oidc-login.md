# Example: Sign in with EUDI

**Status:** Planned for Q4 2026.

---

## What This Will Do

"Sign in with EUDI" using standard OpenID Connect — like "Sign in with Google" but backed by a government-issued digital identity.

```
1. User clicks "Sign in with EUDI"
2. Your app redirects to PylonID OAuth authorize endpoint
3. User scans QR with EUDI wallet
4. User consents → PylonID redirects back with authorization code
5. Your app exchanges code for ID token
6. User is logged in
```

---

## Why OIDC?

- **Standard:** Industry-standard OpenID Connect — works with existing auth libraries
- **Familiar:** Same flow as Google, Apple, or Microsoft login
- **Secure:** OAuth 2.0 with PKCE
- **Private:** No passwords, no social profile scraping

---

## Planned Endpoints

| Endpoint | Purpose |
|----------|---------|
| `GET /.well-known/openid-configuration` | OIDC discovery |
| `GET /oauth/authorize` | Start auth flow |
| `POST /oauth/token` | Exchange code for tokens |
| `GET /oauth/userinfo` | Get user attributes |

---

## Planned Flow

```javascript
// 1. Redirect to PylonID
window.location.href = 'https://pylonid.eu/oauth/authorize?' +
  new URLSearchParams({
    client_id: 'your-client-id',
    redirect_uri: 'https://yourapp.com/callback',
    response_type: 'code',
    scope: 'openid age_over_18',
    code_challenge: pkceChallenge,
    code_challenge_method: 'S256'
  });

// 2. Exchange code (on your backend)
const tokens = await fetch('https://pylonid.eu/oauth/token', {
  method: 'POST',
  body: new URLSearchParams({
    grant_type: 'authorization_code',
    code: authorizationCode,
    redirect_uri: 'https://yourapp.com/callback',
    client_id: 'your-client-id',
    code_verifier: pkceVerifier
  })
}).then(r => r.json());

// tokens.id_token contains verified claims
```

---

**Current:** Use [Age Verification](./age-verification.md) (available now)
**Reference:** [API Reference](../3-api-reference.md) | [Changelog](../10-changelog.md)
