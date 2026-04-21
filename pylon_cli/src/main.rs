use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyAgeRequest {
    policy: serde_json::Value,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyAgeResponse {
    #[serde(rename = "verificationId")]
    verification_id: String,
    status: String,
    #[serde(rename = "walletUrl")]
    wallet_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusResponse {
    #[serde(rename = "verificationId")]
    verification_id: String,
    status: String,
    result: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
struct PendingVerification {
    callback_url: String,
    min_age: u32,
    status: String,
    result: Option<serde_json::Value>,
}

#[derive(Clone)]
struct AppState {
    verifications: Arc<RwLock<HashMap<String, PendingVerification>>>,
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "pylon-emulator",
        "version": "1.0.0",
    }))
}

async fn verify_age(
    State(state): State<AppState>,
    Json(req): Json<VerifyAgeRequest>,
) -> Result<(StatusCode, Json<VerifyAgeResponse>), (StatusCode, String)> {
    let min_age = req.policy.get("minAge")
        .and_then(|v| v.as_u64())
        .unwrap_or(18) as u32;

    let verification_id = format!(
        "ver_local_{}",
        &uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    );

    println!("  New verification: {}", verification_id);
    println!("  Min age: {}", min_age);
    println!("  Callback: {}", req.callback_url);

    state.verifications.write().await.insert(
        verification_id.clone(),
        PendingVerification {
            callback_url: req.callback_url,
            min_age,
            status: "pending".into(),
            result: None,
        },
    );

    let wallet_url = format!("http://localhost:7777/scan/{}", verification_id);

    Ok((StatusCode::OK, Json(VerifyAgeResponse {
        verification_id,
        status: "pending".into(),
        wallet_url,
    })))
}

async fn get_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<StatusResponse>, (StatusCode, String)> {
    let verifications = state.verifications.read().await;
    let v = verifications.get(&id)
        .ok_or((StatusCode::NOT_FOUND, "Verification not found".into()))?;

    Ok(Json(StatusResponse {
        verification_id: id,
        status: v.status.clone(),
        result: v.result.clone(),
    }))
}

async fn simulate_accept(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("\n  User accepted: {}", id);

    let pending = state.verifications.read().await
        .get(&id).cloned()
        .ok_or((StatusCode::NOT_FOUND, "Verification not found".into()))?;

    // Mock result — emulator always verifies age as true
    let result = serde_json::json!({ "age_over_18": true });

    state.verifications.write().await
        .entry(id.clone())
        .and_modify(|v| {
            v.status = "verified".into();
            v.result = Some(result.clone());
        });

    // Fire webhook
    let callback_url = pending.callback_url.clone();
    let vid = id.clone();
    tokio::spawn(async move {
        let payload = serde_json::json!({
            "event": "verification.completed",
            "verificationId": vid,
            "status": "verified",
            "result": { "age_over_18": true },
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        println!("  Webhook -> {}", callback_url);

        match reqwest::Client::new()
            .post(&callback_url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => println!("  Webhook response: {}", resp.status()),
            Err(e) => println!("  Webhook failed: {}", e),
        }
    });

    Ok(StatusCode::OK)
}

async fn simulate_reject(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("\n  User rejected: {}", id);

    let pending = state.verifications.read().await
        .get(&id).cloned()
        .ok_or((StatusCode::NOT_FOUND, "Verification not found".into()))?;

    state.verifications.write().await
        .entry(id.clone())
        .and_modify(|v| {
            v.status = "rejected".into();
        });

    // Fire webhook
    let callback_url = pending.callback_url.clone();
    let vid = id.clone();
    tokio::spawn(async move {
        let payload = serde_json::json!({
            "event": "verification.completed",
            "verificationId": vid,
            "status": "rejected",
            "result": {},
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        println!("  Webhook -> {}", callback_url);

        match reqwest::Client::new()
            .post(&callback_url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => println!("  Webhook response: {}", resp.status()),
            Err(e) => println!("  Webhook failed: {}", e),
        }
    });

    Ok(StatusCode::OK)
}

async fn wallet_ui(Path(id): Path<String>) -> Html<String> {
    Html(format!(r#"<!DOCTYPE html>
<html>
<head><title>PylonID — Wallet Emulator</title>
<style>
body {{ font-family: system-ui, -apple-system, sans-serif; display: flex; justify-content: center;
       align-items: center; height: 100vh; margin: 0; background: #0f172a; color: #e2e8f0; }}
.card {{ background: #1e293b; border-radius: 16px; padding: 48px; box-shadow: 0 20px 60px rgba(0,0,0,0.5);
         text-align: center; max-width: 420px; border: 1px solid #334155; }}
h1 {{ font-size: 20px; margin: 0 0 8px 0; color: #f8fafc; }}
.sub {{ color: #94a3b8; font-size: 14px; margin: 0 0 32px 0; }}
.id {{ font-family: monospace; font-size: 12px; color: #64748b; margin: 0 0 24px 0; }}
button {{ padding: 14px 32px; border: none; border-radius: 8px; font-size: 15px;
          font-weight: 600; cursor: pointer; margin: 6px; transition: opacity 0.2s; }}
button:hover {{ opacity: 0.85; }}
.accept {{ background: #10b981; color: white; }}
.reject {{ background: #ef4444; color: white; }}
#msg {{ margin-top: 24px; font-weight: 600; font-size: 15px; min-height: 24px; }}
.ok {{ color: #10b981; }}
.fail {{ color: #ef4444; }}
.info {{ color: #94a3b8; }}
</style></head>
<body><div class="card">
<h1>EUDI Wallet Emulator</h1>
<p class="sub">PylonID is requesting verification</p>
<p class="id">{}</p>
<p style="margin: 0 0 24px 0;">Share <strong>age_over_18</strong>?</p>
<button class="accept" onclick="respond('accept')">Accept</button>
<button class="reject" onclick="respond('reject')">Reject</button>
<div id="msg"></div>
</div>
<script>
async function respond(action) {{
    document.getElementById('msg').className = 'info';
    document.getElementById('msg').textContent = 'Processing...';
    try {{
        const r = await fetch('/wallet/' + action + '/{}', {{method:'POST'}});
        if (r.ok) {{
            document.getElementById('msg').className = action === 'accept' ? 'ok' : 'fail';
            document.getElementById('msg').textContent = action === 'accept' ? '✅ Verified — webhook sent' : '❌ Rejected — webhook sent';
        }} else {{
            document.getElementById('msg').className = 'fail';
            document.getElementById('msg').textContent = 'Error: ' + r.statusText;
        }}
    }} catch(e) {{
        document.getElementById('msg').className = 'fail';
        document.getElementById('msg').textContent = 'Error: ' + e.message;
    }}
    document.querySelectorAll('button').forEach(b => b.disabled = true);
}}
</script></body></html>"#, id, id))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        verifications: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/verify/age", post(verify_age))
        .route("/v1/status/{id}", get(get_status))
        .route("/scan/{id}", get(wallet_ui))
        .route("/wallet/accept/{id}", post(simulate_accept))
        .route("/wallet/reject/{id}", post(simulate_reject))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7777")
        .await
        .expect("Failed to bind port 7777");

    println!();
    println!("  PylonID Emulator");
    println!("  ────────────────");
    println!("  API:    http://localhost:7777");
    println!("  Wallet: http://localhost:7777/scan/<id>");
    println!();
    println!("  Endpoints:");
    println!("    POST /v1/verify/age     Start verification");
    println!("    GET  /v1/status/:id     Check status");
    println!("    GET  /health            Health check");
    println!();
    println!("  Ready.");
    println!();

    axum::serve(listener, app).await.expect("Server error");
}
