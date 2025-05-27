#![doc = include_str!("../README.md")]

pub use derive_io_macros::*;

pub mod __support {
    #[doc(hidden)]
    pub use crate::__derive_io_async_read_parse as derive_io_async_read_parse;
    #[doc(hidden)]
    pub use crate::__derive_io_async_write_parse as derive_io_async_write_parse;
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_async_read_parse {
    ( $($input:tt)* ) => {
        $crate::__derive_impl!(__generate__ AsyncRead $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_async_write_parse {
    ( $($input:tt)* ) => {
        $crate::__derive_impl!(__generate__ AsyncWrite $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_impl {
    // Found, ignore additional attributes
    ( __find_struct__ #[read] [$self_type:path] [#[read] $(#[$_attr:meta])* $name:ident: $type:ty $(, $($fields:tt)*)?] -> $__next__:ident($self:ident $(, $arg:ident)*) ) => {
        $crate::__derive_impl!($__next__( $self $(, $arg)*) { let $self_type { $name, .. } = $self else { unreachable!() }; $name });
    };

    // Found, ignore additional attributes
    ( __find_struct__ #[write] [$self_type:path] [#[write] $(#[$_attr:meta])* $name:ident: $type:ty $(, $($fields:tt)*)?] -> $__next__:ident($self:ident $(, $arg:ident)*) ) => {
        $crate::__derive_impl!($__next__( $self $(, $arg)*) { let $self_type { $name, .. } = $self else { unreachable!() }; $name });
    };

    // Unknown attribute, skip it
    ( __find_struct__ #[$which:ident] [$self_type:path] [#[$_attr:meta] $($fields:tt)*] -> $($__next__:tt)* ) => {
        $crate::__derive_impl!(__find_struct__ #[$which] [$self_type] [$($fields)*] -> $($__next__)*);
    };

    // Unknown field, skip it
    ( __find_struct__ #[$which:ident] [$self_type:path] [$name:ident: $type:ty $(, $($fields:tt)*)?] -> $($__next__:tt)* ) => {
        $crate::__derive_impl!(__find_struct__ #[$which] [$self_type] [$($($fields)*)?] -> $($__next__)*);
    };

    ( __find_struct__ #[read] [$self_type:path] [] -> $($__next__:tt)* ) => {
        compile_error!("No #[read] field found");
    };
    ( __find_struct__ #[write] [$self_type:path] [] -> $($__next__:tt)* ) => {
        compile_error!("No #[write] field found");
    };


    // Found, ignore additional attributes
    ( __find_tuple__ #[read] [$self_type:path] $index:tt [#[read] $(#[$_attr:meta])* $type:ty $(, $($fields:tt)*)?] -> $__next__:ident($self:ident $(, $arg:ident)*) ) => {
        $crate::__derive_impl!($__next__( $self $(, $arg)*) $crate::__derive_impl!( __extract_index__ $self [$self_type] $index ));
    };

    // Found, ignore additional attributes
    ( __find_tuple__ #[write] [$self_type:path] $index:tt [#[write] $(#[$_attr:meta])* $type:ty $(, $($fields:tt)*)?] -> $__next__:ident($self:ident $(, $arg:ident)*) ) => {
        $crate::__derive_impl!($__next__( $self $(, $arg)*) $crate::__derive_impl!( __extract_index__ $self [$self_type] $index ));
    };

    // Unknown attribute, skip it
    ( __find_tuple__ #[$which:ident] [$self_type:path] $index:tt [#[$_attr:meta] $($fields:tt)*] -> $($__next__:tt)* ) => {
        $crate::__derive_impl!(__find_tuple__ #[$which] [$self_type] $index [$($fields)*] -> $($__next__)*);
    };

    // Unknown field, skip it
    ( __find_tuple__ #[$which:ident] [$self_type:path] ($($index:tt),*) [$type:ty $(, $($fields:tt)*)?] -> $($__next__:tt)* ) => {
        $crate::__derive_impl!(__find_tuple__ #[$which] [$self_type] (_ $(, $index)*) [$($($fields)*)?] -> $($__next__)*);
    };

    ( __find_tuple__ #[read] [$self_type:path] $index:tt [] -> $($__next__:tt)* ) => {
        compile_error!("No #[read] field found");
    };
    ( __find_tuple__ #[write] [$self_type:path] $index:tt [] -> $($__next__:tt)* ) => {
        compile_error!("No #[write] field found");
    };

    ( __extract_index__ $self:ident [$self_type:path] ($($index:tt),*) ) => {
        { let $self_type ($($index,)* x, ..) = $self else { unreachable!() }; x }
    };

    // tokio::io::AsyncRead::poll_read
    ( __generate_poll_read__ ($self:ident, $cx:ident, $buf:ident) $name:expr ) => {
        ::std::pin::Pin::new(&mut $name).poll_read($cx, $buf)
    };

    // tokio::io::AsyncWrite::poll_write
    ( __generate_poll_write__ ($self:ident, $cx:ident, $buf:ident) $name:expr ) => {
        ::std::pin::Pin::new(&mut $name).poll_write($cx, $buf)
    };

    // tokio::io::AsyncWrite::poll_flush
    ( __generate_poll_flush__ ($self:ident, $cx:ident) $name:expr ) => {
        ::std::pin::Pin::new(&mut $name).poll_flush($cx)
    };

    // tokio::io::AsyncWrite::poll_shutdown
    ( __generate_poll_shutdown__ ($self:ident, $cx:ident) $name:expr ) => {
        ::std::pin::Pin::new(&mut $name).poll_shutdown($cx)
    };

    // tokio::io::AsyncWrite::poll_write_vectored
    ( __generate_poll_write_vectored__ ($self:ident, $cx:ident, $bufs:ident) $name:expr ) => {
        ::std::pin::Pin::new(&mut $name).poll_write_vectored($cx, $bufs)
    };

    // tokio::io::AsyncWrite::is_write_vectored
    ( __generate_is_write_vectored__ ($self:ident) $name:expr ) => {
        ($name).is_write_vectored()
    };

    ( __generate__ AsyncRead $(#[$attr:meta])* $vis:vis enum $name:ident { $( $(#[$eattr:meta])* $field:ident $( ($($tuple:tt)*) )? $( {$($struct:tt)*} )? ),* $(,)?} ) => {
        impl ::tokio::io::AsyncRead for $name {
            fn poll_read(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                match this {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[read] [Self::$field] [$($struct)*] -> __generate_poll_read__(this, cx, buf))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[read] [Self::$field] () [$($tuple)*] -> __generate_poll_read__(this, cx, buf))
                            )?
                        }
                    )*
                }
            }
        }
    };

    ( __generate__ AsyncRead $(#[$attr:meta])* $vis:vis struct $name:ident { $($fields:tt)* } ) => {
        impl ::tokio::io::AsyncRead for $name {
            fn poll_read(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_struct__ #[read] [Self] [$($fields)*] -> __generate_poll_read__(this, cx, buf))
            }
        }
    };

    ( __generate__ AsyncRead $(#[$attr:meta])* $vis:vis struct $name:ident( $($fields:tt)* ); ) => {
        impl ::tokio::io::AsyncRead for $name {
            fn poll_read(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_tuple__ #[read] [Self] () [$($fields)*] -> __generate_poll_read__(this, cx, buf))
            }
        }
    };

    ( __generate__ AsyncWrite $(#[$attr:meta])* $vis:vis enum $name:ident { $( $(#[$eattr:meta])* $field:ident $( ($($tuple:tt)*) )? $( {$($struct:tt)*} )? ),* $(,)?} ) => {
        impl ::tokio::io::AsyncWrite for $name {
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                match this {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[write] [Self::$field] [$($struct)*] -> __generate_poll_write__(this, cx, buf))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[read] [Self::$field] () [$($tuple)*] -> __generate_poll_write__(this, cx, buf))
                            )?
                        }
                    )*
                }
            }

            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                match this {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[write] [Self::$field] [$($struct)*] -> __generate_poll_flush__(this, cx))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[write] [Self::$field] () [$($tuple)*] -> __generate_poll_flush__(this, cx))
                            )?
                        }
                    )*
                }
            }

