use std::marker::PhantomPinned;
use std::net::SocketAddr;

use derive_io::{AsyncRead, AsyncWrite};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[cfg(unix)]
use tokio::net::UnixStream;

#[derive(AsyncRead, AsyncWrite)]
pub enum TokioStreams {
    Tcp(
        #[read]
        #[write]
        TcpStream,
    ),
    #[cfg(unix)]
    Unix(
        #[read]
        #[write]
        UnixStream,
    ),
    #[cfg(windows)]
    Windows(
        #[read]
        #[write]
        tokio::net::windows::NamedPipeClient,
    ),
    Split {
        #[read]
        read: tokio::net::tcp::OwnedReadHalf,
        #[write]
        write: tokio::net::tcp::OwnedWriteHalf,
    },
}

#[derive(AsyncRead, AsyncWrite)]
pub struct TupleStruct(
    u8,
    u8,
    #[read]
    #[write]
    TcpStream,
);

#[derive(AsyncRead, AsyncWrite)]
pub struct TupleStructPin(
    #[read]
    #[write]
    TcpStream,
    PhantomPinned,
);

#[derive(AsyncRead, AsyncWrite)]
#[cfg(unix)]
pub struct TupleUnixStruct(
    #[read]
    #[write]
    UnixStream,
);

#[derive(AsyncRead, AsyncWrite)]
pub struct NamedStruct {
    #[read]
    #[write]
    tcp: TcpStream,
}

#[derive(AsyncRead, AsyncWrite)]
pub struct ReadWriteStruct {
    #[read]
    reader: tokio::net::tcp::OwnedReadHalf,
    #[write]
    writer: tokio::net::tcp::OwnedWriteHalf,
}

async fn make_tcp_stream(address: SocketAddr) -> TcpStream {
    TcpStream::connect(address).await.unwrap()
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    eprintln!("address: {}", address);

    let handle = tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(b"Hello, world!").await.unwrap();
            socket.shutdown().await.unwrap();
            let mut buf = Vec::new();
            socket.read_to_end(&mut buf).await.unwrap();
        }
    });

    let mut buf = Vec::new();

    eprintln!("Test 1");
    let mut stm = TokioStreams::Tcp(make_tcp_stream(address).await);
    stm.write_all(&buf).await.unwrap();
    stm.read_to_end(&mut buf).await.unwrap();
    stm.flush().await.unwrap();
    stm.shutdown().await.unwrap();

    eprintln!("Test 2");
    let mut stm = TupleStruct(0, 0, make_tcp_stream(address).await);
    stm.write_all(&buf).await.unwrap();
    stm.read_to_end(&mut buf).await.unwrap();
    stm.flush().await.unwrap();
    stm.shutdown().await.unwrap();

    eprintln!("Test 3");
    let mut stm = NamedStruct {
        tcp: make_tcp_stream(address).await,
    };
    stm.write_all(&buf).await.unwrap();
    stm.read_to_end(&mut buf).await.unwrap();
    stm.flush().await.unwrap();
    stm.shutdown().await.unwrap();

    eprintln!("Test 4");
    let (reader, writer) = make_tcp_stream(address).await.into_split();
    let mut stm = ReadWriteStruct { reader, writer };
    stm.write_all(&buf).await.unwrap();
    stm.read_to_end(&mut buf).await.unwrap();
    stm.flush().await.unwrap();
    stm.shutdown().await.unwrap();
}
