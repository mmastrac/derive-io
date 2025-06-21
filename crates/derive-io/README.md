# derive-io

[![Crates.io](https://img.shields.io/crates/v/derive-io.svg)](https://crates.io/crates/derive-io)
[![Documentation](https://docs.rs/derive-io/badge.svg)](https://docs.rs/derive-io)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md)
[![Build Status](https://github.com/mmastrac/derive-io/workflows/CI/badge.svg)](https://github.com/mmastrac/derive-io/actions)

A Rust crate that provides derive macros for implementing sync and async I/O traits on structs and enums (including Tokio, stdlib I/O, and more).

## Supported traits

- `#[derive(Read)]`: `std::io::Read`
- `#[derive(Write)]`: `std::io::Write`
- `#[derive(AsyncRead)]`: `tokio::io::AsyncRead`
- `#[derive(AsyncWrite)]`: `tokio::io::AsyncWrite`
- `#[derive(AsFileDescriptor)]`:
    - `std::os::fd::{AsFd, AsRawFd}`
    - `std::os::windows::io::{AsHandle, AsRawHandle}`
- `#[derive(AsSocketDescriptor)]`:
    - `std::os::fd::{AsFd, AsRawFd}`
    - `std::os::windows::io::{AsSocket, AsRawSocket}`

## Features

- Derive I/O traits for structs and enums
- Support for both named and tuple structs
- Support for enums with multiple variants
- Support for split read/write streams
- Support for generic types
- Individual methods can be overridden with custom implementations
- Support for `as_ref` or `deref` attribute on fields to delegate to the inner type
  - Note: for traits requiring a pinned-self (ie: async read/write), the holder
    type must be `Unpin`!

## Tokio

```rust
use tokio::net::*;
use derive_io::{AsyncRead, AsyncWrite, AsSocketDescriptor};

#[derive(AsyncRead, AsyncWrite, AsSocketDescriptor)]
pub enum TokioStreams {
    Tcp(#[read] #[write] #[descriptor] TcpStream),
    #[cfg(unix)]
    Unix(#[read] #[write] #[descriptor] UnixStream),
    Split{ 
        #[read] #[descriptor(as_ref)] read: tokio::net::tcp::OwnedReadHalf, 
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
