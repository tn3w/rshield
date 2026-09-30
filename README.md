<p align="center">
    <picture>
        <source height="128" media="(prefers-color-scheme: dark)" srcset="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-dark.png">
        <source height="128" media="(prefers-color-scheme: light)" srcset="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-light.png">
        <img height="128" alt="Picture from Block Page" src="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-light.png">
    </picture>
</p>
<h1 align="center">rshield</h1>
<p align="center">An Actix-web middleware for checking IP addresses to identify unglobal, malicious, and TOR connections. It provides the browser with a zero-click Proof of Work (PoW) task, or, if JavaScript is disabled, a one-click CAPTCHA image challenge.</p>

<p align="center">
    <a href="https://github.com/tn3w/rshield/actions/workflows/test.yml"><img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/tn3w/rshield/test.yml?style=flat-square&label=tests"></a>
    <a href="https://crates.io/crates/rshield"><img alt="Crates.io" src="https://img.shields.io/crates/v/rshield?style=flat-square"></a>
    <img alt="MSRV" src="https://img.shields.io/badge/rust-1.85%2B-orange?style=flat-square">
    <a href="https://github.com/tn3w/rshield/blob/main/LICENSE"><img alt="License" src="https://img.shields.io/github/license/tn3w/rshield?style=flat-square"></a>
    <a href="https://github.com/tn3w/rshield/issues"><img alt="Issues" src="https://img.shields.io/github/issues/tn3w/rshield?style=flat-square"></a>
    <a href="https://github.com/tn3w/rshield/stargazers"><img alt="Stars" src="https://img.shields.io/github/stars/tn3w/rshield?style=flat-square"></a>
</p>

## Install

```bash
cargo add rshield
```

Or from git:

```toml
[dependencies]
rshield = { git = "https://github.com/tn3w/rshield" }
```

Rust 1.85 or newer. A Redis server on `127.0.0.1:6379` is used for caching.

## How it works

1. Each request's peer IP is checked: unglobal ranges, known malicious addresses (ipapi), TOR exit nodes.
2. Results are cached in Redis.
3. Flagged clients get a challenge page in their `Accept-Language` (107 languages).
4. Browsers with JavaScript solve a zero-click proof-of-work task; without JavaScript, a one-click image CAPTCHA is shown.

## Quick start

```rust
use actix_web::{web, App, HttpResponse, HttpServer};
use rshield::{CookieMiddleware, RequestValidationMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(RequestValidationMiddleware)
            .service(web::scope("").route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello World!") }),
            ))
            .wrap(CookieMiddleware)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

`CookieMiddleware` must be registered after all services, `RequestValidationMiddleware` depends on it.

## Development

```bash
git clone https://github.com/tn3w/rshield.git
cd rshield
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test                  # unit tests
cargo test -- --ignored --skip benchmark   # network + Redis tests
```

Redis and OpenSSL headers are needed locally:

```bash
sudo apt-get install redis libssl-dev -y
redis-server --daemonize yes
```

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md).

## Attribution

- Logo icon: [Rust icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/rust)

## License

[Apache-2.0](https://github.com/tn3w/rshield/blob/main/LICENSE)
