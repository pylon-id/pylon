# Example: OIDC Login

**Status:** 🔄 Planned. Not yet available.

"Sign in with EUDI" using OpenID Connect is under development. This example shows the planned integration pattern.

---

## Current Status

PYLON does not currently provide OAuth/OIDC login functionality. The following endpoints are **not yet implemented:**

- ❌ `/.well-known/openid-configuration`
- ❌ `/oauth/authorize`
- ❌ `/oauth/token`
- ❌ `/oauth/userinfo`

---

## Planned Flow

```
1. User clicks "Sign in with EUDI"
2. Your app redirects to PYLON OAuth
3. User scans QR code with wallet
4. Wallet asks: "Sign in to Your App?"
5. User taps Accept
6. PYLON redirects back with auth code
7. Your app exchanges code for ID token
8. User logged in
```

---

## Why OIDC? (When Available)

✅ **Standard:** Uses industry-standard OpenID Connect  
✅ **Familiar:** Works like "Sign in with Google"  
✅ **Secure:** OAuth 2.0 with PKCE  
✅ **Private:** No password sharing  

---

## Roadmap

- **Q2 2026:** OAuth/OIDC provider functionality
- **Q3 2026:** "Sign in with EUDI" button SDK

---

**Current:** Use [Age Verification](./age-verification.md) for now  
**Reference:** [API Reference](../3-api-reference.md) | [Troubleshooting](../8-troubleshooting.md)
