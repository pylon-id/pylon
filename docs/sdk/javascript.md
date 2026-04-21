# JavaScript / TypeScript Integration

Direct HTTP integration with fetch (Node.js 18+, Deno, Bun, or browser).

---

## Start a Verification

```javascript
const response = await fetch('https://pylonid.eu/v1/verify/age', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${process.env.PYLON_API_KEY}`
  },
  body: JSON.stringify({
    policy: { minAge: 18 },
    callbackUrl: 'https://yourapp.com/webhooks/pylon'
  })
});

const data = await response.json();
console.log('Verification:', data.verificationId);
console.log('Wallet URL:', data.walletUrl);
// Display data.walletUrl as QR code
```

---

## Handle Webhooks (Express)

```javascript
import express from 'express';
import crypto from 'crypto';

const app = express();

function validateSignature(body, signature, secret) {
  const computed = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(computed));
}

app.post('/webhooks/pylon', express.raw({ type: 'application/json' }), (req, res) => {
  const signature = req.headers['x-pylon-signature'];

  if (!validateSignature(req.body, signature, process.env.PYLON_WEBHOOK_SECRET)) {
    return res.status(401).json({ error: 'Invalid signature' });
  }

  const payload = JSON.parse(req.body);

  if (payload.status === 'verified') {
    console.log(`✅ ${payload.verificationId}: verified`);
  } else {
    console.log(`❌ ${payload.verificationId}: ${payload.status}`);
  }

  res.status(200).json({ received: true });
});

app.listen(3000);
```

---

## Handle Webhooks (Next.js API Route)

```typescript
// app/api/webhooks/pylon/route.ts
import crypto from 'crypto';
import { NextRequest, NextResponse } from 'next/server';

function validateSignature(body: Buffer, signature: string, secret: string): boolean {
  const computed = 'sha256=' + crypto
    .createHmac('sha256', secret)
    .update(body)
    .digest('hex');
  return crypto.timingSafeEqual(Buffer.from(signature), Buffer.from(computed));
}

export async function POST(req: NextRequest) {
  const body = Buffer.from(await req.arrayBuffer());
  const signature = req.headers.get('x-pylon-signature') ?? '';
  const secret = process.env.PYLON_WEBHOOK_SECRET!;

  if (!validateSignature(body, signature, secret)) {
    return NextResponse.json({ error: 'Invalid signature' }, { status: 401 });
  }

  const payload = JSON.parse(body.toString());

  if (payload.status === 'verified') {
    // Grant access
  }

  return NextResponse.json({ received: true });
}
```

---

## TypeScript Types

```typescript
interface VerifyAgeRequest {
  policy: { minAge: number };
  callbackUrl: string;
}

interface VerifyAgeResponse {
  verificationId: string;
  status: string;
  walletUrl: string;
  requestUri: string;
  expiresAt: string;
}

interface WebhookPayload {
  event: 'verification.completed';
  verificationId: string;
  status: 'verified' | 'rejected' | 'expired' | 'error';
  result: { age_over_18?: boolean };
  timestamp: string;
}
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
    callbackUrl: 'https://yourapp.com/webhooks/pylon'
  })
});

if (!response.ok) {
  switch (response.status) {
    case 401: console.error('Invalid API key'); break;
    case 400: console.error('Invalid request'); break;
    case 429: console.error('Rate limited'); break;
    default:  console.error(`Error: ${response.status}`); break;
  }
}
```

---

**Reference:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
