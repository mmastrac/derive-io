use std::ops::{Deref, DerefMut};

use derive_io::{AsSocketDescriptor, AsyncRead, AsyncWrite};

use tokio::net::TcpStream;

/// [`Holder`] - Simple wrapper around TcpStream that implements AsRef and AsMut for delegation.
pub struct Holder(pub TcpStream);

impl Deref for Holder {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Holder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// [`DerefStruct`] - Demonstrates wrapper types using Deref/DerefMut attributes for inner stream delegation.
#[derive(AsyncRead, AsyncWrite, AsSocketDescriptor)]
pub struct DerefStruct {
    #[read(deref)]
    #[write(deref)]
    #[descriptor(deref)]
    stream: Holder,
}

impl DerefStruct {
    pub fn new(stream: TcpStream) -> Self {
        DerefStruct {
            stream: Holder(stream),
        }
    }
}
