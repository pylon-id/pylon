# Java Integration

Direct HTTP integration with Java's built-in HTTP client (Java 11+).

---

## Start a Verification

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;

public class PylonExample {
    public static void main(String[] args) throws Exception {
        String apiKey = System.getenv("PYLON_API_KEY");

        String json = """
            {
                "policy": { "minAge": 18 },
                "callbackUrl": "https://yourapp.com/webhooks/pylon"
            }
            """;

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://pylonid.eu/v1/verify/age"))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer " + apiKey)
            .POST(HttpRequest.BodyPublishers.ofString(json))
            .build();

        HttpResponse<String> response = HttpClient.newHttpClient()
            .send(request, HttpResponse.BodyHandlers.ofString());

        System.out.println(response.body());
        // Parse JSON, display walletUrl as QR code
    }
}
```

---

## Handle Webhooks (Spring Boot)

```java
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.nio.charset.StandardCharsets;
import java.util.Map;

@RestController
public class WebhookController {

    private boolean validateSignature(String body, String signature, String secret) {
        try {
            Mac mac = Mac.getInstance("HmacSHA256");
            mac.init(new SecretKeySpec(
                secret.getBytes(StandardCharsets.UTF_8), "HmacSHA256"));
            byte[] hash = mac.doFinal(body.getBytes(StandardCharsets.UTF_8));

            StringBuilder sb = new StringBuilder("sha256=");
            for (byte b : hash) sb.append(String.format("%02x", b));

            return sb.toString().equals(signature);
        } catch (Exception e) {
            return false;
        }
    }

    @PostMapping("/webhooks/pylon")
    public ResponseEntity<?> handleWebhook(
        @RequestHeader("X-Pylon-Signature") String signature,
        @RequestBody String body
    ) {
        String secret = System.getenv("PYLON_WEBHOOK_SECRET");

        if (!validateSignature(body, signature, secret)) {
            return ResponseEntity.status(401).build();
        }

        // Parse body and process
        System.out.println("Webhook received: " + body);
        return ResponseEntity.ok(Map.of("received", true));
    }
}
```

---

## Error Handling

```java
HttpResponse<String> response = HttpClient.newHttpClient()
    .send(request, HttpResponse.BodyHandlers.ofString());

switch (response.statusCode()) {
    case 200 -> System.out.println("Success: " + response.body());
    case 401 -> System.err.println("Invalid API key");
    case 400 -> System.err.println("Invalid request");
    case 429 -> System.err.println("Rate limited — back off and retry");
    default  -> System.err.println("Error: " + response.statusCode());
}
```

---

**Reference:** [API Reference](../3-api-reference.md) | [Webhooks](../6-webhooks.md) | [Troubleshooting](../8-troubleshooting.md)
