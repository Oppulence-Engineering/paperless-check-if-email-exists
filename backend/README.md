[![Docker](https://img.shields.io/docker/v/reacherhq/backend?color=0db7ed&label=docker&sort=date)](https://hub.docker.com/r/reacherhq/backend)
[![Actions Status](https://github.com/reacherhq/check-if-email-exists/workflows/pr/badge.svg)](https://github.com/reacherhq/check-if-email-exists/actions)

<br /><br />

<p align="center"><img align="center" src="https://storage.googleapis.com/saasify-uploads-prod/696e287ad79f0e0352bc201b36d701849f7d55e7.svg" height="96" alt="reacher" /></p>
<h1 align="center">⚙️ Reacher Backend</h1>
<h4 align="center">Backend/API engine for Reacher email verification workflows.</h4>

<br /><br />

This crate holds the backend for [Reacher](https://reacher.email). It is the HTTP API and worker engine behind the product, and it is designed to be useful on its own as a self-hosted backend for Sales/RevOps-style list hygiene workflows.

The supported core workflows in this crate are:

- single verification
- bulk verification jobs and list cleaning
- suppressions
- job approval and email history
- tenant auth and quotas
- scheduled re-verification and pipelines

The hosted dashboard is out of repo. This crate provides the backend/API surface that the dashboard and external integrations talk to.

The backend has the following main components:

-   [`check-if-email-exists`](https://github.com/reacherhq/check-if-email-exists), which performs the core email verification logic,
-   [`warp`](https://github.com/seanmonstar/warp) web framework,
-   [`RabbitMQ`](https://www.rabbitmq.com/) worker for consuming a queue of incoming verification requests.

## Get Started

The [Docker image](./Dockerfile) is hosted on Docker Hub: https://hub.docker.com/r/reacherhq/backend.

To run it, run the following command:

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
    },
}
```

## Configuration

The backend is configured via its [`backend_config.toml`](./backend_config.toml) file.

## API Documentation

See the full [OpenAPI documentation](https://docs.reacher.email/advanced/openapi).

## Build From Source

You can build the backend from source to generate a binary, and run the server locally on your machine. First, [install Rust](https://www.rust-lang.org/tools/install); you'll need Rust 1.37.0 or later. Make sure `openssl` is installed too. Then, run the following commands:

```bash
# Download the code
$ git clone https://github.com/reacherhq/check-if-email-exists
$ cd check-if-email-exists/backend

# Run the backend binary in release mode (slower build, but more performant).
$ cargo run --release --bin reacher_backend
```

The server will then be listening on `http://127.0.0.1:8080`.
