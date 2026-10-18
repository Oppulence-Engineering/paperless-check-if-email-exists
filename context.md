# Codebase Context: check-if-email-exists

> **Purpose**: Comprehensive technical reference for engineers and LLMs working with this codebase.
> **Last Updated**: 2025-12-25
> **Version**: 3.0.0

---

## 1. Executive Summary

### What This System Does
**check-if-email-exists** (also known as **Reacher**) is an email verification service that determines whether an email address exists and is deliverable without actually sending an email. It performs multi-layered verification including syntax validation, DNS/MX record lookup, SMTP server probing, and provider-specific checks.

### Primary Business Value
- **Email List Hygiene**: Validates email lists before marketing campaigns to reduce bounce rates
- **Fraud Prevention**: Detects disposable/temporary email addresses at signup
- **Deliverability Optimization**: Identifies risky addresses (catch-all, full inbox, role accounts)
- **Cost Reduction**: Prevents sending emails to non-existent addresses

### Key Capabilities
| Capability | Description |
|------------|-------------|
| Syntax Validation | RFC-compliant email address parsing |
| MX Lookup | DNS resolution for mail exchange records |
| SMTP Verification | Direct mailbox existence probing via SMTP protocol |
| Headless Browser | JavaScript-based verification for Yahoo/Hotmail |
| API Verification | Provider-specific API calls (Yahoo) |
| Catch-All Detection | Identifies domains accepting all addresses |
| Disposable Detection | Flags temporary email providers |
| Role Account Detection | Identifies generic addresses (info@, admin@) |
| Gravatar Integration | Optional profile existence check |
| HaveIBeenPwned | Optional breach detection |

### Tech Stack Overview
- **Language**: Rust (2021 edition)
- **Async Runtime**: Tokio
- **HTTP Server**: Warp
- **Message Queue**: RabbitMQ (lapin)
- **Database**: PostgreSQL (sqlx)
- **Headless Browser**: ChromeDriver via fantoccini
- **TLS**: rustls with ring crypto provider
- **Serverless**: AWS Lambda + SQS (alternative deployment)

### Licensing
- **MIT** (see `LICENSE.md`). Upstream Reacher moved from AGPL-3.0 to MIT in 0.11.8.

---

## 2. Repo Map

### Directory Structure
```
check-if-email-exists/
├── backend/                    # HTTP API server and worker
│   ├── src/
│   │   ├── main.rs            # Entry point, server + worker bootstrap
│   │   ├── lib.rs             # Library exports
│   │   ├── config.rs          # Configuration system (549 LOC)
│   │   ├── throttle.rs        # Rate limiting logic
│   │   ├── http/              # Warp HTTP routes
│   │   │   ├── mod.rs         # Route definitions (v0, v1 API)
│   │   │   ├── check_email.rs # Single email verification endpoint
│   │   │   └── bulk.rs        # Bulk job endpoints
│   │   ├── storage/           # Persistence layer
│   │   │   ├── mod.rs         # Storage trait definition
│   │   │   ├── postgres.rs    # PostgreSQL adapter
│   │   │   └── commercial_license_trial.rs
│   │   └── worker/            # Background job processing
│   │       ├── mod.rs         # Worker module
│   │       ├── consume.rs     # RabbitMQ consumer
│   │       └── do_work.rs     # Task execution logic
│   ├── migrations/            # PostgreSQL schema migrations
│   │   ├── 20230505000000_init.sql
│   │   ├── 20240525000000_add_job_id.sql
│   │   ├── 20240622000000_add_request_column.sql
│   │   ├── 20240929000000_add_backend_name.sql
│   │   └── 20241102174000_bulk_jobs.sql
│   ├── backend_config.toml    # Default configuration template
│   └── Cargo.toml             # Backend crate manifest
│
├── core/                       # Core verification library
│   ├── src/
│   │   ├── lib.rs             # Main check_email() function
│   │   ├── syntax.rs          # Email address parsing
│   │   ├── mx.rs              # DNS MX record lookup
│   │   ├── rules.rs           # Provider-specific rules
│   │   ├── rules.json         # Rule definitions
│   │   ├── haveibeenpwned.rs  # Breach check integration
│   │   ├── smtp/              # SMTP verification
│   │   │   ├── mod.rs         # SMTP entry point
│   │   │   ├── connect.rs     # TCP/TLS connection
│   │   │   ├── error.rs       # Error types
│   │   │   ├── gmail.rs       # Gmail-specific handling
│   │   │   ├── yahoo.rs       # Yahoo headless/API
│   │   │   ├── outlook.rs     # Hotmail/Outlook handling
│   │   │   ├── headless.rs    # Browser automation base
│   │   │   ├── http_api.rs    # HTTP API verification
│   │   │   ├── parser.rs      # SMTP response parsing
│   │   │   └── verif_method.rs # Verification method configs
│   │   ├── misc/              # Miscellaneous checks
│   │   │   └── mod.rs         # Disposable, role, gravatar
│   │   └── util/              # Utilities
│   │       ├── input_output.rs # Input/Output types
│   │       └── sentry.rs      # Error reporting
│   └── Cargo.toml             # Core crate manifest
│
├── cli/                        # Command-line interface
│   ├── src/
│   │   └── main.rs            # CLI entry point
│   └── Cargo.toml             # CLI crate manifest
│
├── sqs/                        # AWS Lambda handler
│   ├── src/
│   │   └── main.rs            # Lambda function handler
│   └── Cargo.toml             # SQS crate manifest
│
├── helm/                       # Kubernetes Helm charts
│   ├── Chart.yaml
│   ├── values.yaml
│   └── templates/
│
├── .github/                    # CI/CD workflows
│   └── workflows/
│       ├── pr.yml             # Pull request checks
│       ├── release.yml        # Release automation
│       ├── check_json.yml     # JSON validation
│       ├── discord.yml        # Discord notifications
│       └── sslyze.yml         # SSL/TLS testing
│
├── Cargo.toml                  # Workspace manifest
├── Cargo.lock                  # Dependency lock file
├── README.md                   # Project documentation
├── Makefile                    # Build shortcuts
├── .env.example                # Environment variable template
└── LICENSE.md                  # MIT license
```

