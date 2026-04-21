# Rust Integration

Direct HTTP integration with `reqwest`.

---

## Dependencies

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
subtle = "2"
```

---

## Start a Verification

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct VerifyRequest {
    policy: Policy,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
}

#[derive(Serialize)]
struct Policy {
    #[serde(rename = "minAge")]
    min_age: u32,
}

#[derive(Deserialize)]
struct VerifyResponse {
    #[serde(rename = "verificationId")]
    verification_id: String,
    #[serde(rename = "walletUrl")]
    wallet_url: String,
    #[serde(rename = "requestUri")]
    request_uri: String,
    #[serde(rename = "expiresAt")]
    expires_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("PYLON_API_KEY")?;

    let resp: VerifyResponse = reqwest::Client::new()
        .post("https://pylonid.eu/v1/verify/age")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&VerifyRequest {
            policy: Policy { min_age: 18 },
            callback_url: "https://yourapp.com/webhooks/pylon".into(),
        })
        .send()
        .await?
        .json()
        .await?;

    println!("Verification: {}", resp.verification_id);
    println!("Wallet URL:   {}", resp.wallet_url);
    // Display resp.wallet_url as QR code

    Ok(())
}
```

---

## Handle Webhooks (Axum)

```rust
use axum::{http::{HeaderMap, StatusCode}, routing::post, Json, Router};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

fn validate_signature(body: &[u8], signature: &str, secret: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let computed = format!("sha256={}", hex::encode(mac.finalize().into_bytes()));
    computed.as_bytes().ct_eq(signature.as_bytes()).into()
}

async fn webhook_handler(
    headers: HeaderMap,
    body: String,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let signature = headers.get("x-pylon-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = std::env::var("PYLON_WEBHOOK_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !validate_signature(body.as_bytes(), signature, &secret) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let payload: serde_json::Value = serde_json::from_str(&body)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("{}: {}", payload["verificationId"], payload["status"]);

    Ok(Json(serde_json::json!({"received": true})))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/webhooks/pylon", post(webhook_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

---

## Error Handling

```rust
let resp = reqwest::Client::new()
    .post("https://pylonid.eu/v1/verify/age")
    .header("Authorization", format!("Bearer {api_key}"))
    .json(&request)
    .send()
    .await?;

match resp.status().as_u16() {
    200 => { /* success */ }
    401 => eprintln!("Invalid API key"),
    400 => eprintln!("Invalid request"),
    429 => eprintln!("Rate limited — back off and retry"),
    code => eprintln!("Unexpected: {code}"),
}
```

---

**Reference:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
