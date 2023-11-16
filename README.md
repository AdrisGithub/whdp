# WHDP - Wizards hypermedia protocol parser

A library to parse the raw string
into a workable type and vice versa.

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
## Example:

```rust
use std::io::Write;
use std::net::TcpListener;

use whdp::request::TryRequest;
use whdp::response::Response;

fn main() {
    let server = TcpListener::bind("0.0.0.0:6969").unwrap();
    let mut resp = Response::default();
    for inco in server.incoming() {
        let mut inco = inco.unwrap();
        println!("{}", inco.try_to_request().unwrap());
        let _ = inco.write_all(resp.to_string().as_bytes());
    }
}

```