### Crate Dependency Graph
```
┌─────────────┐
│    cli      │──────────────────────────────────┐
└─────────────┘                                  │
                                                 ▼
┌─────────────┐     ┌─────────────┐     ┌──────────────┐
│    sqs      │────▶│   backend   │────▶│     core     │
└─────────────┘     └─────────────┘     └──────────────┘
```

### Key Files by Importance
| File | LOC | Purpose |
|------|-----|---------|
| `core/src/lib.rs` | ~280 | Main verification orchestration |
| `backend/src/config.rs` | ~550 | Configuration system |
| `backend/src/worker/consume.rs` | ~200 | RabbitMQ consumer |
| `core/src/smtp/mod.rs` | ~235 | SMTP verification routing |
| `backend/src/http/mod.rs` | ~150 | HTTP API routes |
| `core/src/smtp/verif_method.rs` | ~300 | Provider verification configs |

---

## 3. Architecture Overview

### System Architecture
```
                                    ┌─────────────────┐
                                    │   Client Apps   │
                                    └────────┬────────┘
                                             │ HTTPS
                                             ▼
┌────────────────────────────────────────────────────────────────┐
│                        BACKEND SERVICE                          │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                      Warp HTTP Server                     │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐   │  │
│  │  │  /v0/check  │  │  /v1/check  │  │   /v1/bulk/*    │   │  │
│  │  │   _email    │  │   _email    │  │                 │   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘   │  │
│  └─────────┼────────────────┼──────────────────┼────────────┘  │
│            │                │                  │                │
│            ▼                ▼                  ▼                │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                   Throttle Manager                       │   │
│  │    (per-second, per-minute, per-hour, per-day limits)   │   │
│  └─────────────────────────────┬───────────────────────────┘   │
│                                │                                │
│            ┌───────────────────┼───────────────────┐           │
│            ▼                   ▼                   ▼           │
│  ┌─────────────────┐  ┌───────────────┐  ┌─────────────────┐  │
│  │  Direct Check   │  │   RabbitMQ    │  │   PostgreSQL    │  │
│  │   (sync v0)     │  │    Queue      │  │    Storage      │  │
│  └────────┬────────┘  └───────┬───────┘  └────────┬────────┘  │
│           │                   │                    │           │
└───────────┼───────────────────┼────────────────────┼───────────┘
            │                   │                    │
            ▼                   ▼                    │
┌───────────────────────────────────────────────┐   │
│                 WORKER POOL                    │   │
│  ┌──────────────────────────────────────────┐ │   │
│  │          RabbitMQ Consumer               │ │   │
│  │  - Prefetch: 1 (configurable)            │ │   │
│  │  - Priority queue (0-5)                  │ │   │
│  │  - Auto-requeue on failure               │ │   │
│  └─────────────────┬────────────────────────┘ │   │
│                    ▼                          │   │
│  ┌──────────────────────────────────────────┐ │   │
│  │        check_email_and_send_result       │ │◀──┘
│  │  - Executes core verification            │ │
│  │  - Stores result in PostgreSQL           │ │
│  │  - Sends webhook if configured           │ │
│  │  - Requeues Unknown results (once)       │ │
│  └──────────────────────────────────────────┘ │
└───────────────────────────────────────────────┘
            │
            ▼
┌───────────────────────────────────────────────────────────────┐
│                        CORE LIBRARY                            │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │                     check_email()                        │  │
│  │  1. Syntax Check ─▶ 2. MX Lookup ─▶ 3. SMTP ─▶ 4. Misc  │  │
│  └─────────────────────────────────────────────────────────┘  │
│                                                                │
│  ┌───────────────┐  ┌───────────────┐  ┌──────────────────┐  │
│  │ SMTP Direct   │  │   Headless    │  │    HTTP API      │  │
│  │ (most emails) │  │ (Yahoo/MSFT)  │  │    (Yahoo)       │  │
│  └───────┬───────┘  └───────┬───────┘  └────────┬─────────┘  │
│          │                  │                    │            │
│          ▼                  ▼                    ▼            │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │              External Services                           │ │
│  │  - Target SMTP servers (port 25/465/587)                │ │
│  │  - ChromeDriver (headless browser)                      │ │
│  │  - Yahoo API endpoints                                  │ │
│  │  - SOCKS5 proxies (optional)                            │ │
│  └─────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────┘
```

