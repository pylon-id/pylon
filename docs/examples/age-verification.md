# Example: Age Verification Integration

Complete working example of age verification in your app using direct HTTP integration.

---

## The Flow

```
1. User clicks "Verify age"
2. Your app calls POST /v1/verify/age
3. PYLON returns walletUrl
4. User scans QR code with EUDI wallet
5. Wallet asks: "Share age > 18?"
6. User taps Accept/Deny
7. Wallet sends proof to PYLON
8. PYLON validates and fires webhook
9. Your app gets result and grants/denies access
```

---

## Node.js + Express

```
import express from 'express';
import crypto from 'crypto';
import fetch from 'node-fetch';
import QRCode from 'qrcode';

const app = express();
app.use(express.json());

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
  
  // Generate QR code
  const qr = await QRCode.toDataURL(data.walletUrl);

  res.json({
    verificationId: data.verificationId,
    qrCode: qr,
    walletUrl: data.walletUrl
  });
});

// 2. Handle webhook
app.post('/api/webhooks/pylon', express.raw({ type: 'application/json' }), async (req, res) => {
  const signature = req.headers['x-pylon-signature'];
  const body = req.body.toString();
  
  // Validate signature
  function validateSignature(sig, body, secret) {
    const [t, v1] = sig.split(',');
    const tValue = t.replace('t=', '');
    const signedMessage = `${tValue}.${body}`;
    const computed = crypto.createHmac('sha256', secret).update(signedMessage).digest('hex');
    return crypto.timingSafeEqual(Buffer.from(v1.replace('v1=', '')), Buffer.from(computed));
  }
  
  if (!validateSignature(signature, body, process.env.PYLON_WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const payload = JSON.parse(body);
  const { verificationId, result, attributes } = payload;

  if (result === 'verified' && attributes.ageOver18) {
    console.log(`✅ User verified as age > 18`);
  } else {
    console.log(`❌ Verification failed or rejected`);
  }

  res.status(200).json({ received: true });
});

app.listen(3000);
```

---

## Python + Flask

```
import os
import requests
import hmac
import hashlib
from flask import Flask, request, jsonify
import qrcode
from io import BytesIO
import base64

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
    qr = qrcode.QRCode(version=1, box_size=10, border=5)
    qr.add_data(data['walletUrl'])
    qr.make(fit=True)

    img = qr.make_image(fill_color="black", back_color="white")
    buf = BytesIO()
    img.save(buf)
    qr_base64 = base64.b64encode(buf.getvalue()).decode()

    return jsonify({
        'verificationId': data['verificationId'],
        'qrCode': f'data:image/png;base64,{qr_base64}',
        'walletUrl': data['walletUrl']
    })

def validate_signature(signature, body, secret):
    parts = signature.split(',')
    t = parts.replace('t=', '')
    v1 = parts.replace('v1=', '')
    
    signed_message = f"{t}.{body}"
    computed = hmac.new(secret.encode(), signed_message.encode(), hashlib.sha256).hexdigest()
    
    return hmac.compare_digest(v1, computed)

@app.route('/webhook/pylon', methods=['POST'])
def pylon_webhook():
    signature = request.headers.get('X-Pylon-Signature')
    body = request.get_data().decode()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(signature, body, secret):
        return {'error': 'Invalid signature'}, 401

    data = request.json

    if data['result'] == 'verified' and data.get('attributes', {}).get('ageOver18'):
        print(f"✅ User {data['verificationId']} verified as age > 18")
        return {'received': True}, 200

    return {'received': True}, 200

if __name__ == '__main__':
    app.run(debug=True, port=5000)
```

---

## Testing Locally

```
# Start local emulator
pylon-cli

# In another terminal, start your app
node app.js  # (or python app.py)

# Make request
curl -X POST http://localhost:3000/api/verify-age

# Emulator auto-completes immediately
```

---

## Error Handling

```
try {
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

  if (response.status === 401) {
    console.error('Invalid API key');
  } else if (response.status === 429) {
    console.error('Rate limited');
  } else if (response.status === 400) {
    console.error('Invalid callback URL (must be HTTPS)');
  }
} catch (error) {
  console.error('Network error:', error);
}
```

---

**Next:** [API Reference](../3-api-reference.md) | [Troubleshooting](../8-troubleshooting.md)
