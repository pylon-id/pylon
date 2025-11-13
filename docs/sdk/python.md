# Python SDK

**Status:** 🔄 Planned. Not yet available.

Official Python SDK for PYLON is under development. Use direct HTTP integration until released.

---

## Current Integration (Direct HTTP)

Until the SDK is available, use the requests library:

```bash
pip install requests
```

```python
import os
import requests

def verify_age():
    api_key = os.getenv("PYLON_API_KEY")
    
    response = requests.post(
        "{BASE_URL}/v1/verify/age",
        json={{
            "policy": {{"minAge": 18}},
            "callbackUrl": "https://app.example.com/webhooks/pylon"
        }},
        headers={{
            "Content-Type": "application/json",
            "Authorization": f"Bearer {{api_key}}"
        }}
    )
    
    data = response.json()
    print(f"Verification ID: {{data['verificationId']}}")
    print(f"Wallet URL: {{data['walletUrl']}}")
    # Redirect user to data['walletUrl']

verify_age()
```

---

## Handle Webhooks (Flask)

```python
import os
import hmac
import hashlib
from flask import Flask, request, jsonify

app = Flask(__name__)

def validate_pylon_webhook(signature, body, secret):
    \"\"\"Validate X-Pylon-Signature header\"\"\"
    parts = signature.split(',')
    if len(parts) != 2:
        return False
    
    t = parts[0].replace('t=', '')
    v1 = parts[1].replace('v1=', '')
    
    signed_message = f"{{t}}.{{body}}"
    computed = hmac.new(
        secret.encode(),
        signed_message.encode(),
        hashlib.sha256
    ).hexdigest()
    
    return hmac.compare_digest(v1, computed)

@app.route("/webhooks/pylon", methods=["POST"])
def pylon_webhook():
    signature = request.headers.get("X-Pylon-Signature")
    body = request.get_data().decode()
    secret = os.getenv("PYLON_WEBHOOK_SECRET")

    if not validate_pylon_webhook(signature, body, secret):
        return {{"error": "Invalid signature"}}, 401

    data = request.json

    if data["result"] == "verified":
        print("✅ Verified!")
        return {{"received": True}}, 200

    return {{"received": True}}, 200

if __name__ == "__main__":
    app.run(debug=True, port=3000)
```

---

## Handle Webhooks (FastAPI)

```python
import os
import hmac
import hashlib
from fastapi import FastAPI, Request, HTTPException

app = FastAPI()

def validate_pylon_webhook(signature: str, body: str, secret: str) -> bool:
    parts = signature.split(',')
    if len(parts) != 2:
        return False
    
    t = parts[0].replace('t=', '')
    v1 = parts[1].replace('v1=', '')
    
    signed_message = f"{{t}}.{{body}}"
    computed = hmac.new(
        secret.encode(),
        signed_message.encode(),
        hashlib.sha256
    ).hexdigest()
    
    return hmac.compare_digest(v1, computed)

@app.post("/webhooks/pylon")
async def pylon_webhook(request: Request):
    signature = request.headers.get("X-Pylon-Signature")
    body = (await request.body()).decode()
    secret = os.getenv("PYLON_WEBHOOK_SECRET")

    if not validate_pylon_webhook(signature, body, secret):
        raise HTTPException(status_code=401, detail="Invalid signature")

    data = await request.json()

    if data["result"] == "verified":
        print("✅ Verified!")
        return {{"received": True}}

    return {{"received": True}}
```

---

## Idempotency Handling

```python
import os
from flask import Flask, request
from datetime import datetime

app = Flask(__name__)
processed_webhooks = {{}}  # Use database in production

@app.route("/webhooks/pylon", methods=["POST"])
def pylon_webhook():
    idempotency_key = request.headers.get("X-Pylon-Idempotency-Key")

    # Check if already processed
    if idempotency_key in processed_webhooks:
        return {{"status": "already_processed"}}, 200

    # Validate signature
    signature = request.headers.get("X-Pylon-Signature")
    body = request.get_data().decode()
    secret = os.getenv("PYLON_WEBHOOK_SECRET")
    
    if not validate_pylon_webhook(signature, body, secret):
        return {{"error": "Invalid signature"}}, 401

    # Store idempotency key
    data = request.json
    processed_webhooks[idempotency_key] = {{
        "verification_id": data["verificationId"],
        "result": data["result"],
        "processed_at": datetime.utcnow().isoformat(),
    }}

    # Return 200 immediately
    return {{"received": True}}, 200
    
    # Process asynchronously (use Celery, RQ, etc.)
```

---

## Error Handling

```python
import requests
import os

try:
    api_key = os.getenv("PYLON_API_KEY")
    
    response = requests.post(
        "{BASE_URL}/v1/verify/age",
        json={{
            "policy": {{"minAge": 18}},
            "callbackUrl": "https://app.example.com/webhooks/pylon"
        }},
        headers={{"Authorization": f"Bearer {{api_key}}"}}
    )
    
    if response.status_code == 401:
        print("❌ Invalid API key")
    elif response.status_code == 429:
        print("❌ Rate limited")
    elif response.status_code == 400:
        print(f"❌ Invalid request: {{response.json()}}")
    elif response.ok:
        print(f"✅ Success: {{response.json()}}")
    else:
        print(f"❌ Error: {{response.status_code}}")
        
except requests.exceptions.RequestException as e:
    print(f"❌ Network error: {{e}}")
```

---

## Testing Locally

Start the local emulator:

```bash
pylon-cli
```

Point requests to localhost:

```python
import requests

response = requests.post(
    "http://localhost:7777/v1/verify/age",
    json={{
        "policy": {{"minAge": 18}},
        "callbackUrl": "http://localhost:3000/webhooks/pylon"
    }}
)

print(response.json())
```

---

## Roadmap

- **Q1 2026:** Official Python SDK with type hints and async support

---

**Questions?** See [Troubleshooting](../8-troubleshooting.md) or [API Reference](../3-api-reference.md)