### Serverless Architecture (AWS Lambda + SQS)
```
┌──────────────┐     ┌──────────────┐     ┌──────────────────┐
│  SQS Queue   │────▶│ Lambda Func  │────▶│   ChromeDriver   │
│              │     │  (reacher_   │     │   (headless)     │
│              │     │    sqs)      │     └──────────────────┘
└──────────────┘     └──────┬───────┘
                           │
                           ▼
                    ┌──────────────┐
                    │    Core      │
                    │  check_email │
                    └──────┬───────┘
                           │
              ┌────────────┴────────────┐
              ▼                         ▼
       ┌──────────────┐         ┌──────────────┐
       │   Storage    │         │   Webhook    │
       │  (optional)  │         │  (optional)  │
       └──────────────┘         └──────────────┘
```

### Module Boundaries
| Module | Responsibility | Dependencies |
|--------|----------------|--------------|
| `core` | Email verification logic | External: SMTP servers, DNS, WebDriver |
| `backend` | HTTP API, worker, storage | Internal: `core` |
| `cli` | Command-line interface | Internal: `core` |
| `sqs` | AWS Lambda handler | Internal: `core`, `backend` |

---

## 4. Core Flows

### Flow 1: Single Email Verification (Synchronous - v0 API)
```
Client                 Backend                    Core                    External
  │                      │                         │                         │
  │ POST /v0/check_email │                         │                         │
  │─────────────────────▶│                         │                         │
  │                      │                         │                         │
  │                      │ throttle.check_limit()  │                         │
  │                      │────────────────────────▶│                         │
  │                      │◀────────────────────────│                         │
  │                      │                         │                         │
  │                      │ check_email()           │                         │
  │                      │────────────────────────▶│                         │
  │                      │                         │                         │
  │                      │                         │ check_syntax()          │
  │                      │                         │─────────────────────────│
  │                      │                         │                         │
  │                      │                         │ check_mx() ───────────▶│DNS
  │                      │                         │◀───────────────────────│
  │                      │                         │                         │
  │                      │                         │ check_smtp() ─────────▶│SMTP
  │                      │                         │◀───────────────────────│
  │                      │                         │                         │
  │                      │                         │ check_misc()            │
  │                      │                         │─────────────────────────│
  │                      │                         │                         │
  │                      │◀────────────────────────│                         │
  │ CheckEmailOutput     │                         │                         │
  │◀─────────────────────│                         │                         │
```

