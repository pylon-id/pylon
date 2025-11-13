use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::{post, get},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use pylon_core::{verify_age_presentation, create_test_presentation};

// ============================================================================
// DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyAgeRequest {
    policy: AgePolicy,
    #[serde(rename = "callbackUrl")]
    callback_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AgePolicy {
    #[serde(rename = "minAge")]
    min_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyAgeResponse {
    #[serde(rename = "verificationId")]
    #[allow(dead_code)]
    verification_id: String,
    status: String,
    #[serde(rename = "walletUrl")]
    wallet_url: String,
}

#[derive(Debug, Clone)]
struct PendingVerification {
    #[allow(dead_code)]
    verification_id: String,
    callback_url: String,
    min_age: u32,
    status: String,
}

#[derive(Clone)]
struct AppState {
    verifications: Arc<RwLock<HashMap<String, PendingVerification>>>,
}

// ============================================================================
// API ENDPOINTS
// ============================================================================

async fn verify_age(
    State(state): State<AppState>,
    Json(req): Json<VerifyAgeRequest>,
) -> Result<(StatusCode, Json<VerifyAgeResponse>), (StatusCode, String)> {
    println!("📝 Received age verification request");
    
    let verification_id = format!("ver_local_{}", &uuid::Uuid::new_v4().to_string()[..8].to_uppercase());
    let callback_url = req.callback_url.clone();
    let min_age = req.policy.min_age;
    
    println!("  ✅ Verification ID: {}", verification_id);
    println!("  ✅ Min Age: {}", min_age);
    println!("  ✅ Callback: {}", callback_url);
    
    let pending = PendingVerification {
        verification_id: verification_id.clone(),
        callback_url,
        min_age,
        status: "pending".to_string(),
    };
    
    state.verifications.write().await.insert(verification_id.clone(), pending);
    
    let wallet_url = format!("http://localhost:8000/scan/{}", verification_id);
    
    let response = VerifyAgeResponse {
        verification_id,
        status: "pending".to_string(),
        wallet_url,
    };
    
    Ok((StatusCode::OK, Json(response)))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "pylon-emulator",
        "version": "0.1.0"
    }))
}

// ============================================================================
// WEBHOOK FIRING
// ============================================================================

async fn fire_webhook(callback_url: &str, verification_id: &str, result: bool) {
    let payload = json!({
        "verificationId": verification_id,
        "type": "age",
        "result": if result { "verified" } else { "not_verified" },
        "attributes": {
            "ageOver18": result
        },
        "evidence": {
            "issuer": "LOCAL_TEST",
            "credentialType": "SD-JWT VC",
            "proofHash": "sha256:test123...",
            "issuedAt": "2025-01-15T14:30:00Z"
        },
        "audit": {
            "traceId": format!("trace_{}", verification_id)
        }
    });
    
    println!("🪝 Firing webhook to: {}", callback_url);
    println!("   Payload: {}", serde_json::to_string_pretty(&payload).unwrap_or_default());
    
    match reqwest::Client::new()
        .post(callback_url)
        .json(&payload)
        .send()
        .await
    {
        Ok(resp) => {
            println!("   ✅ Webhook fired. Status: {}", resp.status());
        }
        Err(e) => {
            println!("   ❌ Webhook failed: {}", e);
        }
    }
}

// ============================================================================
// FAKE WALLET
// ============================================================================

async fn simulate_wallet_accept(
    State(state): State<AppState>,
    Path(verification_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    println!("\n👤 User clicked [Accept] in fake wallet");
    println!("   Verification ID: {}", verification_id);
    
    let pending = state.verifications.read().await
        .get(&verification_id)
        .cloned()
        .ok_or((StatusCode::NOT_FOUND, "Verification not found".to_string()))?;
    
    let presentation = create_test_presentation(verification_id.clone(), true)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    let result = verify_age_presentation(presentation, pending.min_age, "https://pylonid.eu/pid-issuer")
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    println!("   ✅ Presentation verified: {}", result);
    
    let callback_url = pending.callback_url.clone();
    let vid = verification_id.clone();
    tokio::spawn(async move {
        fire_webhook(&callback_url, &vid, result).await;
    });
    
    state.verifications.write().await
        .entry(verification_id)
        .and_modify(|v| v.status = "completed".to_string());
    
    Ok(StatusCode::OK)
}

async fn fake_wallet_ui(Path(verification_id): Path<String>) -> Html<String> {
    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>PYLON Fake Wallet</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }}
        .wallet {{
            background: white;
            border-radius: 12px;
            padding: 40px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            text-align: center;
            max-width: 400px;
        }}
        h1 {{
            color: #333;
            margin: 0 0 10px 0;
        }}
        p {{
            color: #666;
            margin: 0 0 30px 0;
        }}
        .buttons {{
            display: flex;
            gap: 10px;
        }}
        button {{
            flex: 1;
            padding: 12px 20px;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s;
        }}
        .accept {{
            background: rgb(16, 185, 129);
            color: white;
        }}
        .accept:hover {{
            background: rgb(5, 150, 105);
            transform: scale(1.02);
        }}
        .reject {{
            background: rgb(239, 68, 68);
            color: white;
        }}
        .reject:hover {{
            background: rgb(220, 38, 38);
            transform: scale(1.02);
        }}
        .loading {{
            display: none;
            margin-top: 20px;
            color: rgb(102, 126, 234);
            font-weight: 600;
        }}
        .success {{
            color: rgb(16, 185, 129);
        }}
    </style>
</head>
<body>
    <div class="wallet">
        <h1>🔑 Age Verification</h1>
        <p id="message">Verify that you are at least 18 years old?</p>
        <div class="buttons">
            <button class="accept" onclick="acceptVerification()">✅ Accept</button>
            <button class="reject" onclick="rejectVerification()">❌ Reject</button>
        </div>
        <div class="loading" id="loading">
            Processing... Please wait
        </div>
    </div>
    
    <script>
        const verificationId = "{}";
        
        async function acceptVerification() {{
            document.getElementById("loading").style.display = "block";
            document.querySelectorAll("button").forEach(b => b.disabled = true);
            
            try {{
                const response = await fetch(
                    `http://localhost:7777/webhook/accept/${{verificationId}}`,
                    {{ method: "POST" }}
                );
                
                if (response.ok) {{
                    document.getElementById("message").textContent = "✅ Verified! Webhook sent.";
                    document.getElementById("message").className = "success";
                }} else {{
                    alert("Error: " + response.statusText);
                }}
            }} catch (e) {{
                alert("Error: " + e.message);
            }}
        }}
        
        function rejectVerification() {{
            alert("You rejected the verification.");
            window.history.back();
        }}
    </script>
</body>
</html>"#,
        verification_id
    );
    Html(html)
}

// ============================================================================
// MAIN
// ============================================================================

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let state = AppState {
        verifications: Arc::new(RwLock::new(HashMap::new())),
    };
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/verify/age", post(verify_age))
        .route("/scan/:id", get(fake_wallet_ui))
        .route("/webhook/accept/:id", post(simulate_wallet_accept))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7777")
        .await
        .expect("Failed to bind port 7777");
    
    println!("\n✨ PYLON Emulator Starting...");
    println!("  🌐 Fake API: http://localhost:7777");
    println!("  👤 Fake Wallet: http://localhost:7777/scan/<id>");
    println!("  📝 Ready for testing!\n");
    
    axum::serve(listener, app)
        .await
        .expect("Server error");
}
