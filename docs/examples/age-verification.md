# Example: Age Verification

Complete working examples of age verification integration.

---

## The Flow

```
1. User clicks "Verify age" in your app
2. Your backend calls POST /v1/verify/age
3. PylonID returns walletUrl
4. Your frontend displays walletUrl as QR code
5. User scans QR with EUDI wallet
6. Wallet shows: "Share age_over_18?"
7. User consents → wallet sends proof to PylonID
8. PylonID validates → fires webhook to your backend
9. Your app grants or denies access
```

---

## Node.js + Express

```javascript
import express from 'express';
import crypto from 'crypto';
import QRCode from 'qrcode';

const app = express();

// 1. Start verification
app.post('/api/verify-age', async (req, res) => {
  const response = await fetch('https://pylonid.eu/v1/verify/age', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${process.env.PYLON_API_KEY}`
    },
    body: JSON.stringify({
      policy: { minAge: 18 },
      callbackUrl: 'https://yourapp.com/api/webhooks/pylon'
    })
  });

  const data = await response.json();
  const qr = await QRCode.toDataURL(data.walletUrl);

  res.json({
    verificationId: data.verificationId,
    qrCode: qr,
    walletUrl: data.walletUrl
  });
});

// 2. Handle webhook
function validateSignature(body, signature, secret) {
  const computed = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(computed));
}

app.post('/api/webhooks/pylon', express.raw({ type: 'application/json' }), (req, res) => {
  const signature = req.headers['x-pylon-signature'];

  if (!validateSignature(req.body, signature, process.env.PYLON_WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const { verificationId, status, result } = JSON.parse(req.body);

  if (status === 'verified' && result.age_over_18) {
    console.log(`✅ ${verificationId}: age verified`);
    // Grant access in your database
  } else {
    console.log(`❌ ${verificationId}: ${status}`);
  }

  res.status(200).json({ received: true });
});

app.listen(3000);
```

---

## Python + Flask

```python
import os, hmac, hashlib, requests, qrcode, base64
from io import BytesIO
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/verify-age', methods=['POST'])
def start_verification():
    response = requests.post(
        'https://pylonid.eu/v1/verify/age',
        json={
            'policy': {'minAge': 18},
            'callbackUrl': 'https://yourapp.com/webhook/pylon'
        },
        headers={'Authorization': f"Bearer {os.getenv('PYLON_API_KEY')}"}
    )
    data = response.json()

    # Generate QR code
    qr = qrcode.make(data['walletUrl'])
    buf = BytesIO()
    qr.save(buf)
    qr_b64 = base64.b64encode(buf.getvalue()).decode()

    return jsonify({
        'verificationId': data['verificationId'],
        'qrCode': f'data:image/png;base64,{qr_b64}',
        'walletUrl': data['walletUrl']
    })

def validate_signature(body, signature, secret):
    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature, computed)

@app.route('/webhook/pylon', methods=['POST'])
def pylon_webhook():
    signature = request.headers.get('X-Pylon-Signature', '')
    body = request.get_data()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(body, signature, secret):
        return {'error': 'Invalid signature'}, 401

    data = request.json

    if data['status'] == 'verified' and data.get('result', {}).get('age_over_18'):
        print(f"✅ {data['verificationId']}: age verified")
    else:
        print(f"❌ {data['verificationId']}: {data['status']}")

    return {'received': True}, 200

if __name__ == '__main__':
    app.run(port=5000)
```

---

## Error Handling

```javascript
const response = await fetch('https://pylonid.eu/v1/verify/age', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${process.env.PYLON_API_KEY}`
  },
  body: JSON.stringify({
    policy: { minAge: 18 },
    callbackUrl: 'https://yourapp.com/webhook'
  })
});

if (!response.ok) {
  switch (response.status) {
    case 401: console.error('Invalid API key'); break;
    case 400: console.error('Invalid request (callbackUrl must be HTTPS)'); break;
    case 429: console.error('Rate limited — retry later'); break;
    default:  console.error(`Error: ${response.status}`); break;
  }
  return;
}

const data = await response.json();
// Display data.walletUrl as QR code
```

---

## Polling as Backup

If webhooks aren't suitable, poll the verification status:

```javascript
async function waitForResult(verificationId) {
  for (let i = 0; i < 60; i++) {
    const resp = await fetch(`https://pylonid.eu/v1/status/${verificationId}`, {
      headers: { 'Authorization': `Bearer ${process.env.PYLON_API_KEY}` }
    });
    const data = await resp.json();

    if (data.status !== 'pending') return data;
    await new Promise(r => setTimeout(r, 2000)); // Poll every 2s
  }
  return { status: 'timeout' };
}
```

---

**Next:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