### Flow 2: Bulk Email Verification (Asynchronous - v1 API)
```
Client                 Backend                 RabbitMQ              Worker
  │                      │                       │                     │
  │ POST /v1/bulk        │                       │                     │
  │─────────────────────▶│                       │                     │
  │                      │                       │                     │
  │                      │ create bulk_job       │                     │
  │                      │ (PostgreSQL)          │                     │
  │                      │                       │                     │
  │ { job_id: "..." }    │                       │                     │
  │◀─────────────────────│                       │                     │
  │                      │                       │                     │
  │ POST /v1/bulk/{id}   │                       │                     │
  │  (emails array)      │                       │                     │
  │─────────────────────▶│                       │                     │
  │                      │                       │                     │
  │                      │ publish tasks ───────▶│                     │
  │                      │ (one per email)       │                     │
  │                      │                       │                     │
  │ { queued: N }        │                       │                     │
  │◀─────────────────────│                       │                     │
  │                      │                       │                     │
  │                      │                       │ consume task ──────▶│
  │                      │                       │                     │
  │                      │                       │         check_email()
  │                      │                       │                     │
  │                      │                       │ ack/nack ◀──────────│
  │                      │                       │                     │
  │                      │                store result (PostgreSQL) ◀──│
  │                      │                       │                     │
  │                      │                       │       webhook ─────▶│ Client
  │                      │                       │                     │
  │ GET /v1/bulk/{id}/   │                       │                     │
  │     results          │                       │                     │
  │─────────────────────▶│                       │                     │
  │                      │                       │                     │
  │ { results: [...] }   │                       │                     │
  │◀─────────────────────│                       │                     │
```

### Flow 3: SMTP Verification Decision Tree
```
                          check_smtp()
                              │
                              ▼
                    ┌─────────────────┐
                    │ Detect Provider │
                    │ from MX host    │
                    └────────┬────────┘
                             │
        ┌────────────────────┼────────────────────┐
        ▼                    ▼                    ▼
   ┌─────────┐          ┌─────────┐          ┌─────────┐
   │ Gmail   │          │ Yahoo   │          │ Hotmail │
   │         │          │         │          │  B2C    │
   └────┬────┘          └────┬────┘          └────┬────┘
        │                    │                    │
        ▼                    ▼                    ▼
   ┌─────────┐     ┌─────────────────┐     ┌─────────────────┐
   │  SMTP   │     │  API/Headless/  │     │ Headless/SMTP   │
   │ Direct  │     │     SMTP        │     │                 │
   └─────────┘     └─────────────────┘     └─────────────────┘
        │                    │                    │
        └────────────────────┼────────────────────┘
                             ▼
                    ┌─────────────────┐
                    │  SmtpDetails    │
                    │ - is_deliverable│
                    │ - is_catch_all  │
                    │ - has_full_inbox│
                    │ - is_disabled   │
                    └─────────────────┘
```

### Flow 4: Throttling Decision Flow
```
                    Incoming Request
                          │
                          ▼
              ┌───────────────────────┐
              │   Check per-second    │
              │   limit (if set)      │
              └───────────┬───────────┘
                          │
               ┌──────────┴──────────┐
               │                     │
           exceeded              within limit
               │                     │
               ▼                     ▼
         ┌─────────┐      ┌───────────────────┐
         │ Reject  │      │ Check per-minute  │
         │ 429     │      │ limit (if set)    │
         └─────────┘      └─────────┬─────────┘
                                    │
                         ... (repeat for hour/day) ...
                                    │
                                    ▼
                          ┌─────────────────┐
                          │ Process Request │
                          └─────────────────┘
```

---

## 5. Domain + Data Model

### Core Domain Types

#### CheckEmailInput
```rust
pub struct CheckEmailInput {
    /// The email address to verify
    pub to_email: String,

    /// Verification method configuration per provider
    pub verif_method: VerifMethod,

    /// WebDriver address for headless verification
    pub webdriver_addr: String,

    /// WebDriver browser configuration
    pub webdriver_config: WebDriverConfig,

    /// Whether to check Gravatar
    pub check_gravatar: bool,

    /// HaveIBeenPwned API key
    pub haveibeenpwned_api_key: Option<String>,

    /// Backend instance name for debugging
    pub backend_name: Option<String>,
}
```

