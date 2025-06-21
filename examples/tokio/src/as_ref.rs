use derive_io::{AsSocketDescriptor, AsyncRead, AsyncWrite};

use tokio::net::TcpStream;

/// [`Holder`] - Simple wrapper around TcpStream that implements AsRef and AsMut for delegation.
pub struct Holder(pub TcpStream);

impl AsRef<TcpStream> for Holder {
    fn as_ref(&self) -> &TcpStream {
        &self.0
    }
}

impl AsMut<TcpStream> for Holder {
    fn as_mut(&mut self) -> &mut TcpStream {
        &mut self.0
    }
}

/// [`AsRefStruct`] - Demonstrates wrapper types using AsRef/AsMut attributes for inner stream delegation.
#[derive(AsyncRead, AsyncWrite, AsSocketDescriptor)]
pub struct AsRefStruct {
    #[read(as_ref)]
    #[write(as_ref)]
    #[descriptor(as_ref)]
    stream: Holder,
}

impl AsRefStruct {
    pub fn new(stream: TcpStream) -> Self {
        AsRefStruct {
            stream: Holder(stream),
        }
    }
}
