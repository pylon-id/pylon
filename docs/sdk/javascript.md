# JavaScript/TypeScript SDK

**Status:** 🔄 Planned. Not yet available.

Official JavaScript/TypeScript SDK for PYLON is under development. Use direct HTTP integration until released.

---

## Current Integration (Direct HTTP)

Until the SDK is available, use native fetch or axios:

```javascript
// Using fetch (Node.js 18+ or browser)
async function verifyAge() {{
  const response = await fetch('{BASE_URL}/v1/verify/age', {{
    method: 'POST',
    headers: {{
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${{process.env.PYLON_API_KEY}}`
    }},
    body: JSON.stringify({{
      policy: {{ minAge: 18 }},
      callbackUrl: 'https://app.example.com/webhooks/pylon'
    }})
  }});

  const data = await response.json();
  console.log('Verification ID:', data.verificationId);
  console.log('Wallet URL:', data.walletUrl);
  // Redirect user to data.walletUrl
}}

verifyAge();
```

---

## Handle Webhooks (Express)

```javascript
import express from 'express';
import crypto from 'crypto';

const app = express();
app.use(express.json());

function validatePylonWebhook(signature, body, secret) {{
  const [t, v1] = signature.split(',');
  const tValue = t.replace('t=', '');
  const v1Value = v1.replace('v1=', '');
  
  const signedMessage = `${{tValue}}.${{body}}`;
  const computed = crypto
    .createHmac('sha256', secret)
    .update(signedMessage)
    .digest('hex');

  return crypto.timingSafeEqual(
    Buffer.from(v1Value),
    Buffer.from(computed)
  );
}}

app.post('/webhooks/pylon', express.raw({{ type: 'application/json' }}), (req, res) => {{
  const signature = req.headers['x-pylon-signature'];
  const body = req.body.toString();
  const secret = process.env.PYLON_WEBHOOK_SECRET;

  if (!validatePylonWebhook(signature, body, secret)) {{
    return res.status(401).json({{ error: 'Invalid signature' }});
  }}

  const payload = JSON.parse(body);
  const {{ verificationId, result, attributes }} = payload;

  if (result === 'verified') {{
    console.log('✅ Verified!');
    return res.json({{ received: true }});
  }}

  res.json({{ received: true }});
}});

app.listen(3000, () => console.log('Webhook server on port 3000'));
```

---

## Handle Webhooks (Next.js API Route)

```javascript
import crypto from 'crypto';

function validatePylonWebhook(signature, body, secret) {{
  const [t, v1] = signature.split(',');
  const tValue = t.replace('t=', '');
  const v1Value = v1.replace('v1=', '');
  
  const signedMessage = `${{tValue}}.${{body}}`;
  const computed = crypto
    .createHmac('sha256', secret)
    .update(signedMessage)
    .digest('hex');

  return v1Value === computed;
}}

export default async function handler(req, res) {{
  if (req.method !== 'POST') {{
    return res.status(405).json({{ error: 'Method not allowed' }});
  }}

  const signature = req.headers['x-pylon-signature'];
  const body = JSON.stringify(req.body);
  const secret = process.env.PYLON_WEBHOOK_SECRET;

  if (!validatePylonWebhook(signature, body, secret)) {{
    return res.status(401).json({{ error: 'Invalid signature' }});
  }}

  const {{ verificationId, result }} = req.body;

  if (result === 'verified') {{
    return res.status(200).json({{ received: true }});
  }}

  return res.status(200).json({{ received: true }});
}}
```

---

## Idempotency Handling

```javascript
app.post('/webhooks/pylon', express.raw({{ type: 'application/json' }}), async (req, res) => {{
  const idempotencyKey = req.headers['x-pylon-idempotency-key'];

  // Check if already processed
  const existing = await db.webhooks.findOne({{ idempotencyKey }});
  if (existing) {{
    return res.status(200).json({{ status: 'already_processed' }});
  }}

  // Validate signature
  const signature = req.headers['x-pylon-signature'];
  const body = req.body.toString();
  if (!validatePylonWebhook(signature, body, process.env.PYLON_WEBHOOK_SECRET)) {{
    return res.status(401).json({{ error: 'Invalid signature' }});
  }}

  // Store idempotency key
  const payload = JSON.parse(body);
  await db.webhooks.insertOne({{
    idempotencyKey,
    verificationId: payload.verificationId,
    result: payload.result,
    processedAt: new Date(),
  }});

  // Return 200 immediately
  res.status(200).json({{ received: true }});

  // Process asynchronously
  processWebhookAsync(payload);
}});
```

---

## Error Handling

```javascript
try {{
  const response = await fetch('{BASE_URL}/v1/verify/age', {{
    method: 'POST',
    headers: {{
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${{process.env.PYLON_API_KEY}}`
    }},
    body: JSON.stringify({{
      policy: {{ minAge: 18 }},
      callbackUrl: 'https://app.example.com/webhooks/pylon'
    }})
  }});

  if (response.status === 401) {{
    console.error('❌ Invalid API key');
  }} else if (response.status === 429) {{
    console.error('❌ Rate limited');
  }} else if (response.status === 400) {{
    console.error('❌ Invalid request');
  }} else if (!response.ok) {{
    console.error('❌ Error:', response.status);
  }}
}} catch (error) {{
  console.error('❌ Network error:', error);
}}
```

---

## Testing Locally

Start the local emulator:

```bash
pylon-cli
```

Point requests to localhost:

```javascript
const response = await fetch('http://localhost:7777/v1/verify/age', {{
  method: 'POST',
  headers: {{ 'Content-Type': 'application/json' }},
  body: JSON.stringify({{
    policy: {{ minAge: 18 }},
    callbackUrl: 'http://localhost:3000/webhooks/pylon'
  }})
}});
```

---

## TypeScript Types

You can define your own types until the SDK is released:

```typescript
interface VerifyAgeRequest {{
  policy: {{
    minAge: number;
  }};
  callbackUrl: string;
}}

interface VerifyAgeResponse {{
  verificationId: string;
  status: string;
  walletUrl: string;
}}

interface WebhookPayload {{
  verificationId: string;
  type: string;
  result: 'verified' | 'not_verified';
  attributes?: {{
    ageOver18?: boolean;
  }};
}}
```

---

## Roadmap

- **Q1 2026:** Official JavaScript/TypeScript SDK with full type safety

---

**Questions?** See [Troubleshooting](../8-troubleshooting.md) or [API Reference](../3-api-reference.md)