#### CheckEmailOutput
```rust
pub struct CheckEmailOutput {
    /// Original input email
    pub input: String,

    /// Confidence level of reachability
    pub is_reachable: Reachable,  // Safe | Risky | Invalid | Unknown

    /// Miscellaneous checks result
    pub misc: Result<MiscDetails, MiscError>,

    /// MX record lookup result
    pub mx: Result<MxDetails, MxError>,

    /// SMTP verification result
    pub smtp: Result<SmtpDetails, SmtpError>,

    /// Syntax validation result
    pub syntax: SyntaxDetails,

    /// Debug information
    pub debug: DebugDetails,
}
```

#### SmtpDetails
```rust
pub struct SmtpDetails {
    pub can_connect_smtp: bool,   // Could we connect to SMTP server?
    pub has_full_inbox: bool,     // Is mailbox full?
    pub is_catch_all: bool,       // Does domain accept all addresses?
    pub is_deliverable: bool,     // Can we deliver to this address?
    pub is_disabled: bool,        // Is account disabled?
}
```

#### Reachable Enum
```rust
pub enum Reachable {
    Safe,     // High confidence deliverable
    Risky,    // Deliverable but risky (catch-all, disposable, role)
    Invalid,  // Not deliverable
    Unknown,  // Could not determine
}
```

### Database Schema

#### Table: `v1_task_result`
```sql
CREATE TABLE v1_task_result (
    id UUID PRIMARY KEY DEFAULT uuid_nil(),

    -- Job identification
    job_id UUID NOT NULL,           -- References bulk job or single-shot ID
    job_type TEXT NOT NULL,         -- 'bulk' or 'singleshot'

    -- Input/Output
    email TEXT NOT NULL,            -- Email being verified
    result JSONB NOT NULL,          -- CheckEmailOutput as JSON
    request JSONB,                  -- Original CheckEmailRequest

    -- Metadata
    backend_name TEXT,              -- Instance that processed this
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexing
    CONSTRAINT unique_job_email UNIQUE (job_id, email)
);
```

#### Table: `bulk_job` (Bulk Job Tracking)
```sql
CREATE TABLE bulk_job (
    id UUID PRIMARY KEY,
    total_emails INT NOT NULL,
    completed_emails INT DEFAULT 0,
    failed_emails INT DEFAULT 0,
    status TEXT DEFAULT 'pending',  -- pending, running, completed, failed
    webhook_url TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Message Queue Schema

#### RabbitMQ Queue: `check_email`
```json
{
  "queue_name": "check_email",
  "properties": {
    "durable": true,
    "arguments": {
      "x-max-priority": 5
    }
  }
}
```

#### Task Message Format
```json
{
  "input": {
    "to_email": "test@example.com",
    "from_email": "verify@reacher.email",
    "hello_name": "reacher.email",
    "smtp_port": 25,
    "proxy": null
  },
  "job_id": {
    "type": "Bulk",
    "id": "550e8400-e29b-41d4-a716-446655440000"
  },
  "webhook": {
    "url": "https://webhook.example.com/results",
    "extra": { "custom": "data" }
  }
}
```

### Provider Detection Logic
```rust
pub enum EmailProvider {
    Gmail,          // *google.com, *googlemail.com
    Yahoo,          // *yahoodns.net, *yahoo.com
    HotmailB2C,     // *outlook.com (consumer)
    HotmailB2B,     // *outlook.com (business/O365)
    Mimecast,       // *mimecast.com
    Proofpoint,     // *pphosted.com
    EverythingElse, // Default fallback
}
```

---

## 6. API & Contracts

### HTTP Endpoints

#### Health & Info
| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Returns `OK` (health check) |
| GET | `/version` | Returns `{ "version": "x.y.z" }` |

#### V0 API (Synchronous)
| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/v0/check_email` | Header | Verify single email synchronously |

#### V1 API (Asynchronous)
| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | `/v1/check_email` | Header | Queue single email for verification |
| POST | `/v1/bulk` | Header | Create new bulk verification job |
| GET | `/v1/bulk/{job_id}` | Header | Get bulk job status |
| POST | `/v1/bulk/{job_id}` | Header | Add emails to bulk job |
| GET | `/v1/bulk/{job_id}/results` | Header | Get bulk job results |

### Request/Response Formats

