# Go SDK

**Status:** 🔄 Planned. Not yet available.

Official Go SDK for PYLON is under development. Use direct HTTP integration until released.

---

## Current Integration (Direct HTTP)

Until the SDK is available, use Go's standard HTTP client:

```go
package main

import (
  "bytes"
  "encoding/json"
  "fmt"
  "net/http"
  "os"
)

type VerifyAgeRequest struct {{
  Policy      AgePolicy `json:"policy"`
  CallbackURL string    `json:"callbackUrl"`
}}

type AgePolicy struct {{
  MinAge int `json:"minAge"`
}}

type VerifyAgeResponse struct {{
  VerificationID string `json:"verificationId"`
  Status         string `json:"status"`
  WalletURL      string `json:"walletUrl"`
}}

func main() {{
  req := VerifyAgeRequest{{
    Policy:      AgePolicy{{MinAge: 18}},
    CallbackURL: "https://app.example.com/webhooks/pylon",
  }}

  body, _ := json.Marshal(req)
  httpReq, _ := http.NewRequest("POST", "{BASE_URL}/v1/verify/age", bytes.NewBuffer(body))
  httpReq.Header.Set("Content-Type", "application/json")
  httpReq.Header.Set("Authorization", "Bearer "+os.Getenv("PYLON_API_KEY"))

  client := &http.Client{{}}
  resp, err := client.Do(httpReq)
  if err != nil {{
    panic(err)
  }}
  defer resp.Body.Close()

  var result VerifyAgeResponse
  json.NewDecoder(resp.Body).Decode(&result)

  fmt.Printf("Verification ID: %s\\n", result.VerificationID)
  fmt.Printf("Wallet URL: %s\\n", result.WalletURL)
  // Redirect user to result.WalletURL
}}
```

---

## Handle Webhooks

```go
package main

import (
  "crypto/hmac"
  "crypto/sha256"
  "encoding/hex"
  "encoding/json"
  "io"
  "net/http"
  "os"
  "strings"
)

type WebhookResult struct {{
  VerificationID string `json:"verificationId"`
  Type           string `json:"type"`
  Result         string `json:"result"`
}}

func validateSignature(signature, body, secret string) bool {{
  parts := strings.Split(signature, ",")
  if len(parts) != 2 {{
    return false
  }}

  t := strings.TrimPrefix(parts[0], "t=")
  v1 := strings.TrimPrefix(parts[1], "v1=")

  signedMessage := t + "." + body
  h := hmac.New(sha256.New, []byte(secret))
  h.Write([]byte(signedMessage))
  computed := hex.EncodeToString(h.Sum(nil))

  return hmac.Equal([]byte(v1), []byte(computed))
}}

func webhookHandler(w http.ResponseWriter, r *http.Request) {{
  signature := r.Header.Get("X-Pylon-Signature")
  body, _ := io.ReadAll(r.Body)
  secret := os.Getenv("PYLON_WEBHOOK_SECRET")

  if !validateSignature(signature, string(body), secret) {{
    w.WriteHeader(http.StatusUnauthorized)
    w.Write([]byte("Invalid signature"))
    return
  }}

  var result WebhookResult
  json.Unmarshal(body, &result)

  if result.Result == "verified" {{
    // Grant access
    w.WriteHeader(http.StatusOK)
    w.Write([]byte(`{{"received":true}}`))
  }} else {{
    w.WriteHeader(http.StatusOK)
    w.Write([]byte(`{{"received":true}}`))
  }}
}}

func main() {{
  http.HandleFunc("/webhooks/pylon", webhookHandler)
  http.ListenAndServe(":3000", nil)
}}
```

---

## Idempotency Handling

```go
func webhookHandler(w http.ResponseWriter, r *http.Request) {{
  idempotencyKey := r.Header.Get("X-Pylon-Idempotency-Key")

  // Check if already processed
  if alreadyProcessed(idempotencyKey) {{
    w.WriteHeader(http.StatusOK)
    w.Write([]byte(`{{"status":"already_processed"}}`))
    return
  }}

  // Validate signature
  signature := r.Header.Get("X-Pylon-Signature")
  body, _ := io.ReadAll(r.Body)
  secret := os.Getenv("PYLON_WEBHOOK_SECRET")

  if !validateSignature(signature, string(body), secret) {{
    w.WriteHeader(http.StatusUnauthorized)
    return
  }}

  // Store idempotency key
  storeIdempotencyKey(idempotencyKey)

  // Process webhook
  var result WebhookResult
  json.Unmarshal(body, &result)

  w.WriteHeader(http.StatusOK)
  w.Write([]byte(`{{"received":true}}`))

  // Process asynchronously
  go processWebhook(result)
}}
```

---

## Error Handling

```go
resp, err := client.Do(httpReq)
if err != nil {{
  panic(err)
}}

switch resp.StatusCode {{
case 401:
  fmt.Println("❌ Invalid API key")
case 429:
  fmt.Println("❌ Rate limited")
case 400:
  fmt.Println("❌ Invalid request")
default:
  fmt.Printf("❌ Error: %d\\n", resp.StatusCode)
}}
```

---

## Testing Locally

Start the local emulator:

```bash
pylon-cli
```

Point requests to localhost:

```go
httpReq, _ := http.NewRequest("POST", "http://localhost:7777/v1/verify/age", bytes.NewBuffer(body))
```

---

## Roadmap

- **Q1 2026:** Official Go SDK release with type-safe client

---

**Questions?** See [Troubleshooting](../8-troubleshooting.md) or [API Reference](../3-api-reference.md)
