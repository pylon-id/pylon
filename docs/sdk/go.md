# Go Integration

Direct HTTP integration with Go's standard library.

---

## Start a Verification

```go
package main

import (
  "bytes"
  "encoding/json"
  "fmt"
  "net/http"
  "os"
)

type VerifyRequest struct {
  Policy      Policy `json:"policy"`
  CallbackURL string `json:"callbackUrl"`
}

type Policy struct {
  MinAge int `json:"minAge"`
}

type VerifyResponse struct {
  VerificationID string `json:"verificationId"`
  Status         string `json:"status"`
  WalletURL      string `json:"walletUrl"`
  RequestURI     string `json:"requestUri"`
  ExpiresAt      string `json:"expiresAt"`
}

func main() {
  body, _ := json.Marshal(VerifyRequest{
    Policy:      Policy{MinAge: 18},
    CallbackURL: "https://yourapp.com/webhooks/pylon",
  })

  req, _ := http.NewRequest("POST", "https://pylonid.eu/v1/verify/age", bytes.NewBuffer(body))
  req.Header.Set("Content-Type", "application/json")
  req.Header.Set("Authorization", "Bearer "+os.Getenv("PYLON_API_KEY"))

  resp, err := http.DefaultClient.Do(req)
  if err != nil {
    panic(err)
  }
  defer resp.Body.Close()

  var result VerifyResponse
  json.NewDecoder(resp.Body).Decode(&result)

  fmt.Printf("Verification: %s\n", result.VerificationID)
  fmt.Printf("Wallet URL:   %s\n", result.WalletURL)
  // Display result.WalletURL as QR code
}
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
)

func validateSignature(body []byte, signature, secret string) bool {
  h := hmac.New(sha256.New, []byte(secret))
  h.Write(body)
  computed := "sha256=" + hex.EncodeToString(h.Sum(nil))
  return hmac.Equal([]byte(signature), []byte(computed))
}

type WebhookPayload struct {
  Event          string                 `json:"event"`
  VerificationID string                 `json:"verificationId"`
  Status         string                 `json:"status"`
  Result         map[string]interface{} `json:"result"`
  Timestamp      string                 `json:"timestamp"`
}

func webhookHandler(w http.ResponseWriter, r *http.Request) {
  signature := r.Header.Get("X-Pylon-Signature")
  body, _ := io.ReadAll(r.Body)
  secret := os.Getenv("PYLON_WEBHOOK_SECRET")

  if !validateSignature(body, signature, secret) {
    http.Error(w, "Invalid signature", http.StatusUnauthorized)
    return
  }

  var payload WebhookPayload
  json.Unmarshal(body, &payload)

  if payload.Status == "verified" {
    // Grant access
    fmt.Printf("✅ %s: verified\n", payload.VerificationID)
  } else {
    fmt.Printf("❌ %s: %s\n", payload.VerificationID, payload.Status)
  }

  w.WriteHeader(http.StatusOK)
  w.Write([]byte(`{"received":true}`))
}

func main() {
  http.HandleFunc("/webhooks/pylon", webhookHandler)
  http.ListenAndServe(":3000", nil)
}
```

---

## Error Handling

```go
resp, err := http.DefaultClient.Do(req)
if err != nil {
  fmt.Fprintf(os.Stderr, "Network error: %v\n", err)
  return
}

switch resp.StatusCode {
case 200:
  // Success
case 401:
  fmt.Fprintln(os.Stderr, "Invalid API key")
case 400:
  fmt.Fprintln(os.Stderr, "Invalid request (check JSON, callbackUrl must be HTTPS)")
case 429:
  fmt.Fprintln(os.Stderr, "Rate limited — back off and retry")
default:
  fmt.Fprintf(os.Stderr, "Unexpected: %d\n", resp.StatusCode)
}
```

---

**Reference:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
