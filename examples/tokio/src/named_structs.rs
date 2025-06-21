use derive_io::{AsyncRead, AsyncWrite};

use tokio::net::TcpStream;

/// [`NamedStruct`] - Tests basic named structs with single stream field and phantom data.
#[derive(AsyncRead, AsyncWrite)]
pub struct NamedStruct {
    #[read]
    #[write]
    tcp: TcpStream,
    pub(crate) _phantom: std::marker::PhantomData<()>,
}

impl NamedStruct {
    pub fn new(tcp: TcpStream) -> Self {
        NamedStruct {
            tcp,
            _phantom: std::marker::PhantomData,
        }
    }
}

/// [`ReadWriteStruct`] - Tests structs with separate read and write stream halves.
#[derive(AsyncRead, AsyncWrite)]
pub struct ReadWriteStruct {
    #[read]
    reader: tokio::net::tcp::OwnedReadHalf,
    #[write]
    writer: tokio::net::tcp::OwnedWriteHalf,
}

impl ReadWriteStruct {
    pub fn new(
        reader: tokio::net::tcp::OwnedReadHalf,
        writer: tokio::net::tcp::OwnedWriteHalf,
    ) -> Self {
        ReadWriteStruct { reader, writer }
    }
}
