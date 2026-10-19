# Reacher Go SDK

Official Go SDK for the Reacher Email Verification API.

## Installation

```bash
go get github.com/Oppulence-Engineering/check-if-email-exists/sdks/golang/v4
```

## Quick Start

```go
package main

import (
    "context"
    "fmt"
    "log"

    reacher "github.com/Oppulence-Engineering/check-if-email-exists/sdks/golang/v4"
)

func main() {
    // Initialize configuration
    cfg := reacher.NewConfiguration()
    cfg.AddDefaultHeader("Authorization", "YOUR_API_KEY")

    // Create API client
    client := reacher.NewAPIClient(cfg)
    ctx := context.Background()

    // Verify a single email
    request := reacher.CheckEmailRequest{
        ToEmail: "test@example.com",
    }

    result, resp, err := client.DefaultApi.PostV1CheckEmail(ctx).
        CheckEmailRequest(request).
        Execute()

    if err != nil {
        log.Fatalf("Error: %v", err)
    }

    fmt.Printf("Response Status: %d\n", resp.StatusCode)
    fmt.Printf("Is Reachable: %s\n", result.GetIsReachable())
    fmt.Printf("Is Valid Syntax: %t\n", result.Syntax.GetIsValidSyntax())
}
```

## Bulk Email Verification

```go
package main

import (
    "context"
    "fmt"
    "log"
    "time"

    reacher "github.com/Oppulence-Engineering/check-if-email-exists/sdks/golang/v4"
)

func main() {
    cfg := reacher.NewConfiguration()
    cfg.AddDefaultHeader("Authorization", "YOUR_API_KEY")
    client := reacher.NewAPIClient(cfg)
    ctx := context.Background()

    // Start bulk verification
    emails := []string{"email1@example.com", "email2@example.com"}
    bulkRequest := reacher.PostV1BulkRequest{
        Input: emails,
    }

    bulkResult, _, err := client.DefaultApi.PostV1Bulk(ctx).
        PostV1BulkRequest(bulkRequest).
        Execute()

    if err != nil {
        log.Fatalf("Failed to start bulk job: %v", err)
    }

    jobId := bulkResult.GetJobId()
    fmt.Printf("Bulk job started: %d\n", jobId)

    // Poll for completion
    for {
        progress, _, err := client.DefaultApi.GetV1Bulk(ctx, int32(jobId)).Execute()
        if err != nil {
            log.Fatalf("Failed to get progress: %v", err)
        }

        fmt.Printf("Progress: %d/%d\n",
            progress.GetTotalProcessed(),
            progress.GetTotalRecords())

        if progress.GetJobStatus() == "Completed" {
            break
        }
        time.Sleep(2 * time.Second)
    }

    // Get results
    results, _, err := client.DefaultApi.GetV1BulkResults(ctx, fmt.Sprintf("%d", jobId)).Execute()
    if err != nil {
        log.Fatalf("Failed to get results: %v", err)
    }

    fmt.Printf("Results: %+v\n", results)
}
```

## Configuration

```go
cfg := reacher.NewConfiguration()
cfg.Servers = reacher.ServerConfigurations{
    {
        URL: "https://api.reacher.email",
        Description: "Reacher Production",
    },
}
cfg.AddDefaultHeader("Authorization", "YOUR_API_KEY")

// Optional: Configure HTTP client
cfg.HTTPClient = &http.Client{
    Timeout: 30 * time.Second,
}
```

## Error Handling

```go
result, resp, err := client.DefaultApi.PostV1CheckEmail(ctx).
    CheckEmailRequest(request).
    Execute()

if err != nil {
    // Check for API error
    if apiErr, ok := err.(*reacher.GenericOpenAPIError); ok {
        fmt.Printf("API Error: %s\n", apiErr.Body())
    }

    // Check HTTP status
    if resp != nil {
        fmt.Printf("HTTP Status: %d\n", resp.StatusCode)
    }
}
```

## License

MIT