            fn poll_shutdown(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                match this {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[write] [Self::$field] [$($struct)*] -> __generate_poll_shutdown__(this, cx))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[write] [Self::$field] () [$($tuple)*] -> __generate_poll_shutdown__(this, cx))
                            )?
                        }
                    )*
                }
            }

            fn is_write_vectored(&self) -> bool {
                match self {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[write] [Self::$field] [$($struct)*] -> __generate_is_write_vectored__(self))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[write] [Self::$field] () [$($tuple)*] -> __generate_is_write_vectored__(self))
                            )?
                        }
                    )*
                }
            }

            fn poll_write_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                match this {
                    $(
                        $name::$field {..} => {
                            $(
                                $crate::__derive_impl!(__find_struct__ #[write] [Self::$field] [$($struct)*] -> __generate_poll_write_vectored__(this, cx, bufs))
                            )?

                            $(
                                $crate::__derive_impl!(__find_tuple__ #[write] [Self::$field] () [$($tuple)*] -> __generate_poll_write_vectored__(this, cx, bufs))
                            )?
                        }
                    )*
                }
            }
        }
    };

    ( __generate__ AsyncWrite $(#[$attr:meta])* $vis:vis struct $name:ident { $($fields:tt)* } ) => {
        impl ::tokio::io::AsyncWrite for $name {
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_struct__ #[write] [Self] [$($fields)*] -> __generate_poll_write__(this, cx, buf))
            }

            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_struct__ #[write] [Self] [$($fields)*] -> __generate_poll_flush__(this, cx))
            }

            fn poll_shutdown(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_struct__ #[write] [Self] [$($fields)*] -> __generate_poll_shutdown__(this, cx))
            }

            fn is_write_vectored(&self) -> bool {
                $crate::__derive_impl!(__find_struct__ #[write] [Self] [$($fields)*] -> __generate_is_write_vectored__(self))
            }

            fn poll_write_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_struct__ #[write] [Self] [$($fields)*] -> __generate_poll_write_vectored__(this, cx, bufs))
            }
        }
    };

    ( __generate__ AsyncWrite $(#[$attr:meta])* $vis:vis struct $name:ident( $($fields:tt)* ); ) => {
        impl ::tokio::io::AsyncWrite for $name {
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_tuple__ #[write] [Self] () [$($fields)*] -> __generate_poll_write__(this, cx, buf))
            }

            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_tuple__ #[write] [Self] () [$($fields)*] -> __generate_poll_flush__(this, cx))
            }

            fn poll_shutdown(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_tuple__ #[write] [Self] () [$($fields)*] -> __generate_poll_shutdown__(this, cx))
            }

            fn is_write_vectored(&self) -> bool {
                $crate::__derive_impl!(__find_tuple__ #[write] [Self] () [$($fields)*] -> __generate_is_write_vectored__(self))
            }

            fn poll_write_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let mut this = unsafe { self.get_unchecked_mut() };
                $crate::__derive_impl!(__find_tuple__ #[write] [Self] () [$($fields)*] -> __generate_poll_write_vectored__(this, cx, bufs))
            }
        }
    };

    ( __generate__ fn $($input:tt)* ) => {
        const _: &str = stringify!($($input)*);
    };
}
