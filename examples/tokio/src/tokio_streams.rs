use derive_io::{AsSocketDescriptor, AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

#[cfg(unix)]
use tokio::net::UnixStream;

/// [`TokioStreams`] - Tests multi-variant enums with different stream types and platform-specific variants.
#[derive(AsyncRead, AsyncWrite, AsSocketDescriptor)]
#[allow(unused)]
pub enum TokioStreams {
    Tcp(
        #[read]
        #[write]
        #[descriptor]
        TcpStream,
    ),
    #[cfg(unix)]
    Unix(
        #[read]
        #[write]
        #[descriptor]
        UnixStream,
    ),
    #[cfg(windows)]
    Windows(
        #[read]
        #[write]
        #[descriptor]
        tokio::net::windows::named_pipe::NamedPipeClient,
    ),
    Split {
        #[read]
        #[descriptor(as_ref)]
        read: tokio::net::tcp::OwnedReadHalf,
        #[write]
        write: tokio::net::tcp::OwnedWriteHalf,
    },
}
