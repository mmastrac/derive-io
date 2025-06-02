# derive-io

A Rust crate that provides derive macros for implementing sync and async I/O traits on structs and enums (including Tokio, stdlib I/O, and more).

## Supported traits

- `#[derive(Read)]`: `std::io::Read`
- `#[derive(Write)]`: `std::io::Write`
- `#[derive(AsyncRead)]`: `tokio::io::AsyncRead`
- `#[derive(AsyncWrite)]`: `tokio::io::AsyncWrite`
- `#[derive(AsDescriptor)]`:
    - `std::os::{AsFd, AsRawFd}`
    - `std::os::windows::io::{AsHandle, AsRawHandle, AsSocket, AsRawSocket}`

## Features

- Derive I/O traits for structs and enums
- Support for both named and tuple structs
- Support for enums with multiple variants
- Support for split read/write streams
- Support for generic types
- Individual methods can be overridden with custom implementations

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

Generic types are supported. The generated implementations will automatically
add a `where` clause to the impl block for each stream type.

```rust
use derive_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
pub struct Generic<S> { // where S: AsyncRead + AsyncWrite
    #[read]
    #[write]
    stream: S,
}
```

Override one method in the write implementation:

```rust
use derive_io::{AsyncRead, AsyncWrite};

#[derive(AsyncRead, AsyncWrite)]
pub struct Override {
    #[read]
    #[write(poll_write=override_poll_write)]
    stream: tokio::net::TcpStream,
}

pub fn override_poll_write(
    stm: std::pin::Pin<&mut tokio::net::TcpStream>,
    cx: &mut std::task::Context<'_>,
    buf: &[u8],
) -> std::task::Poll<std::io::Result<usize>> {
    todo!()
}
```
