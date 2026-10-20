# MillionVerifier Rust client

This local, unpublished workspace crate is generated with OpenAPI Generator **7.14.0** from
the specification embedded in <https://developer.millionverifier.com/> (API version 3.1.0,
OpenAPI 3.0.0), retrieved on 2026-10-20.

`openapi.upstream.json` preserves the source. `scripts/generate_millionverifier_sdk.py`
creates the working spec and generated code. Run `make sdk-generate-millionverifier`.
Do not edit generated files independently.

The working spec includes real-time verification and credit balance, which are the
operations used by the platform's row-based job queue. It normalizes the HTTPS base URL,
the canonical trailing slash, and the equivalent HTTP 200 response alternatives. Result
values remain strings/integers so the backend can reject unknown combinations rather than
silently accept them. Subresults may be empty on provider errors.

The deterministic postprocessing aligns reqwest with the backend (0.13/Rustls), disables
crate publishing, removes unused generated dependencies/publishing scripts, and strips
credential-bearing URLs from transport errors. The backend supplies connection/request
timeouts, disables redirects and automatic HTTP retries, validates application-level
errors and live mode, and redacts credentials in its configuration.

The SDK is an HTTP client, not a verification policy or billing engine. The backend owns
status mapping, tenant quotas, suppression rules, task recovery and paid-attempt records.
MillionVerifier's vendor-side bulk upload/poll/download API is intentionally not part of
this first client; uploaded platform lists already run through durable per-address tasks.
