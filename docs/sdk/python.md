# Python Integration

Direct HTTP integration with the `requests` library.

---

## Start a Verification

```python
import os
import requests

response = requests.post(
    'https://pylonid.eu/v1/verify/age',
    json={
        'policy': {'minAge': 18},
        'callbackUrl': 'https://yourapp.com/webhooks/pylon'
    },
    headers={
        'Authorization': f"Bearer {os.getenv('PYLON_API_KEY')}"
    }
)

data = response.json()
print(f"Verification: {data['verificationId']}")
print(f"Wallet URL:   {data['walletUrl']}")
# Display data['walletUrl'] as QR code
```

---

## Handle Webhooks (Flask)

```python
import hmac, hashlib, os
from flask import Flask, request

app = Flask(__name__)

def validate_signature(body, signature, secret):
    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature, computed)

@app.route('/webhooks/pylon', methods=['POST'])
def pylon_webhook():
    signature = request.headers.get('X-Pylon-Signature', '')
    body = request.get_data()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(body, signature, secret):
        return {'error': 'Invalid signature'}, 401

    data = request.json

    if data['status'] == 'verified':
        print(f"✅ {data['verificationId']}: verified")
    else:
        print(f"❌ {data['verificationId']}: {data['status']}")

    return {'received': True}, 200

if __name__ == '__main__':
    app.run(port=3000)
```

---

## Handle Webhooks (FastAPI)

```python
import hmac, hashlib, os
from fastapi import FastAPI, Request, HTTPException

app = FastAPI()

def validate_signature(body: bytes, signature: str, secret: str) -> bool:
    computed = 'sha256=' + hmac.new(
        secret.encode(), body, hashlib.sha256
    ).hexdigest()
    return hmac.compare_digest(signature, computed)

@app.post('/webhooks/pylon')
async def pylon_webhook(request: Request):
    signature = request.headers.get('X-Pylon-Signature', '')
    body = await request.body()
    secret = os.getenv('PYLON_WEBHOOK_SECRET')

    if not validate_signature(body, signature, secret):
        raise HTTPException(status_code=401, detail='Invalid signature')

    data = await request.json()

    if data['status'] == 'verified':
        print(f"✅ {data['verificationId']}: verified")

    return {'received': True}
```

---

## Error Handling

```python
import requests, os

response = requests.post(
    'https://pylonid.eu/v1/verify/age',
    json={
        'policy': {'minAge': 18},
        'callbackUrl': 'https://yourapp.com/webhooks/pylon'
    },
    headers={'Authorization': f"Bearer {os.getenv('PYLON_API_KEY')}"}
)

if response.status_code == 401:
    print('Invalid API key')
elif response.status_code == 400:
    print(f'Invalid request: {response.json()}')
elif response.status_code == 429:
    print('Rate limited — back off and retry')
elif response.ok:
    data = response.json()
    print(f"Wallet URL: {data['walletUrl']}")
```

---

**Reference:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
