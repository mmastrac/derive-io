use derive_io::{AsyncRead, AsyncWrite};

use tokio::net::TcpStream;

/// [`Generic`] - Tests generic structs with inline trait bounds on stream parameters.
#[derive(AsyncRead, AsyncWrite)]
pub struct Generic<S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin> {
    #[read]
    #[write]
    stream: S,
}

impl<S> Generic<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

/// [`Generic2`] - Tests generic structs with where clause trait bounds for stream parameters.
#[derive(AsyncRead, AsyncWrite)]
pub struct Generic2<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    #[read]
    #[write]
    stream: S,
}

impl<S> Generic2<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

/// [`GenericUnrelated`] - Tests generic structs with unrelated type parameters alongside stream fields.
#[derive(AsyncRead, AsyncWrite)]
pub struct GenericUnrelated<T, S> {
    #[read]
    #[write]
    stream: S,
    #[allow(unused)]
    t: T,
}

impl<T, S: Unpin> GenericUnrelated<T, S> {
    pub fn new(stream: S, t: T) -> Self {
        Self { stream, t }
    }
}

/// [`GenericUnrelated2`] - Tests generic structs with concrete stream types and unrelated generic parameters.
#[derive(AsyncRead, AsyncWrite)]
pub struct GenericUnrelated2<T> {
    #[read]
    #[write]
    stream: TcpStream,
    #[allow(unused)]
    t: T,
}