#### POST /v0/check_email
**Request:**
```json
{
  "to_email": "someone@example.com",
  "from_email": "verify@reacher.email",
  "hello_name": "reacher.email",
  "proxy": {
    "host": "proxy.example.com",
    "port": 1080,
    "username": "user",
    "password": "pass"
  },
  "smtp_port": 25,
  "smtp_timeout": { "secs": 30, "nanos": 0 },
  "smtp_security": "Opportunistic"
}
```

**Response (200 OK):**
```json
{
  "input": "someone@example.com",
  "is_reachable": "safe",
  "misc": {
    "is_disposable": false,
    "is_role_account": false,
    "gravatar_url": null,
    "haveibeenpwned": null
  },
  "mx": {
    "accepts_mail": true,
    "records": ["mx1.example.com", "mx2.example.com"]
  },
  "smtp": {
    "can_connect_smtp": true,
    "has_full_inbox": false,
    "is_catch_all": false,
    "is_deliverable": true,
    "is_disabled": false
  },
  "syntax": {
    "address": "someone@example.com",
    "domain": "example.com",
    "is_valid_syntax": true,
    "username": "someone",
    "suggestion": null
  },
  "debug": {
    "start_time": "2025-01-01T12:00:00Z",
    "end_time": "2025-01-01T12:00:05Z",
    "duration": { "secs": 5, "nanos": 123456789 },
    "smtp": {
      "verif_method": {
        "type": "Smtp",
        "host": "mx1.example.com",
        "verif_method": { ... }
      }
    }
  }
}
```

#### POST /v1/bulk
**Request:**
```json
{
  "webhook": {
    "url": "https://webhook.example.com/callback",
    "extra": { "campaign_id": "abc123" }
  }
}
```

**Response (201 Created):**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Error Responses
```json
{
  "error": "rate_limit_exceeded",
  "message": "Too many requests. Retry after 60 seconds.",
  "retry_after": 60
}
```

### Webhook Payload
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "someone@example.com",
  "result": { /* CheckEmailOutput */ },
  "extra": { "campaign_id": "abc123" }
}
```

---

## 7. Auth / Permissions / Security

### Authentication Methods

#### Header-Based Secret
```
x-reacher-secret: your-secret-key
```
Configured via `RCH__HTTP__HEADER_SECRET` environment variable.

#### Backend Name Tracking
The `RCH__BACKEND_NAME` is logged with each verification for audit purposes.

### Security Considerations

#### SMTP Connection Security
```rust
pub enum SmtpSecurity {
    None,           // No encryption (port 25)
    Opportunistic,  // STARTTLS if available
    Wrapper,        // Implicit TLS (port 465)
}
```

#### Proxy Support
- SOCKS5 proxies for SMTP connections
- Per-provider proxy configuration
- Timeout configuration per proxy

#### Rate Limiting
- Configurable limits per time window
- Applied before processing to prevent abuse
- Returns HTTP 429 when exceeded

### Sensitive Data Handling
| Data Type | Handling |
|-----------|----------|
| Email addresses | Stored in database, logged (hashed in production recommended) |
| SMTP credentials | Not stored, passed per-request |
| API secrets | Environment variables only |
| Proxy credentials | Configuration only, not logged |

### TLS Configuration
- Uses `rustls` with `ring` crypto provider
- Provider initialized once at startup
- Supports modern TLS versions only

---

## 8. Integrations

### External Services

#### RabbitMQ (lapin)
```toml
[dependencies]
lapin = "2.5.0"
```
**Configuration:**
```toml
[worker.rabbitmq]
url = "amqp://guest:guest@localhost:5672"
concurrency = 5
```

#### PostgreSQL (sqlx)
```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
```
**Configuration:**
```toml
[storage.postgres]
db_url = "postgres://user:pass@localhost:5432/reacher"
```

#### Sentry (Error Tracking)
```toml
[dependencies]
sentry = "0.35"
```
**Configuration:**
```
SENTRY_DSN=https://key@sentry.io/project
```

#### ChromeDriver (Headless Browser)
**Configuration:**
```toml
[webdriver]
addr = "http://localhost:9515"
```

### SDK/Client Libraries
The project provides a Rust library that can be used directly:
```rust
use check_if_email_exists::{check_email, CheckEmailInputBuilder};

