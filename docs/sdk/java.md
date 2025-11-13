# Java SDK

**Status:** 🔄 Planned. Not yet available.

Official Java SDK for PYLON is under development. Use direct HTTP integration until released.

---

## Current Integration (Direct HTTP)

Until the SDK is available, use Java's standard HTTP client (Java 11+):

```
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.Map;

public class PylonExample {
    public static void main(String[] args) throws Exception {
        String apiKey = System.getenv("PYLON_API_KEY");
        
        // Build request
        Map<String, Object> requestBody = Map.of(
            "policy", Map.of("minAge", 18),
            "callbackUrl", "https://app.example.com/webhooks/pylon"
        );
        
        ObjectMapper mapper = new ObjectMapper();
        String json = mapper.writeValueAsString(requestBody);
        
        HttpClient client = HttpClient.newHttpClient();
        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create("https://pylonid.eu/v1/verify/age"))
            .header("Content-Type", "application/json")
            .header("Authorization", "Bearer " + apiKey)
            .POST(HttpRequest.BodyPublishers.ofString(json))
            .build();
        
        HttpResponse<String> response = client.send(request, 
            HttpResponse.BodyHandlers.ofString());
        
        Map<String, Object> result = mapper.readValue(response.body(), Map.class);
        System.out.println("Verification ID: " + result.get("verificationId"));
        System.out.println("Wallet URL: " + result.get("walletUrl"));
        // Redirect user to wallet URL
    }
}
```

---

## Handle Webhooks (Spring Boot)

```
import org.springframework.web.bind.annotation.*;
import org.springframework.http.ResponseEntity;
import org.springframework.http.HttpStatus;
import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;
import java.nio.charset.StandardCharsets;

@RestController
public class WebhookController {
    
    private boolean validateSignature(String signature, String body, String secret) {
        try {
            String[] parts = signature.split(",");
            if (parts.length != 2) return false;
            
            String t = parts.replace("t=", "");
            String v1 = parts.replace("v1=", "");
            
            String signedMessage = t + "." + body;
            
            Mac mac = Mac.getInstance("HmacSHA256");
            SecretKeySpec secretKey = new SecretKeySpec(
                secret.getBytes(StandardCharsets.UTF_8), 
                "HmacSHA256"
            );
            mac.init(secretKey);
            
            byte[] hash = mac.doFinal(signedMessage.getBytes(StandardCharsets.UTF_8));
            StringBuilder computed = new StringBuilder();
            for (byte b : hash) {
                computed.append(String.format("%02x", b));
            }
            
            return v1.equals(computed.toString());
        } catch (Exception e) {
            return false;
        }
    }
    
    @PostMapping("/webhooks/pylon")
    public ResponseEntity<?> handlePylonWebhook(
        @RequestHeader("X-Pylon-Signature") String signature,
        @RequestBody String body
    ) {
        String secret = System.getenv("PYLON_WEBHOOK_SECRET");
        
        if (!validateSignature(signature, body, secret)) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).build();
        }

        if (body.contains("\"result\":\"verified\"")) {
            System.out.println("✅ Verified!");
            return ResponseEntity.ok(Map.of("received", true));
        }

        return ResponseEntity.ok(Map.of("received", true));
    }
}
```

---

## Idempotency Handling

```
import org.springframework.data.repository.CrudRepository;
import java.time.Instant;

@RestController
public class WebhookController {
    
    @Autowired
    private WebhookRepository webhookRepo;
    
    @PostMapping("/webhooks/pylon")
    public ResponseEntity<?> handlePylonWebhook(
        @RequestHeader("X-Pylon-Idempotency-Key") String idempotencyKey,
        @RequestHeader("X-Pylon-Signature") String signature,
        @RequestBody String body
    ) {
        
        // Check if already processed
        if (webhookRepo.existsById(idempotencyKey)) {
            return ResponseEntity.ok(Map.of("status", "already_processed"));
        }

        // Validate signature
        String secret = System.getenv("PYLON_WEBHOOK_SECRET");
        if (!validateSignature(signature, body, secret)) {
            return ResponseEntity.status(HttpStatus.UNAUTHORIZED).build();
        }

        // Store idempotency key
        WebhookRecord record = new WebhookRecord();
        record.setIdempotencyKey(idempotencyKey);
        record.setProcessedAt(Instant.now());
        webhookRepo.save(record);

        // Return 200 immediately
        ResponseEntity.ok(Map.of("received", true));

        // Process asynchronously
        processWebhookAsync(body);

        return ResponseEntity.ok(Map.of("received", true));
    }
    
    @Async
    private void processWebhookAsync(String body) {
        // Do background work here
    }
}

interface WebhookRepository extends CrudRepository<WebhookRecord, String> {}
```

---

## Error Handling

```
HttpResponse<String> response = client.send(request, 
    HttpResponse.BodyHandlers.ofString());

switch (response.statusCode()) {
    case 401:
        System.err.println("❌ Invalid API key");
        break;
    case 429:
        System.err.println("❌ Rate limited");
        break;
    case 400:
        System.err.println("❌ Invalid request");
        break;
    default:
        System.err.println("❌ Error: " + response.statusCode());
}
```

---

## Testing Locally

Start the local emulator:

```
pylon-cli
```

Point requests to localhost:

```
HttpRequest request = HttpRequest.newBuilder()
    .uri(URI.create("http://localhost:7777/v1/verify/age"))
    .header("Content-Type", "application/json")
    .POST(HttpRequest.BodyPublishers.ofString(json))
    .build();
```

---

## Roadmap

- **Q1 2026:** Official Java SDK release with async-first API using Project Reactor

---

**Questions?** See [Troubleshooting](../8-troubleshooting.md) or [API Reference](../3-api-reference.md)
