//! Tokio examples demonstrating various derive-io patterns.
//!
//! This example shows how to use derive-io with tokio's async I/O types.
//! Each example demonstrates different patterns and use cases for the derive macros.

mod as_ref;
mod complex_stream;
mod generic_enums;
mod generic_structs;
mod named_structs;
mod override_example;
mod tokio_streams;
mod tuple_structs;

use std::net::SocketAddr;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use as_ref::AsRefStruct;
use named_structs::{NamedStruct, ReadWriteStruct};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_streams::TokioStreams;
use tuple_structs::TupleStruct;

use crate::complex_stream::ComplexStream;
use crate::generic_enums::EnumGeneric;
use crate::generic_structs::{Generic, Generic2, GenericUnrelated};

pub async fn test_stream(test_name: &str, mut stream: impl AsyncRead + AsyncWrite + Unpin) {
    eprint!("test {} ... ", test_name);
    let mut buf = Vec::new();
    stream.write_all(&buf).await.unwrap();
    stream.read_to_end(&mut buf).await.unwrap();
    stream.flush().await.unwrap();
    stream.shutdown().await.unwrap();
    eprintln!(" OK");
}

pub async fn make_tcp_stream(address: SocketAddr) -> TcpStream {
    TcpStream::connect(address).await.unwrap()
}

#[tokio::main]
pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    eprintln!("Listening onaddress: {}", address);

    let _handle = tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            socket.write_all(b"Hello, world!").await.unwrap();
            socket.shutdown().await.unwrap();
            let mut buf = Vec::new();
            socket.read_to_end(&mut buf).await.unwrap();
        }
    });

    let stm = TokioStreams::Tcp(make_tcp_stream(address).await);
    test_stream("TokioStreams enum", stm).await;

    let stm = TupleStruct::new(0, 0, make_tcp_stream(address).await);
    test_stream("TupleStruct", stm).await;

    let stm = NamedStruct::new(make_tcp_stream(address).await);
    test_stream("NamedStruct", stm).await;

    let (reader, writer) = make_tcp_stream(address).await.into_split();
    let stm = ReadWriteStruct::new(reader, writer);
    test_stream("ReadWriteStruct with split halves", stm).await;

    let stm = AsRefStruct::new(make_tcp_stream(address).await);
    test_stream("AsRefStruct with wrapper", stm).await;

    let stm = GenericUnrelated::new(make_tcp_stream(address).await, 0);
    test_stream("GenericUnrelated", stm).await;

    let stm: EnumGeneric<tokio::net::TcpStream, tokio::net::TcpStream> =
        EnumGeneric::new_s(make_tcp_stream(address).await);
    test_stream("EnumGeneric", stm).await;

    let stm: Generic<tokio::net::TcpStream> = Generic::new(make_tcp_stream(address).await);
    test_stream("Generic", stm).await;

    let stm: Generic2<tokio::net::TcpStream> = Generic2::new(make_tcp_stream(address).await);
    test_stream("Generic2", stm).await;

    let stm: ComplexStream<'_, _, ()> = ComplexStream::A(make_tcp_stream(address).await, None);
    test_stream("ComplexStream #1", stm).await;

    let stm: ComplexStream<_> = ComplexStream::B(
        GenericUnrelated::new(make_tcp_stream(address).await, ()),
        None,
    );
    test_stream("ComplexStream #2", stm).await;

    eprintln!();
    eprintln!("All tests completed successfully!");
    eprintln!();
}
