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
pub enum EnumGeneric<T, S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite,
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    T(
        #[read]
        #[write]
        T,
    ),
    S(
        #[read]
        #[write]
        S,
    ),
}

#[derive(AsyncRead, AsyncWrite)]
pub struct TupleStruct(
    #[allow(unused)] u8,
    #[allow(unused)] u8,
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

#[derive(AsyncRead, AsyncWrite)]
pub struct Generic<S: tokio::io::AsyncRead + tokio::io::AsyncWrite> {
    #[read]
    #[write]
    stream: S,
}

#[derive(AsyncRead, AsyncWrite)]
pub struct Generic2<S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    #[read]
    #[write]
    stream: S,
}

#[derive(AsyncRead, AsyncWrite)]
pub struct GenericUnrelated<T, S> {
    #[read]
    #[write]
    stream: S,
    #[allow(unused)]
    t: T,
}

#[derive(AsyncRead, AsyncWrite)]
pub struct GenericUnrelated2<T> {
    #[read]
    #[write]
    stream: TcpStream,
    #[allow(unused)]
    t: T,
}

#[derive(AsyncRead, AsyncWrite)]
pub struct Override {
    #[read(poll_read=override_poll_read)]
    #[write(poll_write=override_poll_write, poll_flush=override_poll_flush, poll_shutdown=override_poll_shutdown)]
    stream: TcpStream,
}

fn override_poll_read<S: tokio::io::AsyncRead>(
    stm: std::pin::Pin<&mut S>,
    cx: &mut std::task::Context<'_>,
    buf: &mut tokio::io::ReadBuf<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}

fn override_poll_write<S: tokio::io::AsyncWrite>(
    stm: std::pin::Pin<&mut S>,
    cx: &mut std::task::Context<'_>,
    buf: &[u8],
) -> std::task::Poll<std::io::Result<usize>> {
    std::task::Poll::Pending
}

fn override_poll_flush<S: tokio::io::AsyncWrite>(
    stm: std::pin::Pin<&mut S>,
    cx: &mut std::task::Context<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}

fn override_poll_shutdown<S: tokio::io::AsyncWrite>(
    stm: std::pin::Pin<&mut S>,
    cx: &mut std::task::Context<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}

#[derive(AsyncRead, AsyncWrite, derive_more::Debug)]
enum ComplexStream<S: std::fmt::Debug, D: std::any::Any> {
    #[debug("hi")]
    A(
        #[read]
        #[write]
        S,
        Option<D>,
    ),
    #[debug("hi")]
    B(
        #[read]
        #[write]
        GenericUnrelated<S, D>,
        Option<D>,
    ),
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    eprintln!("address: {}", address);

    let _handle = tokio::spawn(async move {
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
