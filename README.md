<p align="center">
    <picture>
        <source height="128" media="(prefers-color-scheme: dark)" srcset="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-dark.png">
        <source height="128" media="(prefers-color-scheme: light)" srcset="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-light.png">
        <img height="128" alt="Picture from Block Page" src="https://github.com/tn3w/rshield/releases/download/logo/rusty-logo-light.png">
    </picture>
</p>
<h1 align="center">rshield</h1>
<p align="center">An Actix-web middleware for checking IP addresses to identify unglobal, malicious, and TOR connections. It provides the browser with a zero-click Proof of Work (PoW) task, or, if JavaScript is disabled, a one-click CAPTCHA image challenge.</p>

## 🚀 Installing
Just add the following line to `[dependencies]` in your `Cargo.toml` file:
```toml
[dependencies]
rshield = { git = "https://github.com/tn3w/rshield" }
```

And then integrate the MiddleWare into your Actix-web app by registering it:

```rust
use actix_web::{web, App, HttpServer, HttpResponse};
use rshield::{RequestValidationMiddleware, CookieMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.wrap(RateLimitMiddleware) // Coming soon
            .wrap(RequestValidationMiddleware) // Register the request validation middleware
            .service(
                web::scope("")
                    .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World!") }))
            )
            .wrap(CookieMiddleware)  // Register the cookie middleware after all services (required for RequestValidationMiddleware)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## Building
1. Install Rust using rust-up (optional): 
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Clone the git project:
    ```bash
    git clone https://github.com/tn3w/rshield.git
    ```

3. Move into the project folder:
    ```bash
    cd rshield
    ```

4. Setup Redis
    ```bash
    sudo apt-get update
    sudo apt-get install redis -y
    sudo systemctl enable redis-server.service
    sudo systemctl start redis-server.service
    ```

5. Install libssl-dev:
    ```bash
    sudo apt-get update
    sudo apt-get install libssl-dev -y
    ``` 

6. Build rshield
    ```bash
    cargo build --release
    ```

### Attribution
- Logo icon: [Rust icons created by Freepik - Flaticon](https://www.flaticon.com/free-icons/rust)
