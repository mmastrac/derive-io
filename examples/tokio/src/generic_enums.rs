use derive_io::{AsFileDescriptor, AsyncRead, AsyncWrite};

/// [`EnumGeneric`] - Tests generic enums with complex trait bounds and multiple stream variants.
#[derive(AsyncRead, AsyncWrite, AsFileDescriptor)]
pub enum EnumGeneric<T, S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite,
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    T(
        #[descriptor]
        #[read]
        #[write]
        T,
    ),
    S(
        #[descriptor]
        #[read]
        #[write]
        S,
    ),
}

impl<T, S> EnumGeneric<T, S>
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite,
    T: tokio::io::AsyncRead + tokio::io::AsyncWrite,
{
    #[expect(unused)]
    pub fn new_t(t: T) -> Self {
        Self::T(t)
    }

    pub fn new_s(s: S) -> Self {
        Self::S(s)
    }
}
