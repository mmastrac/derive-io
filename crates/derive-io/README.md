# derive-io

[![Crates.io](https://img.shields.io/crates/v/derive-io.svg)](https://crates.io/crates/derive-io)
[![Documentation](https://docs.rs/derive-io/badge.svg)](https://docs.rs/derive-io)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README.md)
[![Build Status](https://github.com/mmastrac/derive-io/workflows/CI/badge.svg)](https://github.com/mmastrac/derive-io/actions)

A Rust crate that provides derive macros for implementing sync and async I/O traits on structs and enums (including Tokio, stdlib I/O, and more).

## Supported traits

- `#[derive(Read)]`: [`std::io::Read`]
- `#[derive(BufRead)]`: [`std:io::BufRead`]
- `#[derive(Write)]`: [`std::io::Write`]
- `#[derive(AsyncRead)]`: [`tokio::io::AsyncRead`]
- `#[derive(AsyncWrite)]`: [`tokio::io::AsyncWrite`]
- `#[derive(AsFileDescriptor)]`:
    - `std::os::fd::{AsFd, AsRawFd}`
    - `std::os::windows::io::{AsHandle, AsRawHandle}`
- `#[derive(AsSocketDescriptor)]`:
    - `std::os::fd::{AsFd, AsRawFd}`
    - `std::os::windows::io::{AsSocket, AsRawSocket}`

## Features

- Derive most common I/O traits for structs and enums
- Support for both named and tuple structs
- Support for enums with multiple variants
- Support for split read/write streams (ie: two fields provide the read/write halves)
- Support for generic types
- Support for duck typing (ie: implementing traits using a method with a "similar" interface)
- Individual methods can be overridden with custom implementations
- Support for `as_ref` or `deref` attribute on fields to delegate to the inner type
  - Note: for traits requiring a pinned-self (ie: async read/write), the holder
    type and the outer type must both be `Unpin`!
- Pin safety: internal pin projection never allows a `&mut` to escape, thus
  upholding any `Pin` guarantees.

## `as_ref`/`deref` delegation

Most I/O traits are implemented correctly for `Box<dyn (trait)>` (that is: they
are implemented for `Box<T> where T: ?Sized`). However, some traits have
accidental or intentional additional `Sized` requirements which prevent
automatic delegation from working. Generally this is only required for
`AsFileDescriptor` and `AsSocketDescriptor`, as most other traits are
implemented for themselves on `Box<T> where T: Trait + ?Sized`.

To uphold `Pin` safety guarantees, both the inner and outer types must be
`Unpin`.

The `as_ref` attribute can be used to delegate to the inner type's unwrapped
type `as_ref`/`as_mut` implementation. The `deref` attribute can be used to
delegate to the inner type's pointee via `Deref`/`DerefMut`.

```rust
use derive_io::{AsyncRead, AsyncWrite, AsFileDescriptor};

#[cfg(unix)]
trait MyStream: tokio::io::AsyncRead + tokio::io::AsyncWrite 
    + std::os::fd::AsFd + std::os::fd::AsRawFd + Unpin {}
#[cfg(windows)]
trait MyStream: tokio::io::AsyncRead + tokio::io::AsyncWrite 
    + std::os::windows::io::AsHandle + std::os::windows::io::AsRawHandle + Unpin {}

#[derive(AsyncRead, AsyncWrite, AsFileDescriptor)]
pub struct DelegateAsRef {
    #[read]
    #[write]
    // This won't work with #[descriptor] because `AsRawFd` is not implemented for
    // `Box<dyn AsRawFd>`.
    #[descriptor(as_ref)]
    stream: Box<dyn MyStream>,
}

#[derive(AsyncRead, AsyncWrite, AsFileDescriptor)]
pub struct DelegateDeref {
    #[read]
    #[write]
    // This won't work with #[descriptor] because `AsRawFd` is not implemented for
    // `Box<dyn AsRawFd>`. This won't work with #[descriptor(as_ref)] because
    // `as_ref` and `as_mut` on a `Pin` gives you a `Box`.
    #[descriptor(deref)]
    stream: std::pin::Pin<Box<dyn MyStream>>,
}
```

## Overrides

`#[read(<function>=<override>)]` and `#[write(<function>=<override>)]` may be
specified to redirect a method to a custom implementation.

## `duck` delegation

`duck` delegation uses non-trait `impl` methods defined on a type to implement
the trait (i.e. "duck typing"). This is useful for when you want to implement a
trait for a type that doesn't implement the trait directly, but has methods that
are similar to the trait

`#[read(duck)]` and `#[write(duck)]` may be specified on the outer type or an
inner field.

When using `duck` delegation, specify the methods to delegate to using the
`#[duck(...)]` attribute:

```rust
use derive_io::{AsyncRead, AsyncWrite};
use std::task::{Context, Poll};

#[derive(AsyncRead, AsyncWrite)]
#[duck(poll_read, poll_write, poll_flush, poll_shutdown, poll_write_vectored, is_write_vectored)]
#[read(duck)]
#[write(duck)]
pub struct DuckType {
    inner: tokio::net::TcpStream,
}

impl DuckType {
    pub fn poll_read(
        &mut self,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        todo!()
    }

    // ... poll_write, poll_flush, poll_shutdown, poll_write_vectored, is_write_vectored, etc
}
```

# Examples

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
