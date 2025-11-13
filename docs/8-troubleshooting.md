# Troubleshooting & FAQ

Quick solutions for common issues.

***

## Verification Issues

### "walletUrl not opening in wallet app"

**Symptoms:** User sees browser page, wallet doesn't open

**Solutions:**
1. **Check URL format:** Should be `https://pylonid.eu/...`
2. **Test on mobile:** Desktop browsers can't open wallet
3. **Wallet installed?** Ask user to install EUDI wallet first
4. **QR code:** If URL doesn't work, display QR code instead
5. **Deep linking:** Some wallets require specific link format

### "User sees 'Wallet not found'"

**Cause:** No compatible EUDI wallet installed

**Solutions:**
1. Redirect user to wallet app store
2. Support multiple wallets (Austria, Germany, Italy)
3. Provide fallback (alternative verification method)

### "Verification times out (user doesn't respond)"

**Timeout:** 15 minutes

**Solutions:**
1. Extend timeout on your UI (show timer, refresh link)
2. Allow user to start over (new verification request)
3. Don't charge user if they time out

***

## Authentication Issues

### "401 Unauthorized"

**Cause:** Invalid or missing API key

**Debug:**
```bash
echo $PYLON_API_KEY           # Is it set?
echo ${#PYLON_API_KEY}         # Is it long enough? (>30 chars)
curl -H "Authorization: Bearer $PYLON_API_KEY" https://pylonid.eu/health
```

**Solutions:**
1. Copy API key from dashboard again
2. Check it's the correct environment (sandbox vs production)
3. Ensure no whitespace/newlines in key
4. Regenerate key if it's >90 days old

### "403 Forbidden"

**Cause:** API key lacks permission

**Solutions:**
1. Check key scope in dashboard (should have full access)
2. Rotate key if permissions were recently changed
3. Contact [support@pylonid.eu](mailto:support@pylonid.eu) if issue persists

***

## Rate Limiting

### "429 Too Many Requests"

**Limits:**
- Free tier: 1,000 ops/month
- Sandbox: 10,000 ops/month
- Pay-as-you-go: Unlimited (charged per request)

**Solutions:**

1. **Check usage:** Dashboard > Usage
2. **Wait 60 seconds:** PYLON resets limits hourly
3. **Upgrade tier:** Paid plans have higher limits
4. **Batch requests:** Combine multiple verifications if possible

**Example retry logic:**

```javascript
async function retryWithBackoff(fn, maxAttempts = 3) {
  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      return await fn();
    } catch (error) {
      if (error.status !== 429 || attempt === maxAttempts) {
        throw error;
      }
      // Wait 2^attempt seconds
      const delay = Math.pow(2, attempt) * 1000;
      await new Promise(resolve => setTimeout(resolve, delay));
    }
  }
}
```

***

## Webhook Issues

### "My webhook never fires"

**Checklist:**

1. **Is callback URL HTTPS?** (HTTP rejected)
   ```bash
   curl https://app.example.com/webhook
   # Should return 200, not error
   ```

2. **Is it publicly accessible?**
   ```bash
   # Test from another machine
   curl https://app.example.com/webhook
   ```

3. **Does it return HTTP 200?**
   ```bash
   curl -i https://app.example.com/webhook
   # Look for "200 OK" in headers
   ```

4. **Check delivery log:** Dashboard > Webhooks > Delivery Log
   - Shows attempted deliveries
   - Error messages if delivery failed

**Solutions:**

- **Firewall blocking:** Whitelist PYLON IPs (ask support)
- **SSL certificate expired:** Renew certificate
- **Service down:** Check uptime of your webhook server
- **Timeout >10s:** Your handler is too slow (see below)

### "Webhook signature validation fails"

**Debug steps:**

