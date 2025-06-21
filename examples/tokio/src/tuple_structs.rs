use std::marker::PhantomPinned;

use derive_io::{AsSocketDescriptor, AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

#[cfg(unix)]
use tokio::net::UnixStream;

/// [`TupleStruct`] - Tests tuple structs with mixed field types where only one field is a stream.
#[derive(AsyncRead, AsyncWrite, AsSocketDescriptor)]
pub struct TupleStruct(
    #[allow(unused)] u8,
    #[allow(unused)] u8,
    #[read]
    #[write]
    #[descriptor]
    TcpStream,
);

impl TupleStruct {
    pub fn new(field1: u8, field2: u8, stream: TcpStream) -> Self {
        TupleStruct(field1, field2, stream)
    }
}

/// [`TupleStructPin`] - Tests tuple structs with pinned fields for async operations.
#[derive(AsyncRead, AsyncWrite)]
pub struct TupleStructPin(
    #[read]
    #[write]
    pub TcpStream,
    pub(crate) PhantomPinned,
);

/// [`TupleUnixStruct`] - Tests Unix-specific tuple structs with conditional compilation.
#[derive(AsyncRead, AsyncWrite)]
#[cfg(unix)]
pub struct TupleUnixStruct(
    #[read]
    #[write]
    UnixStream,
);

#[cfg(unix)]
impl TupleUnixStruct {
    #[expect(unused)]
    pub fn new(stream: UnixStream) -> Self {
        TupleUnixStruct(stream)
    }
}