let input = CheckEmailInputBuilder::default()
    .to_email("test@example.com".into())
    .build()
    .unwrap();

let result = check_email(&input).await;
```

### Webhook Integration
```rust
pub struct TaskWebhook {
    pub url: String,
    pub extra: serde_json::Value,
}
```
Results are POSTed to the webhook URL upon completion.

---

## 9. Operational Concerns

### Configuration Reference

#### Environment Variables (Prefix: `RCH__`)
| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `RCH__HTTP__HOST` | String | `127.0.0.1` | HTTP server bind address |
| `RCH__HTTP__PORT` | u16 | `8080` | HTTP server port |
| `RCH__HTTP__HEADER_SECRET` | String | None | API authentication secret |
| `RCH__WORKER__ENABLE` | bool | `true` | Enable background worker |
| `RCH__WORKER__RABBITMQ__URL` | String | Required | RabbitMQ connection URL |
| `RCH__WORKER__RABBITMQ__CONCURRENCY` | u16 | `5` | Worker concurrency |
| `RCH__STORAGE__POSTGRES__DB_URL` | String | None | PostgreSQL connection URL |
| `RCH__WEBDRIVER__ADDR` | String | None | ChromeDriver address |
| `RCH__BACKEND_NAME` | String | None | Instance identifier |
| `SENTRY_DSN` | String | None | Sentry error reporting |
| `RUST_LOG` | String | `info` | Log level |

#### Configuration File (backend_config.toml)
```toml
[verif_method]
# Gmail verification via SMTP
[verif_method.gmail.Smtp]
from_email = "verify@example.com"
hello_name = "example.com"
smtp_port = 25
smtp_timeout = { secs = 30, nanos = 0 }
retries = 2

# Yahoo verification options
[verif_method.yahoo]
# Options: "Headless", "Api", or Smtp config
type = "Headless"

# Hotmail B2C verification
[verif_method.hotmailb2c]
# Options: "Headless" or Smtp config
type = "Headless"

# Throttle configuration
[throttle.max_requests]
per_second = 10
per_minute = 100
per_hour = 1000
per_day = 10000

# Proxy definitions
[verif_method.proxies.proxy1]
host = "proxy.example.com"
port = 1080
username = "user"
password = "pass"
```

### Deployment Options

#### Docker
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin reacher_backend

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/reacher_backend /usr/local/bin/
CMD ["reacher_backend"]
```

#### Kubernetes (Helm)
```bash
helm install reacher ./helm \
  --set config.http.headerSecret=your-secret \
  --set config.worker.rabbitmq.url=amqp://... \
  --set config.storage.postgres.dbUrl=postgres://...
```

#### AWS Lambda
```bash
cargo build --release --bin reacher_sqs
# Deploy with SAM or Terraform
```

### Monitoring & Observability

#### Logging
```rust
tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::from_default_env())
    .json()  // JSON format for log aggregation
    .init();
```

#### Key Metrics to Monitor
| Metric | Source | Alert Threshold |
|--------|--------|-----------------|
| Request latency | Warp middleware | > 30s |
| Queue depth | RabbitMQ | > 1000 messages |
| Worker errors | Sentry | > 10/minute |
| Throttle rejections | Application logs | > 50/minute |
| Database connections | PostgreSQL | > 80% pool |

#### Health Checks
- `GET /` returns `OK` for load balancer health checks
- Worker health: Check RabbitMQ connection status
- Database health: Check PostgreSQL connection pool

### Scaling Considerations
- **Horizontal scaling**: Multiple backend instances with shared RabbitMQ
- **Worker concurrency**: Configurable per instance
- **Rate limiting**: Applied at individual instance level (consider Redis for distributed)
- **Database connections**: Pool size should match expected concurrency

---

## 10. Testing / CI / Tooling

### Test Structure
```
core/
└── src/
    ├── lib.rs          # Integration tests
    ├── smtp/
    │   └── mod.rs      # SMTP timeout tests
    └── rules.rs        # Rule matching tests
```

### Running Tests
```bash
# All tests
cargo test

# Specific crate
cargo test -p check-if-email-exists

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### CI/CD Workflows

#### Pull Request Checks (`.github/workflows/pr.yml`)
- Rust format check (`cargo fmt --check`)
- Clippy lints (`cargo clippy`)
- Unit tests (`cargo test`)
- Build verification

#### Release Automation (`.github/workflows/release.yml`)
- Tag-triggered releases
- Multi-platform binary builds
- Docker image publication
- Helm chart versioning

#### Additional Workflows
- `check_json.yml`: Validates JSON configuration files
- `discord.yml`: Release notifications
- `sslyze.yml`: SSL/TLS configuration testing

### Development Commands
```bash
# Build all crates
cargo build

