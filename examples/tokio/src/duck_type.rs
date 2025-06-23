use std::{
    io::IoSlice,
    pin::Pin,
    task::{Context, Poll},
};

use derive_io::{AsyncRead, AsyncWrite};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    net::TcpStream,
};

/// Implements [`AsyncRead`] and [`AsyncWrite`] for a [`TcpStream`] via duck typing.
///
/// This is useful for when you want to implement [`AsyncRead`] and [`AsyncWrite`] for a type that
/// implements poll_read and poll_write, but not as the proper trait.
///
/// Duck typing supports coercing of the receiver to the correct type.
#[derive(AsyncRead, AsyncWrite)]
#[duck(
    poll_read,
    poll_write,
    poll_flush,
    poll_shutdown,
    poll_write_vectored,
    is_write_vectored
)]
#[read(duck)]
#[write(duck)]
pub struct DuckType {
    inner: TcpStream,
}

#[deny(unused)]
impl DuckType {
    pub fn new(inner: TcpStream) -> Self {
        Self { inner }
    }

    fn poll_read(
        &mut self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }

    fn poll_write(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }

    fn poll_write_vectored(
        &mut self,
        cx: &mut Context<'_>,
        bufs: &[IoSlice<'_>],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner.is_write_vectored()
    }
}
