# Rust SDK

**Status:** 🔄 Planned. Not yet available.

Official Rust SDK for PYLON is under development. Use direct HTTP integration until released.

---

## Current Integration (Direct HTTP)

Until the SDK is available, use reqwest:

```toml
[dependencies]
reqwest = {{ version = "0.11", features = ["json"] }}
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
```

```rust
use reqwest;
use serde::{{Deserialize, Serialize}};
use std::env;

#[derive(Serialize)]
struct VerifyAgeRequest {{
    policy: AgePolicy,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
}}

#[derive(Serialize)]
struct AgePolicy {{
    #[serde(rename = "minAge")]
    min_age: u32,
}}

#[derive(Deserialize)]
struct VerifyAgeResponse {{
    #[serde(rename = "verificationId")]
    verification_id: String,
    status: String,
    #[serde(rename = "walletUrl")]
    wallet_url: String,
}}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let api_key = env::var("PYLON_API_KEY")?;
    
    let request = VerifyAgeRequest {{
        policy: AgePolicy {{ min_age: 18 }},
        callback_url: "https://app.example.com/webhooks/pylon".to_string(),
    }};
    
    let client = reqwest::Client::new();
    let resp = client
        .post("{BASE_URL}/v1/verify/age")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {{}}", api_key))
        .json(&request)
        .send()
        .await?;
    
    let result: VerifyAgeResponse = resp.json().await?;
    
    println!("Verification ID: {{}}", result.verification_id);
    println!("Wallet URL: {{}}", result.wallet_url);
    // Redirect user to result.wallet_url
    
    Ok(())
}}
```

---

## Handle Webhooks (Axum)

```rust
use axum::{{
    extract::State,
    http::{{HeaderMap, StatusCode}},
    response::IntoResponse,
    routing::post,
    Json, Router,
}};
use serde::{{Deserialize, Serialize}};
use hmac::{{Hmac, Mac}};
use sha2::Sha256;
use hex;

#[derive(Deserialize)]
struct WebhookPayload {{
    #[serde(rename = "verificationId")]
    verification_id: String,
    result: String,
}}

fn validate_signature(signature: &str, body: &str, secret: &str) -> bool {{
    let parts: Vec<&str> = signature.split(',').collect();
    if parts.len() != 2 {{
        return false;
    }}
    
    let t = parts[0].trim_start_matches("t=");
    let v1 = parts[1].trim_start_matches("v1=");
    
    let signed_message = format!("{{}}.{{}}", t, body);
    
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(signed_message.as_bytes());
    let computed = hex::encode(mac.finalize().into_bytes());
    
    v1 == computed
}}

async fn pylon_webhook(
    headers: HeaderMap,
    body: String,
) -> Result<Json<serde_json::Value>, StatusCode> {{
    let signature = headers
        .get("x-pylon-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let secret = std::env::var("PYLON_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !validate_signature(signature, &body, &secret) {{
        return Err(StatusCode::UNAUTHORIZED);
    }}
    
    let payload: WebhookPayload = serde_json::from_str(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    if payload.result == "verified" {{
        println!("✅ Verified!");
        return Ok(Json(serde_json::json!({{"received": true}})));
    }}
    
    Ok(Json(serde_json::json!({{"received": true}})))
}}

#[tokio::main]
async fn main() {{
    let app = Router::new()
        .route("/webhooks/pylon", post(pylon_webhook));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}}
```

Required dependencies:

```toml
[dependencies]
axum = "0.7"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
```

---

## Idempotency Handling

```rust
use std::collections::HashSet;
use std::sync::Mutex;

// In production, use a database instead
lazy_static::lazy_static! {{
    static ref PROCESSED: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}}

async fn pylon_webhook(
    headers: HeaderMap,
    body: String,
) -> Result<Json<serde_json::Value>, StatusCode> {{
    let idempotency_key = headers
        .get("x-pylon-idempotency-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;
    
    // Check if already processed
    {{
        let processed = PROCESSED.lock().unwrap();
        if processed.contains(idempotency_key) {{
            return Ok(Json(serde_json::json!({{"status": "already_processed"}})));
        }}
    }}
    
    // Validate signature
    let signature = headers.get("x-pylon-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let secret = std::env::var("PYLON_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if !validate_signature(signature, &body, &secret) {{
        return Err(StatusCode::UNAUTHORIZED);
    }}
    
    // Store idempotency key
    {{
        let mut processed = PROCESSED.lock().unwrap();
        processed.insert(idempotency_key.to_string());
    }}
    
    // Return 200 immediately
    Ok(Json(serde_json::json!({{"received": true}})))
    
    // Process asynchronously (spawn background task)
}}
```

---

## Error Handling

```rust
match client
    .post("{BASE_URL}/v1/verify/age")
    .header("Authorization", format!("Bearer {{}}", api_key))
    .json(&request)
    .send()
    .await
{{
    Ok(resp) => match resp.status().as_u16() {{
        401 => eprintln!("❌ Invalid API key"),
        429 => eprintln!("❌ Rate limited"),
        400 => eprintln!("❌ Invalid request"),
        200..=299 => println!("✅ Success"),
        code => eprintln!("❌ Error: {{}}", code),
    }},
    Err(e) => eprintln!("❌ Network error: {{}}", e),
}}
```

---

## Testing Locally

Start the local emulator:

```bash
pylon-cli
```

Point requests to localhost:

```rust
let resp = client
    .post("http://localhost:7777/v1/verify/age")
    .json(&request)
    .send()
    .await?;
```

---

## Roadmap

- **Q1 2026:** Official Rust SDK with async-first design using Tokio

---

**Questions?** See [Troubleshooting](../8-troubleshooting.md) or [API Reference](../3-api-reference.md)