# Build release
cargo build --release

# Run backend locally
cargo run -p reacher_backend

# Run CLI
cargo run -p reacher -- someone@example.com

# Format code
cargo fmt

# Lint
cargo clippy

# Generate documentation
cargo doc --open
```

### Makefile Targets
```makefile
build:
    cargo build --release

test:
    cargo test

docker:
    docker build -t reacher .

helm-package:
    helm package helm/
```

---

## 11. Risks / Gaps / Recommendations

### Known Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| SMTP blocking by providers | High | Use proxies, rotate IPs, implement backoff |
| Rate limiting per-instance only | Medium | Consider Redis for distributed limiting |
| Headless browser resource usage | Medium | Limit concurrent headless sessions |
| Single MX record selection | Low | Currently uses lowest priority only |
| No retry persistence | Medium | Requeued tasks lost on crash |

### Technical Debt
1. **Duplicated code**: `sqs/main.rs` duplicates storage/webhook logic from backend
2. **Clone overhead**: Several `clone()` calls noted as TODO in `smtp/mod.rs`
3. **Beta rules system**: `rules.rs` marked as needing refinement
4. **Hard-coded ChromeDriver path**: Lambda handler uses `/opt/chromedriver-linux64/chromedriver`

### Recommendations

#### Short-term
1. Add distributed rate limiting with Redis
2. Implement proper retry queue with persistence
3. Add metrics endpoint for Prometheus scraping
4. Consolidate webhook/storage logic between backend and SQS handler

#### Medium-term
1. Add circuit breaker for external service failures
2. Implement connection pooling for SMTP connections
3. Add support for multiple MX record attempts
4. Create comprehensive integration test suite

#### Long-term
1. Consider message-based architecture (Kafka/NATS) for better scaling
2. Implement email verification result caching
3. Add ML-based deliverability prediction
4. Support additional verification methods (BIMI, DMARC)

---

## 12. Glossary

### Domain Terms
| Term | Definition |
|------|------------|
| **MX Record** | DNS Mail Exchange record pointing to mail servers |
| **Catch-All** | Domain configured to accept all email addresses |
| **Disposable Email** | Temporary email from services like Mailinator |
| **Role Account** | Generic address like info@, admin@, support@ |
| **SMTP** | Simple Mail Transfer Protocol (RFC 5321) |
| **EHLO/HELO** | SMTP greeting commands |
| **RCPT TO** | SMTP command to specify recipient |
| **STARTTLS** | Command to upgrade SMTP connection to TLS |
| **Deliverable** | Email address that will accept messages |
| **Full Inbox** | Mailbox at storage capacity (550 response) |

### Technical Terms
| Term | Definition |
|------|------------|
| **Warp** | Rust async web framework built on hyper |
| **Tokio** | Rust async runtime for concurrent execution |
| **lapin** | Rust AMQP 0.9.1 client for RabbitMQ |
| **sqlx** | Rust async SQL library with compile-time checks |
| **fantoccini** | Rust WebDriver client for browser automation |
| **rustls** | Pure-Rust TLS implementation |
| **SOCKS5** | Proxy protocol for TCP connections |
| **Prefetch** | RabbitMQ setting for message buffering |
| **Requeue** | Returning message to queue for retry |

### Reachability Levels
| Level | Meaning |
|-------|---------|
| **Safe** | High confidence the email exists and is deliverable |
| **Risky** | Email exists but has risk factors (catch-all, disposable, role) |
| **Invalid** | Email definitely does not exist or is undeliverable |
| **Unknown** | Could not determine status (timeout, server error) |

### Provider Abbreviations
| Abbreviation | Full Name |
|--------------|-----------|
| **B2C** | Business-to-Consumer (personal accounts) |
| **B2B** | Business-to-Business (Office 365, corporate) |
| **MX** | Mail Exchange |
| **HIBP** | Have I Been Pwned (breach database) |

---

*This document is auto-generated and should be updated when significant architectural changes occur.*
