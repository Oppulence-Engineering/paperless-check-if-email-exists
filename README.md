[![Crate](https://img.shields.io/crates/v/check-if-email-exists.svg)](https://crates.io/crates/check-if-email-exists)
[![Docs](https://docs.rs/check-if-email-exists/badge.svg)](https://docs.rs/check-if-email-exists)
[![Docker](https://img.shields.io/docker/v/reacherhq/backend?color=0db7ed&label=docker&sort=date)](https://hub.docker.com/r/reacherhq/backend)
[![Actions Status](https://github.com/reacherhq/check-if-email-exists/workflows/pr/badge.svg)](https://github.com/reacherhq/check-if-email-exists/actions)

<br /><br />

<p align="center"><img align="center" src="https://storage.googleapis.com/saasify-uploads-prod/696e287ad79f0e0352bc201b36d701849f7d55e7.svg" height="96" alt="reacher" /></p>
<h1 align="center">check-if-email-exists</h1>
<h4 align="center">Check if an email address exists without sending any email.<br/>Comes with a <a href="./backend">⚙️ backend/API engine</a>.</h4>

<br /><br /><br />

## Repository Scope

This repository is strongest as a backend/API engine for email verification and list hygiene workflows. The supported core in this codebase is:

- single email verification
- bulk jobs and CSV list cleaning
- suppression management
- scheduled re-verification and pipelines
- tenant auth, quotas, job approval, and email history

The hosted Reacher dashboard is a separate product surface and is not part of this repository.

## 👉 Live Demo: https://reacher.email

The open-source backend can also be used through the hosted Reacher SaaS. That hosted dashboard experience is separate from this repo's backend and worker implementation.

## Get Started

3 non-SaaS ways to get started with `check-if-email-exists`.

### 1. ⚙️ HTTP backend using Docker (popular method 🥇) [[Full docs](./backend/README.md)]

This option allows you to run a HTTP backend using Docker 🐳, on a cloud instance or your own server. Please note that outbound port 25 must be open.

```bash
docker run -p 8080:8080 reacherhq/backend:latest
```

Then send a `POST http://localhost:8080/v0/check_email` request with the following body:

```js
{
    "to_email": "someone@gmail.com",
    "proxy": {                        // (optional) SOCK5 proxy to run the verification through, default is empty
        "host": "my-proxy.io",
        "port": 1080,
        "username": "me",             // (optional) Proxy username
        "password": "pass"            // (optional) Proxy password
    }
}
```

### 2. Download the CLI [[Full docs](./cli/README.md)]

> Note: The CLI binary doesn't connect to any backend, it checks the email directly from your computer.

Head to the [releases page](https://github.com/reacherhq/check-if-email-exists/releases) and download the binary for your platform.

```bash
> $ check_if_email_exists --help
check_if_email_exists 0.9.1
Check if an email address exists without sending an email.

USAGE:
    check_if_email_exists [FLAGS] [OPTIONS] [TO_EMAIL]
```

Check out the [dedicated README.md](./cli/README.md) for all options and flags.

### 3. Programmatic Usage [[Full docs](https://docs.rs/check-if-email-exists)]

In your own Rust project, you can add `check-if-email-exists` in your `Cargo.toml`:

```toml
[dependencies]
check-if-email-exists = "0.9"
```

And use it in your code as follows:

```rust
use check_if_email_exists::{check_email, CheckEmailInput, CheckEmailInputProxy};

async fn check() {
    // Let's say we want to test the deliverability of someone@gmail.com.
    let mut input = CheckEmailInput::new(vec!["someone@gmail.com".into()]);

    // Verify this email, using async/await syntax.
    let result = check_email(&input).await;

    // `result` is a `Vec<CheckEmailOutput>`, where the CheckEmailOutput
    // struct contains all information about our email.
    println!("{:?}", result);
}
```

The reference docs are hosted on [docs.rs](https://docs.rs/check-if-email-exists).

## ✈️ JSON Output

The output will be a JSON with the below format, the fields should be self-explanatory. For `someone@gmail.com` (note that it is disabled by Gmail), here's the exact output:

```json
{
	"input": "someone@gmail.com",
	"is_reachable": "invalid",
	"misc": {
		"is_disposable": false,
		"is_role_account": false,
		"is_b2c": true
	},
	"mx": {
		"accepts_mail": true,
		"records": [
			"alt3.gmail-smtp-in.l.google.com.",
			"gmail-smtp-in.l.google.com.",
			"alt1.gmail-smtp-in.l.google.com.",
			"alt4.gmail-smtp-in.l.google.com.",
			"alt2.gmail-smtp-in.l.google.com."
		]
	},
	"smtp": {
		"can_connect_smtp": true,
		"has_full_inbox": false,
		"is_catch_all": false,
		"is_deliverable": false,
		"is_disabled": true
	},
	"syntax": {
		"domain": "gmail.com",
		"is_valid_syntax": true,
		"username": "someone",
		"suggestion": null
	}
}
```

## What Does This Tool Check?

| Included? | Feature                                       | Description                                                                                                                     | JSON field                                                                |
| --------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| ✅        | **Email reachability**                        | How confident are we in sending an email to this address? Can be one of `safe`, `risky`, `invalid` or `unknown`.                | `is_reachable`                                                            |
| ✅        | **Syntax validation**                         | Is the address syntactically valid?                                                                                             | `syntax.is_valid_syntax`                                                  |
| ✅        | **DNS records validation**                    | Does the domain of the email address have valid MX DNS records?                                                                 | `mx.accepts_mail`                                                         |
| ✅        | **Disposable email address (DEA) validation** | Is the address provided by a known [disposable email address](https://en.wikipedia.org/wiki/Disposable_email_address) provider? | `misc.is_disposable`                                                      |
| ✅        | **SMTP server validation**                    | Can the mail exchanger of the email address domain be contacted successfully?                                                   | `smtp.can_connect_smtp`                                                   |
| ✅        | **Email deliverability**                      | Is an email sent to this address deliverable?                                                                                   | `smtp.is_deliverable`                                                     |
| ✅        | **Mailbox disabled**                          | Has this email address been disabled by the email provider?                                                                     | `smtp.is_disabled`                                                        |
| ✅        | **Full inbox**                                | Is the inbox of this mailbox full?                                                                                              | `smtp.has_full_inbox`                                                     |
| ✅        | **Catch-all address**                         | Is this email address a [catch-all](https://debounce.io/blog/help/what-is-a-catch-all-or-accept-all/) address?                  | `smtp.is_catch_all`                                                       |
| ✅        | **Role account validation**                   | Is the email address a well-known role account?                                                                                 | `misc.is_role_account`                                                    |
| ✅        | **Gravatar Url**                              | The url of the [Gravatar](https://gravatar.com/) email address profile picture                                                  | `misc.gravatar_url`                                                       |
| ✅        | **Have I Been Pwned?**                        | Has this email been compromised in a [data breach](https://haveibeenpwned.com/)?                                                | `misc.haveibeenpwned`                                                     |
| 🔜        | **Free email provider check**                 | Is the email address bound to a known free email provider?                                                                      | [Issue #89](https://github.com/reacherhq/check-if-email-exists/issues/89) |
| 🔜        | **Syntax validation, provider-specific**      | According to the syntactic rules of the target mail provider, is the address syntactically valid?                               | [Issue #90](https://github.com/reacherhq/check-if-email-exists/issues/90) |
| 🔜        | **Honeypot detection**                        | Does email address under test hide a [honeypot](https://en.wikipedia.org/wiki/Spamtrap)?                                        | [Issue #91](https://github.com/reacherhq/check-if-email-exists/issues/91) |

## 🏗️ Architecture

### System Architecture Overview

```mermaid
graph TB
    subgraph "Client Layer"
        CLI[CLI Tool]
        HTTP_CLIENT[HTTP Client]
    end

    subgraph "Backend Service"
        HTTP_SERVER[Warp HTTP Server]
        CONFIG[BackendConfig]
        THROTTLE[ThrottleManager]
    end

    subgraph "API Endpoints"
        V0_CHECK[v0/check_email]
        V1_CHECK[v1/check_email]
        V0_BULK[v0/bulk]
        V1_BULK[v1/bulk]
        VERSION[/version/]
    end

    subgraph "Worker System"
        RABBITMQ[RabbitMQ Queue<br/>check_email]
        WORKER[Worker Consumer]
        SINGLE_SHOT[Single-Shot Handler]
    end

    subgraph "Core Verification Library"
        SYNTAX[Syntax Check]
        MX[MX DNS Lookup]
        SMTP[SMTP Verification]
        MISC[Misc Checks]
    end

    subgraph "SMTP Providers"
        GMAIL[Gmail SMTP]
        YAHOO[Yahoo API/Headless]
        HOTMAIL[Hotmail Headless/SMTP]
        GENERIC[Generic SMTP]
    end

    subgraph "Storage Layer"
        POSTGRES[(PostgreSQL)]
        STORAGE[Storage Adapter]
        V0_RESULTS[email_results table]
        V1_RESULTS[v1_task_result table]
        BULK_JOBS[bulk_jobs table]
    end

    subgraph "External Services"
        DNS[DNS Servers]
        SMTP_SERVERS[SMTP Servers]
        WEBDRIVER[WebDriver<br/>for Headless]
        PROXY[SOCKS5 Proxy]
        WEBHOOK[Webhook URLs]
        HIBP[Have I Been Pwned API]
        GRAVATAR[Gravatar API]
    end

    CLI --> HTTP_SERVER
    HTTP_CLIENT --> HTTP_SERVER
    HTTP_SERVER --> CONFIG
    HTTP_SERVER --> THROTTLE
    HTTP_SERVER --> V0_CHECK
    HTTP_SERVER --> V1_CHECK
    HTTP_SERVER --> V0_BULK
    HTTP_SERVER --> V1_BULK
    HTTP_SERVER --> VERSION

    V0_CHECK --> SYNTAX
    V1_CHECK --> RABBITMQ
    V1_CHECK --> SYNTAX
    V0_BULK --> POSTGRES
    V1_BULK --> RABBITMQ

    RABBITMQ --> WORKER
    WORKER --> THROTTLE
    WORKER --> SYNTAX
    WORKER --> SINGLE_SHOT

    SYNTAX --> MX
    MX --> DNS
    MX --> SMTP
    SMTP --> GMAIL
    SMTP --> YAHOO
    SMTP --> HOTMAIL
    SMTP --> GENERIC
    SMTP --> PROXY
    YAHOO --> WEBDRIVER
    HOTMAIL --> WEBDRIVER

    SYNTAX --> MISC
    MISC --> HIBP
    MISC --> GRAVATAR

    WORKER --> STORAGE
    V0_CHECK --> STORAGE
    V1_CHECK --> STORAGE
    STORAGE --> POSTGRES
    POSTGRES --> V0_RESULTS
    POSTGRES --> V1_RESULTS
    POSTGRES --> BULK_JOBS

    WORKER --> WEBHOOK
    GMAIL --> SMTP_SERVERS
    GENERIC --> SMTP_SERVERS
```

### Email Verification Process Flow

```mermaid
flowchart TD
    START([Email Input]) --> SYNTAX_CHECK{check_syntax}

    SYNTAX_CHECK -->|Invalid| INVALID[Return Invalid<br/>is_reachable: Invalid]
    SYNTAX_CHECK -->|Valid| MX_LOOKUP[check_mx<br/>DNS MX Lookup]

    MX_LOOKUP -->|No MX Records| NO_MX[Return Invalid<br/>is_reachable: Invalid]
    MX_LOOKUP -->|Error| UNKNOWN[Return Unknown<br/>is_reachable: Unknown]
    MX_LOOKUP -->|Success| SELECT_MX[Select MX Record<br/>Lowest Priority]

    SELECT_MX --> PROVIDER_CHECK{Identify Provider}

    PROVIDER_CHECK -->|Gmail| GMAIL_CONFIG[Gmail SMTP Config]
    PROVIDER_CHECK -->|Yahoo| YAHOO_CHECK{Yahoo Method}
    PROVIDER_CHECK -->|Hotmail B2C| HOTMAIL_B2C_CHECK{Hotmail B2C Method}
    PROVIDER_CHECK -->|Hotmail B2B| HOTMAIL_B2B_CONFIG[Hotmail B2B SMTP Config]
    PROVIDER_CHECK -->|Mimecast| MIMECAST_CONFIG[Mimecast SMTP Config]
    PROVIDER_CHECK -->|Proofpoint| PROOFPOINT_CONFIG[Proofpoint SMTP Config]
    PROVIDER_CHECK -->|Other| GENERIC_CONFIG[Generic SMTP Config]

    YAHOO_CHECK -->|API| YAHOO_API[Yahoo API Call]
    YAHOO_CHECK -->|Headless| YAHOO_HEADLESS[Yahoo Headless Browser]
    YAHOO_CHECK -->|SMTP| YAHOO_SMTP[Yahoo SMTP]

    HOTMAIL_B2C_CHECK -->|Headless| HOTMAIL_HEADLESS[Hotmail Headless Browser]
    HOTMAIL_B2C_CHECK -->|SMTP| HOTMAIL_SMTP[Hotmail SMTP]

    GMAIL_CONFIG --> SMTP_CONNECT
    YAHOO_SMTP --> SMTP_CONNECT
    HOTMAIL_SMTP --> SMTP_CONNECT
    HOTMAIL_B2B_CONFIG --> SMTP_CONNECT
    MIMECAST_CONFIG --> SMTP_CONNECT
    PROOFPOINT_CONFIG --> SMTP_CONNECT
    GENERIC_CONFIG --> SMTP_CONNECT

    SMTP_CONNECT[Connect to SMTP Server<br/>via Proxy if configured] --> SMTP_EHLO[EHLO Command]
    SMTP_EHLO --> SMTP_MAIL[MAIL FROM Command]
    SMTP_MAIL --> SMTP_RCPT[RCPT TO Command]
    SMTP_RCPT --> PARSE_RESPONSE[Parse SMTP Response]

    PARSE_RESPONSE --> SMTP_RESULT{SMTP Result}

    SMTP_RESULT -->|Success| DELIVERABLE[is_deliverable: true]
    SMTP_RESULT -->|Disabled| DISABLED[is_disabled: true]
    SMTP_RESULT -->|Full Inbox| FULL_INBOX[has_full_inbox: true]
    SMTP_RESULT -->|Catch-All| CATCH_ALL[is_catch_all: true]
    SMTP_RESULT -->|Error| SMTP_ERROR[SmtpError]

    YAHOO_API --> API_RESULT[API Result]
    YAHOO_HEADLESS --> HEADLESS_RESULT[Headless Result]
    HOTMAIL_HEADLESS --> HEADLESS_RESULT

    DELIVERABLE --> MISC_CHECK
    DISABLED --> MISC_CHECK
    FULL_INBOX --> MISC_CHECK
    CATCH_ALL --> MISC_CHECK
    SMTP_ERROR --> MISC_CHECK
    API_RESULT --> MISC_CHECK
    HEADLESS_RESULT --> MISC_CHECK

    MISC_CHECK[check_misc] --> DISPOSABLE{is_disposable?}
    DISPOSABLE --> ROLE_ACCOUNT{is_role_account?}
    ROLE_ACCOUNT --> GRAVATAR_CHECK[Check Gravatar]
    GRAVATAR_CHECK --> HIBP_CHECK[Check Have I Been Pwned]

    HIBP_CHECK --> CALCULATE_REACHABLE[calculate_reachable]

    CALCULATE_REACHABLE --> REACHABLE_RESULT{Reachability}
    REACHABLE_RESULT -->|Safe| SAFE[is_reachable: Safe]
    REACHABLE_RESULT -->|Risky| RISKY[is_reachable: Risky]
    REACHABLE_RESULT -->|Invalid| INVALID_REACH[is_reachable: Invalid]
    REACHABLE_RESULT -->|Unknown| UNKNOWN_REACH[is_reachable: Unknown]

    SAFE --> OUTPUT[CheckEmailOutput]
    RISKY --> OUTPUT
    INVALID_REACH --> OUTPUT
    UNKNOWN_REACH --> OUTPUT
    INVALID --> OUTPUT
    NO_MX --> OUTPUT
    UNKNOWN --> OUTPUT

    OUTPUT --> END([Return Result])
```

### API Request Flow - Single Email Check

```mermaid
sequenceDiagram
    participant Client
    participant HTTP_Server
    participant ThrottleManager
    participant Worker as Worker Mode?
    participant RabbitMQ
    participant Worker_Process
    participant Core_Lib as Core Library
    participant Storage
    participant Postgres

    Client->>HTTP_Server: POST /v0/check_email or /v1/check_email
    HTTP_Server->>ThrottleManager: check_throttle()

    alt Throttled
        ThrottleManager-->>HTTP_Server: ThrottleResult
        HTTP_Server-->>Client: 429 Too Many Requests
    else Not Throttled
        alt v0/check_email (Direct)
            HTTP_Server->>Core_Lib: check_email()
            Core_Lib->>Core_Lib: Syntax Check
            Core_Lib->>Core_Lib: MX Lookup
            Core_Lib->>Core_Lib: SMTP Verification
            Core_Lib->>Core_Lib: Misc Checks
            Core_Lib-->>HTTP_Server: CheckEmailOutput
            HTTP_Server->>Storage: store()
            Storage->>Postgres: INSERT result
            HTTP_Server-->>Client: 200 OK + JSON
        else v1/check_email (Worker Mode)
            HTTP_Server->>RabbitMQ: Publish task (priority 5)
            RabbitMQ-->>HTTP_Server: ACK
            HTTP_Server->>RabbitMQ: Create reply queue
            HTTP_Server->>RabbitMQ: Consume reply queue

            RabbitMQ->>Worker_Process: Deliver task
            Worker_Process->>ThrottleManager: check_throttle()
            alt Worker Throttled
                Worker_Process->>RabbitMQ: Reject (requeue=false)
                Worker_Process->>RabbitMQ: Send error reply
                RabbitMQ-->>HTTP_Server: Error reply
                HTTP_Server-->>Client: 429 Too Many Requests
            else Worker Not Throttled
                Worker_Process->>Core_Lib: check_email()
                Core_Lib->>Core_Lib: Syntax Check
                Core_Lib->>Core_Lib: MX Lookup
                Core_Lib->>Core_Lib: SMTP Verification
                Core_Lib->>Core_Lib: Misc Checks
                Core_Lib-->>Worker_Process: CheckEmailOutput

                alt Result is Unknown
                    Worker_Process->>RabbitMQ: Reject (requeue=true)
                else Success or Error
                    Worker_Process->>RabbitMQ: ACK
                    Worker_Process->>Storage: store()
                    Storage->>Postgres: INSERT result
                    Worker_Process->>RabbitMQ: Send reply
                    RabbitMQ-->>HTTP_Server: Success reply
                    HTTP_Server-->>Client: 200 OK + JSON
                end
            end
        end
        ThrottleManager->>ThrottleManager: increment_counters()
    end
```

### Worker Queue Architecture

```mermaid
graph TB
    subgraph "HTTP Server"
        API_HANDLER[API Handler]
        TASK_PUBLISHER[Task Publisher]
    end

    subgraph "RabbitMQ"
        QUEUE[check_email Queue<br/>Priority Queue<br/>Max Priority: 5]
        REPLY_QUEUE[Reply Queue<br/>Auto-delete<br/>Exclusive]
    end

    subgraph "Worker Process"
        CONSUMER[Message Consumer]
        THROTTLE_CHECK[Throttle Check]
        TASK_PROCESSOR[Task Processor]
        EMAIL_CHECKER[Email Checker]
        RESULT_HANDLER[Result Handler]
    end

    subgraph "Task Types"
        SINGLE_SHOT_TASK[Single-Shot Task<br/>Priority: 5]
        BULK_TASK[Bulk Task<br/>Priority: 1]
    end

    subgraph "Result Handling"
        STORAGE[Storage Adapter]
        WEBHOOK_SENDER[Webhook Sender]
        REPLY_SENDER[Reply Sender]
    end

    subgraph "PostgreSQL"
        DB[(Database)]
    end

    API_HANDLER --> TASK_PUBLISHER
    TASK_PUBLISHER -->|Publish| QUEUE
    TASK_PUBLISHER -->|Create| REPLY_QUEUE

    QUEUE -->|Deliver| CONSUMER
    CONSUMER --> THROTTLE_CHECK

    THROTTLE_CHECK -->|Throttled| THROTTLE_REJECT{Job Type?}
    THROTTLE_CHECK -->|Not Throttled| TASK_PROCESSOR

    THROTTLE_REJECT -->|Single-Shot| REJECT_NO_REQUEUE[Reject<br/>requeue=false]
    THROTTLE_REJECT -->|Bulk| REJECT_REQUEUE[Reject<br/>requeue=true]

    REJECT_NO_REQUEUE --> REPLY_SENDER
    REJECT_REQUEUE --> QUEUE

    TASK_PROCESSOR --> EMAIL_CHECKER
    EMAIL_CHECKER --> RESULT_HANDLER

    RESULT_HANDLER -->|Unknown Result| UNKNOWN_REJECT{First Attempt?}
    UNKNOWN_REJECT -->|Yes| QUEUE
    UNKNOWN_REJECT -->|No| ACK

    RESULT_HANDLER -->|Success/Error| ACK[ACK Message]
    ACK --> STORAGE
    ACK --> WEBHOOK_SENDER
    ACK --> REPLY_HANDLER{Job Type?}

    REPLY_HANDLER -->|Single-Shot| REPLY_SENDER
    REPLY_HANDLER -->|Bulk| SKIP_REPLY[Skip Reply]

    REPLY_SENDER --> REPLY_QUEUE
    STORAGE --> DB
    WEBHOOK_SENDER -->|HTTP POST| WEBHOOK_URL[Webhook URL]

    SINGLE_SHOT_TASK --> QUEUE
    BULK_TASK --> QUEUE
```

### Bulk Job Processing Flow

```mermaid
sequenceDiagram
    participant Client
    participant API as API Handler
    participant Postgres
    participant RabbitMQ
    participant Worker
    participant Core as Core Library
    participant Storage
    participant Webhook

    Note over Client,Webhook: v1/bulk Flow (RabbitMQ-based)

    Client->>API: POST /v1/bulk<br/>{input: [emails], webhook?}
    API->>Postgres: INSERT INTO v1_bulk_job<br/>RETURNING id
    Postgres-->>API: job_id

    loop For each email (concurrent: 10)
        API->>RabbitMQ: Publish CheckEmailTask<br/>{job_id: Bulk(id), priority: 1}
    end

    API-->>Client: 200 OK<br/>{job_id}

    Note over RabbitMQ,Worker: Worker Processing

    RabbitMQ->>Worker: Deliver task
    Worker->>Worker: Check throttle

    alt Throttled
        Worker->>RabbitMQ: Reject (requeue=true)
        RabbitMQ->>Worker: Redeliver later
    else Not Throttled
        Worker->>Core: check_email()
        Core-->>Worker: CheckEmailOutput

        alt Result is Unknown (first attempt)
            Worker->>RabbitMQ: Reject (requeue=true)
            RabbitMQ->>Worker: Redeliver
        else Success/Error
            Worker->>RabbitMQ: ACK
            Worker->>Storage: store(task, result)
            Storage->>Postgres: INSERT INTO v1_task_result
            Worker->>Webhook: POST webhook URL (if configured)
            Webhook-->>Worker: Response
        end
    end

    Note over Client,Webhook: v0/bulk Flow (sqlxmq-based)

    Client->>API: POST /v0/bulk<br/>{input: [emails]}
    API->>Postgres: INSERT INTO bulk_jobs<br/>RETURNING id
    Postgres-->>API: job_id

    loop For each email
        API->>Postgres: sqlxmq::spawn<br/>email_verification_task
    end

    API-->>Client: 200 OK<br/>{job_id}

    Note over Postgres,Core: sqlxmq Job Processing

    Postgres->>Postgres: mq_poll()<br/>Fetch job
    Postgres->>Core: email_verification_task()<br/>check_email() with retries
    Core-->>Postgres: CheckEmailOutput

    Postgres->>Postgres: INSERT INTO email_results<br/>{job_id, result}
    Postgres->>Postgres: Job complete
```

### Component Interaction Diagram

```mermaid
graph LR
    subgraph "Configuration Layer"
        CONFIG_FILE[backend_config.toml]
        ENV_VARS[Environment Variables<br/>RCH__*]
        CONFIG_LOADER[Config Loader]
    end

    subgraph "HTTP Layer"
        WARP[Warp Server]
        ROUTES[Route Handlers]
        MIDDLEWARE[Middleware<br/>Auth, Throttle, Logging]
    end

    subgraph "Business Logic"
        VERIFICATION[Email Verification]
        THROTTLE_LOGIC[Throttle Logic]
        STORAGE_LOGIC[Storage Logic]
    end

    subgraph "Core Library"
        SYNTAX_MODULE[Syntax Module]
        MX_MODULE[MX Module]
        SMTP_MODULE[SMTP Module]
        MISC_MODULE[Misc Module]
    end

    subgraph "Infrastructure"
        RABBITMQ_INFRA[RabbitMQ]
        POSTGRES_INFRA[PostgreSQL]
        DNS_INFRA[DNS Resolver]
        SMTP_INFRA[SMTP Servers]
    end

    CONFIG_FILE --> CONFIG_LOADER
    ENV_VARS --> CONFIG_LOADER
    CONFIG_LOADER --> WARP
    CONFIG_LOADER --> VERIFICATION
    CONFIG_LOADER --> THROTTLE_LOGIC

    WARP --> MIDDLEWARE
    MIDDLEWARE --> ROUTES
    ROUTES --> VERIFICATION
    ROUTES --> THROTTLE_LOGIC
    ROUTES --> STORAGE_LOGIC

    VERIFICATION --> SYNTAX_MODULE
    VERIFICATION --> MX_MODULE
    VERIFICATION --> SMTP_MODULE
    VERIFICATION --> MISC_MODULE

    SYNTAX_MODULE --> DNS_INFRA
    MX_MODULE --> DNS_INFRA
    SMTP_MODULE --> SMTP_INFRA

    THROTTLE_LOGIC --> RABBITMQ_INFRA
    STORAGE_LOGIC --> POSTGRES_INFRA
    VERIFICATION --> RABBITMQ_INFRA
    VERIFICATION --> POSTGRES_INFRA
```

## 🤔 Why?

Many online services (https://hunter.io, https://verify-email.org, https://email-checker.net) offer this service for a paid fee. Here is an open-source alternative to those tools.

### Roadmap Ideas / Nice-to-Haves

- **Contact intelligence & enrichment** – augment verification output with titles, firmographics, technographic signals, and social intent data to rival Hunter.io, Apollo, or Fliar.
- **Prospecting workflows** – support domain/person queries, bulk CSV enrich → verify → export loops, and configurable lead scoring to build campaign-ready lists.
- **Deliverability toolkit** – scheduled re-verification, inbox warm-up scores, provider-aware retry heuristics, and spam-trap/DEA/honeypot risk modeling.
- **Automation & integrations** – drag-and-drop workflow builder plus native Salesforce, HubSpot, Outreach, Salesloft, Pipedrive, Close, and webhook/GraphQL connectors.
- **Compliance & auditability** – consent tracking, suppression list sync, retention policies, and immutable audit logs covering data collection + verification events.
- **Collaboration & AI assistance** – shared workspaces, fine-grained permissions, and AI-generated persona summaries or icebreakers based on enriched data.
## License

`check-if-email-exists`'s source code is provided under a **dual license model**.

### Commercial license

If you want to use `check-if-email-exists` to develop commercial sites, tools, and applications, the Commercial License is the appropriate license. With this option, your source code is kept proprietary. Purchase a `check-if-email-exists` Commercial License at https://reacher.email/pricing.

### Open source license

If you are creating an open-source application under a license compatible with the GNU Affero GPL License v3, you may use `check-if-email-exists` under the terms of the [AGPL-3.0](./LICENSE.AGPL).

[➡️ Read more](https://docs.reacher.email/self-hosting/licensing) about Reacher's license.

## 🔨 Build From Source

Build the [CLI from source](./cli/README.md#build-from-source) or the [HTTP backend from source](./backend/README.md#build-from-source).

## Infrastructure & Hosting Notes

- **RabbitMQ / Worker Queue** – The managed AMQP cluster is provisioned on [CloudAMQP](https://api.cloudamqp.com/console/75fa503d-fdf0-4d60-a1b0-9250e364c8cb/nodes) and exposed via an `amqps://` URI. All worker deployments (including Helm) source the `RCH__WORKER__RABBITMQ__URL` secret to connect to this hosted control plane. For access, see the CloudAMQP console referenced above.

## Local Development & Testing

1. **Bootstrap dependencies**
   - Install Docker Desktop (or Podman + docker-compose shim), Go 1.22+, Rust toolchain (if building the backend), Node 18+ (for docs / ancillary tooling) and Helm 3.
   - Copy `.env.example` to `.env` and fill in SMTP proxy + auth secrets. These values are mounted into every container inside `rabbitmq/docker-compose.yaml`.
2. **Bring up the full stack**
   ```bash
   make deploy-and-test
   ```
   The target launches Postgres, RabbitMQ and two worker instances, then executes `scripts/test_email.go` which hits both single-shot and bulk endpoints until the health probe passes.
3. **Run ad-hoc checks**
   - `cargo run --bin backend` for local-only iterations.
   - `go run scripts/test_email.go` to revalidate after changing any API surface.
4. **Inspect infrastructure**
   - RabbitMQ UI: `http://localhost:15672` (guest/guest).
   - Postgres: `postgres://postgres:postgres@localhost:5432/reacher_db`.

## Configuration Reference

| Key | Purpose | Default / Source |
| --- | ------- | ---------------- |
| `RCH__STORAGE__POSTGRES__DB_URL` | Primary Postgres connection string | `.env`, GitHub secret `HELM_POSTGRES_URL`, Helm `config.secretEnv` |
| `RCH__WORKER__RABBITMQ__URL` | AMQP endpoint for worker queue | CloudAMQP URI, GitHub secret `HELM_RABBIT_URL` |
| `RCH__PROXY__*` | Bright Data SOCKS5 endpoints (host/port/user/password) | `.env`, GitHub secrets |
| `RCH__HELLO_NAME` / `RCH__FROM_EMAIL` | Values used during SMTP HELO / MAIL commands | `.env.example` |
| `RCH__HEADER_SECRET` | Value required in `x-reacher-secret` header for authenticated API calls | GitHub secret `RCH__HEADER_SECRET` |
| `RCH__SENTRY_DSN` | Observability hook for production deployments | GitHub secret `HELM_SENTRY_DSN` |

Refer to `backend/backend_config.toml` for the canonical schema and to `docs/self-hosting/reacher-configuration-v0.10.md` for the full narrative explanation.

## Proxy & Bright Data Integration

- The backend currently expects a **pure SOCKS5** proxy. Bright Data’s dedicated residential zones support this; the unified superproxy endpoint defaults to HTTP CONNECT and is therefore incompatible. See `docs/self-hosting/proxies/README.md` for instructions and the reference Helm values from the [lead scraper service](https://github.com/Oppulence-Engineering/oppulence-paperless-backend/tree/master/services/lead-scraper-service).
- Add the working proxy credentials to `.env` for local testing and mirror them into the GitHub Action secrets (`RCH__PROXY__HOST`, `RCH__PROXY__PORT`, etc.) so the Helm deployment inherits the same configuration via `values.yaml`.

## Containerization & Compose Stack

- `rabbitmq/docker-compose.yaml` defines a reproducible stack containing RabbitMQ, Postgres and two backend workers (mapped to `8080`/`8081`). Each worker reads from the same queue but advertises a unique `RCH__BACKEND_NAME` for logging clarity.
- The compose file consumes `.env`, so sensitive material never lives inside version control. Update the file to point at `ghcr.io/oppulence-engineering/check-if-email-exists-backend:latest` once the image is published, or keep using `reacherhq/backend:beta` while upstream tags catch up.

## CI/CD & Release Flow

- **Docker build + Helm deploy**: `.github/workflows/deploy_backend.yml` builds the backend image with `docker/build-push-action`, pushes it to GHCR, then defers Helm deployment to the reusable workflow (`deploy-helm-direct-template.yml`). Tagging a release (`v*.*.*`) automatically publishes the image and rolls out the chart to the `paperless` namespace.
- **CLI release pipeline**: `.github/workflows/deploy_cli.yml` builds the CLI binaries, signs them, and attaches artifacts to GitHub releases. Update this workflow when the CLI requires new targets or toolchains.
- **Reusable Helm template**: `.github/workflows/deploy-helm-direct-template.yml` constructs kubeconfig material straight from GitHub secrets (`HELM_DIRECT_*`). Any service in the org can inherit it by pointing `uses:` to the template and passing chart metadata plus image overrides.
- **Secret overrides**: The same template now accepts newline-delimited `secret_env_overrides` so deployments inject `config.secretEnv.*` values (e.g., `RCH__PROXY__PORT`) straight from GitHub Secrets without duplicating scripts across repos.

## Helm Chart Overview

- Located in `helm/check-if-email-exists`. Generated via `helm create` but extensively customized:
  - `values.yaml` separates `config.env` (non-sensitive toggles) from `config.secretEnv` so secrets can be injected at deploy time.
  - `templates/configmap-env.yaml` and `templates/secret-env.yaml` translate those maps into actual Kubernetes primitives that the `Deployment` consumes via `envFrom`.
- Override values during deploys using `--set` or `-f` overlays. In CI we rely on `--set image.repository=…` and `--set config.secretEnv.*` arguments supplied by the workflow to keep the chart generic.

## Observability & Alerting

- Structured logs (JSON) are emitted by default; set `RUST_LOG=reacher=debug` when diagnosing gnarly SMTP issues.
- Sentry is optional but strongly recommended in production. Populate `RCH__SENTRY_DSN` to capture panic-level crashes and SMTP edge cases.
- RabbitMQ, Postgres and backend health endpoints are exposed at `/version` and `/healthz` (enable auth headers if the service is public).

## Troubleshooting Checklist

1. **DNS issues inside containers** – `docker exec worker1 nslookup gmail.com`. If this fails, set explicit DNS servers in `docker-compose.yaml` (`dns: [“8.8.8.8”, “1.1.1.1”]`).
2. **Proxy errors** – `”Unsupported SOCKS version 72”` means the proxy is speaking HTTP; switch to Bright Data’s SOCKS5 endpoint or another compatible provider.
3. **SMTP failures for demo addresses** – Many demo domains lack MX records. Use a mailbox with real MX (e.g., `*@gmail.com`) when validating connectivity.
4. **CI Helm failures** – Confirm every secret referenced in `deploy-helm-direct-template.yml` exists (`HELM_DIRECT_SERVER`, `HELM_DIRECT_CA_DATA`, etc.). The workflow now validates presence before writing kubeconfig.
5. **Bulk jobs stuck in “Processing”** – Inspect RabbitMQ queue depth (`docker exec rabbitmq rabbitmqctl list_queues name messages_ready`) and ensure workers have network access to the proxy and databases.

## Ralph (Autonomous Agent Loop)

Ralph runs Claude Code in a loop to work through tasks from a PRD. Each iteration picks up one task, implements it, commits, and logs progress.

### Files

| File | Purpose |
|------|---------|
| `PRD.md` | Your requirements document -- Ralph reads this to find what to build next |
| `progress.txt` | Tracks completed work across iterations |
| `ralph-once.sh` | Human-in-the-loop -- runs one task, pauses for review |
| `afk-ralph.sh` | Fully autonomous -- runs N iterations in a Docker sandbox |

### Usage

**Create a PRD first.** Use Claude’s plan mode (`claude` then `shift-tab`) or write one manually.

```bash
# Interactive: one task at a time, review between runs
./ralph-once.sh

# Autonomous: run up to 20 tasks unattended in Docker sandbox
./afk-ralph.sh 20
```

The AFK version uses `docker sandbox run` for isolation. It stops early if Claude determines the PRD is complete.

**First-time setup for AFK mode:**
```bash
docker sandbox run claude
```

See [Getting Started with Ralph](https://www.aihero.dev/getting-started-with-ralph) for the full walkthrough.
