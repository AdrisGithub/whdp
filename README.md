# WHDP - Wizards hypermedia protocol parser

A library to parse the raw string
into a workable type and vice versa.

[![Latest version](https://img.shields.io/badge/crates.io-1.1.8-red)](https://crates.io/crates/whdp)
[![Documentation](https://docs.rs/log/badge.svg)](https://docs.rs/whdp)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_whdp&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_whdp)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_whdp&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_whdp)
[![Technical Debt](https://sonarcloud.io/api/project_badges/measure?project=AdrisGithub_whdp&metric=sqale_index)](https://sonarcloud.io/summary/new_code?id=AdrisGithub_whdp)

## Documentation:

* [`whdp`](https://docs.rs/whdp)

## Explanation:

Http is a text-based protocol. It follows a rather simple format

Requests:

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

Response:

```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
## Usage:

Import the library into your Cargo.toml

```toml
[dependencies]
whdp = "1.1.8"
```

Then just use the `TryRequest` trait to parse it to a `Request`

```rust
use std::io::Write;
use std::net::TcpListener;

use whdp::{Response, TryRequest};

fn main() {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    let mut resp = Response::default();
    for inco in server.incoming() {
        let mut inco = inco.unwrap();
        println!("{}", inco.try_to_request().unwrap()); // don't unwrap immediatly first look if there is an error
        let _ = inco.write_all(resp.to_string().as_bytes());
    }
}

```

And / Or if you want create a Response use the `ResponseBuilder` or the `resp_presets` module

```rust

use std::io::Write;
use std::net::TcpListener;

use whdp::{HttpVersion, ResponseBuilder};
use whdp::presets::ok;

fn main() {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();
    let resp = ResponseBuilder::new()
        .with_body("Hello, World".into())
        .with_status(ok())
        .with_version(HttpVersion::OnePointOne)
        .with_empty_headers()
        .build().unwrap();
    for inco in server.incoming() {
        let mut inco = inco.unwrap();
        let _ = inco.write_all(resp.to_string().as_bytes());
    }
}

```