```javascript
// 1. Ensure you have raw body (before JSON.parse)
console.log(typeof req.body);  // Should be Buffer or string

// 2. Print what you're validating
const signature = req.headers['x-pylon-signature'];
const body = req.body.toString();
const secret = process.env.PYLON_WEBHOOK_SECRET;

console.log('Signature:', signature);
console.log('Body length:', body.length);
console.log('Secret:', secret ? '***' : 'MISSING');

// 3. Manually verify
const [t, v1] = signature.split(',');
const tValue = t.replace('t=', '');
const v1Value = v1.replace('v1=', '');

const signedMsg = `${tValue}.${body}`;
const crypto = require('crypto');
const manual = crypto
  .createHmac('sha256', secret)
  .update(signedMsg)
  .digest('hex');

console.log('Expected:', v1Value);
console.log('Computed:', manual);
console.log('Match:', v1Value === manual);
```

**Common mistakes:**

- ❌ Using `JSON.stringify(req.body)` instead of raw body
- ❌ Missing `t=` or `v1=` prefix in parsing
- ❌ Wrong secret (copy-pasted with extra spaces)
- ❌ Using older SDK with old signature format

### "Webhook times out"

**Symptoms:** Webhook fires, but times out before your app responds

**Cause:** Handler takes >10 seconds

**Solution:** Return 200 immediately, process async:

```javascript
app.post('/webhooks/pylon', async (req, res) => {
  // Validate quickly
  if (!validateSignature(...)) {
    return res.status(401).send('Invalid');
  }

  // Return immediately (don't wait for processing)
  res.status(200).json({ received: true });

  // Process in background
  queue.add('processWebhook', req.body);
});
```

***

## Credential Issues

### "Credential expired"

**Error:** User's credential is >5 years old

**Cause:** Government credential has expired

**Solution:**
- User must renew credential with government wallet
- Provide link to credential renewal
- Retry verification after renewal

### "Credential doesn't meet policy"

**Error:** User is 17, policy requires 18

**Result:** Webhook shows `result: "not_verified"` with `reason: "policy_mismatch"`

**Solution:**
- Show friendly error: "You must be at least 18"
- Offer retry when they turn 18

### "Credential invalid"

**Cause:** Wallet sent invalid/tampered credential

**Result:** Webhook shows `result: "not_verified"` with `reason: "credential_invalid"`

**Solution:**
- Ask user to try again
- Check their wallet is up to date
- Ask them to update wallet app

***

## Testing Issues

### "Local emulator not working"

```bash
# Install
npm install -g pylon-cli

# Start
pylon dev

# Should see:
# ✅ Issuer running on http://localhost:8001
# ✅ Wallet running on http://localhost:8002
# ✅ Proxy running on http://localhost:7777
```

**If it fails:**

```bash
# Kill existing process
lsof -i :7777  # Find what's using port
kill -9 <PID>

# Try again
pylon dev
```

### "Emulator requests fail"

**Check endpoints:**

```bash
curl http://localhost:7777/v1/verify/age
# Should return 200, not connection error
```

**If connection refused:**
- Emulator not running (see above)
- Wrong port (check `pylon dev` output)
- Firewall blocking localhost

***

## Production Readiness

### Checklist Before Going Live

- [ ] **Local testing:** Works with emulator
- [ ] **Sandbox testing:** Works with German wallet (Nov 2025+)
- [ ] **Monitoring:** Error rate tracked
- [ ] **Logging:** All webhooks logged
- [ ] **Alerts:** Notified if webhook delivery fails
- [ ] **Handling:** Error cases handled gracefully
- [ ] **Security:** API key stored securely, webhook validated
- [ ] **Documentation:** Team knows integration details
- [ ] **Backup:** Can handle PYLON outages (graceful fallback)

***

## Getting Help

**GitHub Issues:**
- Bug reports: [github.com/pylon-id/issues](https://github.com/pylon-id/pylon/issues)
- Feature requests: Label as `enhancement`
- Questions: [github.com/pylon-id/discussions](https://github.com/pylon-id/pylon/discussions)

**Email Support:**
- [support@pylonid.eu](mailto:support@pylonid.eu) (24h response)
- [security@pylonid.eu](mailto:security@pylonid.eu) (security issues)
- [compliance@pylonid.eu](mailto:compliance@pylonid.eu) (legal/compliance)

**Community Discord:**
- [discord.gg/pylon-id](https://discord.gg/pylon-id)
- Real-time help from team + developers

**Status Page:**
- [pylonid.eu/status](https://pylonid.eu/status.html)
- Real-time system status
]

