use std::{pin::Pin, task::{Context, Poll}};

use derive_io::{AsyncRead, AsyncWrite};
use tokio::{io::{AsyncRead, AsyncWrite, ReadBuf}, net::TcpStream};

#[derive(AsyncRead, AsyncWrite)]
#[read(duck)]
#[write(duck)]
pub struct DuckType {
  inner: TcpStream,
}

impl DuckType {
    pub fn new(inner: TcpStream) -> Self {
        Self { inner }
    }

    pub fn poll_read(&mut self, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }

    pub fn poll_write(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    pub fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    pub fn poll_shutdown(&mut self, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}