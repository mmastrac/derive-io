use derive_io::{AsyncRead, AsyncWrite};

use tokio::net::TcpStream;

/// [`Override`] - Tests method override functionality for customizing derived trait behavior.
#[derive(AsyncRead, AsyncWrite)]
pub struct Override {
    #[read(poll_read=override_poll_read)]
    #[write(poll_write=override_poll_write, poll_flush=override_poll_flush, poll_shutdown=override_poll_shutdown)]
    stream: TcpStream,
}

/// Override function for `poll_read` that always returns `Pending`.
///
/// This is a simple example that shows how to intercept read operations.
/// In a real application, you might add logging, metrics, or custom error handling.
fn override_poll_read<S: tokio::io::AsyncRead>(
    _stm: std::pin::Pin<&mut S>,
    _cx: &mut std::task::Context<'_>,
    _buf: &mut tokio::io::ReadBuf<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}

/// Override function for `poll_write` that always returns `Pending`.
///
/// This is a simple example that shows how to intercept write operations.
/// In a real application, you might add logging, metrics, or custom error handling.
fn override_poll_write<S: tokio::io::AsyncWrite>(
    _stm: std::pin::Pin<&mut S>,
    _cx: &mut std::task::Context<'_>,
    _buf: &[u8],
) -> std::task::Poll<std::io::Result<usize>> {
    std::task::Poll::Pending
}

/// Override function for `poll_flush` that always returns `Pending`.
///
/// This is a simple example that shows how to intercept flush operations.
/// In a real application, you might add logging, metrics, or custom error handling.
fn override_poll_flush<S: tokio::io::AsyncWrite>(
    _stm: std::pin::Pin<&mut S>,
    _cx: &mut std::task::Context<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}

/// Override function for `poll_shutdown` that always returns `Pending`.
///
/// This is a simple example that shows how to intercept shutdown operations.
/// In a real application, you might add logging, metrics, or custom error handling.
fn override_poll_shutdown<S: tokio::io::AsyncWrite>(
    _stm: std::pin::Pin<&mut S>,
    _cx: &mut std::task::Context<'_>,
) -> std::task::Poll<std::io::Result<()>> {
    std::task::Poll::Pending
}
