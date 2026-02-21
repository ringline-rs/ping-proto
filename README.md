# ping-proto

Simple ASCII PING/PONG protocol implementation.

## Features

- PING request encoding (`PING\r\n`)
- PONG response parsing (both `PONG\r\n` and RESP-style `+PONG\r\n`)
- Error response parsing (`-ERR ...\r\n`)

## Usage

```toml
[dependencies]
ping-proto = "0.0.1"
```

```rust
use ping_proto::{Request, Response};

// Encode a PING request
let mut buf = [0u8; 16];
let len = Request::Ping.encode(&mut buf);
assert_eq!(&buf[..len], b"PING\r\n");

// Parse a PONG response
let data = b"PONG\r\n";
let (response, consumed) = Response::parse(data).unwrap();
assert_eq!(response, Response::Pong);
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.
