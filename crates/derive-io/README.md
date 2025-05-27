# derive-io

A Rust crate that provides derive macros for implementing `AsyncRead` and `AsyncWrite` traits on structs and enums.

## Features

- Derive `AsyncRead` and `AsyncWrite` traits for structs and enums
- Support for both named and tuple structs
- Support for enums with multiple variants
- Support for split read/write streams

## Tokio

```rust
use tokio::net::*;
use derive_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
pub enum TokioStreams {
    Tcp(#[read] #[write] TcpStream),
    #[cfg(unix)]
    Unix(#[read] #[write] UnixStream),
    Split{ 
        #[read] read: tokio::net::tcp::OwnedReadHalf, 
        #[write] write: tokio::net::tcp::OwnedWriteHalf,
    },
}
```
