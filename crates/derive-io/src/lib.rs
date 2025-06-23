#![doc = include_str!("../README.md")]

#[cfg(feature = "std")]
pub use derive_io_macros::{AsFileDescriptor, AsSocketDescriptor, BufRead, Read, Write};

#[cfg(feature = "tokio")]
pub use derive_io_macros::{AsyncRead, AsyncWrite};

#[doc(hidden)]
pub mod __support {
    pub use crate::__derive_io_as_file_descriptor_parse as derive_io_as_file_descriptor_parse;
    pub use crate::__derive_io_as_socket_descriptor_parse as derive_io_as_socket_descriptor_parse;
    pub use crate::__derive_io_async_read_parse as derive_io_async_read_parse;
    pub use crate::__derive_io_async_write_parse as derive_io_async_write_parse;
    pub use crate::__derive_io_bufread_parse as derive_io_bufread_parse;
    pub use crate::__derive_io_read_parse as derive_io_read_parse;
    pub use crate::__derive_io_write_parse as derive_io_write_parse;
    pub use derive_io_macros::{
        extract_meta, find_annotated, find_annotated_multi, if_meta, repeat_in_parenthesis,
        type_has_generic,
    };

    // We need a guaranteed valid implementation of this trait for each trait we support.
    #[doc(hidden)]
    pub trait IsSupported<T> {
        type Type;
    }

    impl IsSupported<&'static dyn std::io::Read> for () {
        type Type = Box<dyn std::io::Read + Unpin>;
    }

    impl IsSupported<&'static dyn std::io::BufRead> for () {
        type Type = Box<dyn std::io::BufRead + Unpin>;
    }

    impl IsSupported<&'static dyn std::io::Write> for () {
        type Type = Box<dyn std::io::Write + Unpin>;
    }

    #[cfg(feature = "tokio")]
    impl IsSupported<&'static dyn tokio::io::AsyncRead> for () {
        type Type = Box<dyn tokio::io::AsyncRead + Unpin>;
    }

    #[cfg(feature = "tokio")]
    impl IsSupported<&'static dyn tokio::io::AsyncWrite> for () {
        type Type = Box<dyn tokio::io::AsyncWrite + Unpin>;
    }

    #[cfg(unix)]
    impl IsSupported<&'static dyn std::os::fd::AsFd> for () {
        type Type = Box<dyn std::os::fd::AsFd + Unpin>;
    }

    #[cfg(unix)]
    // This one has buggy bounds in the rust stdlib
    impl IsSupported<&'static dyn std::os::fd::AsRawFd> for () {
        type Type = std::os::fd::RawFd;
    }

    #[cfg(windows)]
    impl IsSupported<&'static dyn std::os::windows::io::AsHandle> for () {
        type Type = std::os::windows::io::OwnedHandle;
    }

    #[cfg(windows)]
    impl IsSupported<&'static dyn std::os::windows::io::AsRawHandle> for () {
        type Type = std::os::windows::io::OwnedHandle;
    }

    #[cfg(windows)]
    impl IsSupported<&'static dyn std::os::windows::io::AsSocket> for () {
        type Type = std::os::windows::io::OwnedSocket;
    }

    #[cfg(windows)]
    impl IsSupported<&'static dyn std::os::windows::io::AsRawSocket> for () {
        type Type = std::os::windows::io::OwnedSocket;
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_read_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ Read $generics ($($where)*) read $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_bufread_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ BufRead $generics ($($where)*) read $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_write_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ Write $generics ($($where)*) write $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_async_read_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ AsyncRead $generics ($($where)*) read $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_async_write_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ AsyncWrite $generics ($($where)*) write $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_as_file_descriptor_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ AsFileDescriptor $generics ($($where)*) descriptor $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_io_as_socket_descriptor_parse {
    ( ($($input:tt)*) $generics:tt ($($where:tt)*) ) => {
        const _: &str = stringify!( generics = $generics, where = $($where)* );
        $crate::__derive_impl!(__parse_type__ AsSocketDescriptor $generics ($($where)*) descriptor $($input)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derive_impl {
    // Parse enum types, identifying annotated fields. Next macro: __process_derive__
    ( __parse_type__ $generator:ident $generics:tt $where:tt $attr:ident
        $(#[$eattr:meta])* $vis:vis enum $name:ident {
            $( $(#[$iattr:meta])* $field:ident
                $( ( $($(#[$tuple_attr:meta])* $tuple_type:ty),* $(,)?) )?
                $( { $($(#[$struct_attr:meta])* $struct_name:ident : $struct_type:ty),* $(,)? } )?
            ),*
            $(,)?
        }
    ) => {
        $crate::__support::find_annotated_multi!(
            ($crate::__derive_impl)
            (__process_derive__ $generator $attr $generics $where enum $name)
            $attr
            {
                compile_error!(concat!("No #[", stringify!($attr), "] field found"));
            }
            $(
                (
                    (Self::$field)
                    $(
                        $(
                            (($([$tuple_attr])*)
                            (type $tuple_type))
                        )*
                    )?
                    $(
                        $(
                            (($([$struct_attr])*)
                            (type $struct_type:($struct_name)))
                        )*
                    )?
                )
            )*
        );
    };

    // Parse named structs, identifying annotated fields. Next macro: __process_derive__
    ( __parse_type__ $generator:ident $generics:tt $where:tt $attr:ident
        $(#[$sattr:meta])* $vis:vis struct $name:ident { $( $(#[$fattr:meta])* $fvis:vis $fname:ident : $ftype:ty ),* $(,)? }
    ) => {
        $crate::__support::find_annotated_multi!(
            ($crate::__derive_impl)
            (__process_derive__ $generator $attr $generics $where struct $name)
            $attr
            {
                compile_error!(concat!("No #[", stringify!($attr), "] field found"));
            }
            ((Self)
                $( (($([$fattr])*) (type $ftype: ($fname))) )*
                (($([$sattr])*) (: (this)))
            )
        );
    };

    // Parse tuple structs, identifying annotated fields. Next macro: __process_derive__
    ( __parse_type__ $generator:ident $generics:tt $where:tt $attr:ident
        $(#[$sattr:meta])* $vis:vis struct $name:ident ( $( $(#[$fattr:meta])* $fvis:vis $ftype:ty ),* $(,)? );
    ) => {
        $crate::__support::find_annotated_multi!(
            ($crate::__derive_impl)
            (__process_derive__ $generator $attr $generics $where struct $name)
            $attr
            {
                compile_error!(concat!("No #[", stringify!($attr), "] field found"));
            }
            ((Self)
                $( (($([$fattr])*) (type $ftype)) )*
            )
        );
    };

    // Process the identified annotated fields. Next macro: __generate__ $generator
    // Note that the input here is:
    //   (case) index [attr] (type : name)
    ( (__process_derive__ $generator:ident $attr:ident $generics:tt $where:tt $type:ident $name:ident) (
        $( ( ($case:path) $index:literal $fattr:tt ( $( type $ftype:ty )? $( : ($fname:ident) )? ) ) )*
    )) => {
        const _: &str = stringify!( $type $name {
            $(
                # $fattr ($case) => $crate::__derive_impl!(__expand__ $attr ($case) $index $($ftype)? $(: $fname)?)
            )*
        });

        $crate::__derive_impl!(__generate__ $generator $attr $generics $where ($($($ftype)?)*)
            $type $name {
                $(
                    # $fattr ($case) => $crate::__derive_impl!(__expand__ $attr ($case) $index $($ftype)? $(: $fname)?)
                )*
            }
        );
    };

    // Generate the impl block for Read. Next macro: __impl__
    ( __generate__ Read $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        $crate::__derive_impl!(__impl__ ::std::io::Read : $name $generics $where $ftypes #[read] {
            fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::Read read($this, buf)) $struct)
            }
        });
    };

    // Generate the impl block for BufRead. Next macro: __impl__
    ( __generate__ BufRead $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        $crate::__derive_impl!(__impl__ ::std::io::BufRead : $name $generics $where $ftypes #[read] {
            fn fill_buf(&mut self) -> ::std::io::Result<&[u8]> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead fill_buf($this)) $struct)
            }

            fn consume(&mut self, amt: usize) {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead consume($this, amt)) $struct)
            }

            // Not yet stable!
            // fn has_data_left(&mut self) -> ::std::io::Result<bool> {
            //     let $this = self;
            //     $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead has_data_left($this)) $struct)
            // }

            fn read_until(&mut self, byte: u8, buf: &mut Vec<u8>) -> ::std::io::Result<usize> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead read_until($this, byte, buf)) $struct)
            }

            fn skip_until(&mut self, byte: u8) -> ::std::io::Result<usize> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead skip_until($this, byte)) $struct)
            }

            fn read_line(&mut self, buf: &mut String) -> ::std::io::Result<usize> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::BufRead read_line($this, buf)) $struct)
            }

            // Unimplemented because we cannot construct our own `Split`
            // fn split(self, byte: u8) -> ::std::io::Split<Self> {
            //     let $this = self;
            //     $crate::__derive_impl!(__foreach__ take $this (::std::io::BufRead split($this, byte)) $struct)
            // }

            // Unimplemented because we cannot construct our own `Lines`
            // fn lines(self) -> ::std::io::Lines<Self> {
            //     let $this = self;
            //     $crate::__derive_impl!(__foreach__ take $this (::std::io::BufRead lines($this)) $struct)
            // }
        });
    };

    // Generate the impl block for Write. Next macro: __impl__
    ( __generate__ Write $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        $crate::__derive_impl!(__impl__ ::std::io::Write : $name $generics $where $ftypes #[write] {
            fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::Write write($this, buf)) $struct)
            }
            fn flush(&mut self) -> ::std::io::Result<()> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ mut $this (::std::io::Write flush($this)) $struct)
            }
        });
    };

    // Generate the impl block for AsyncRead. Next macro: __impl__
    ( __generate__ AsyncRead $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        $crate::__derive_impl!(__impl__ ::tokio::io::AsyncRead : $name $generics $where $ftypes #[read] {
            #[inline]
            fn poll_read(
                mut self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &mut ::tokio::io::ReadBuf<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let $this = self;
                $crate::__derive_impl!(__foreach_pin__ mut $this (::tokio::io::AsyncRead poll_read($this, cx, buf)) $struct)
            }
        });
    };

    // Generate the impl block for AsyncWrite. Next macro: __impl__
    ( __generate__ AsyncWrite $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        $crate::__derive_impl!(__impl__ ::tokio::io::AsyncWrite : $name $generics $where $ftypes #[write] {
            #[inline]
            fn poll_write(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                buf: &[u8],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let $this = self;
                $crate::__derive_impl!(__foreach_pin__ mut $this (::tokio::io::AsyncWrite poll_write($this, cx, buf)) $struct)
            }

            #[inline]
            fn poll_flush(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let $this = self;
                $crate::__derive_impl!(__foreach_pin__ mut$this (::tokio::io::AsyncWrite poll_flush($this, cx)) $struct)
            }

            #[inline]
            fn poll_shutdown(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
            ) -> ::std::task::Poll<::std::io::Result<()>> {
                let $this = self;
                $crate::__derive_impl!(__foreach_pin__ mut $this (::tokio::io::AsyncWrite poll_shutdown($this, cx)) $struct)
            }

            #[inline]
            fn is_write_vectored(&self) -> bool {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::tokio::io::AsyncWrite is_write_vectored($this)) $struct)
            }

            #[inline]
            fn poll_write_vectored(
                self: ::std::pin::Pin<&mut Self>,
                cx: &mut ::std::task::Context<'_>,
                bufs: &[::std::io::IoSlice<'_>],
            ) -> ::std::task::Poll<::std::io::Result<usize>> {
                let $this = self;
                $crate::__derive_impl!(__foreach_pin__ mut $this (::tokio::io::AsyncWrite poll_write_vectored($this, cx, bufs)) $struct)
            }
        });
    };

    // std::os::fd::{AsFd, AsRawFd}, std::os::windows::io::{AsHandle, AsRawHandle}
    ( __generate__ AsFileDescriptor $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        #[cfg(unix)]
        $crate::__derive_impl!(__impl__ ::std::os::fd::AsFd : $name $generics $where $ftypes #[read] {
            fn as_fd(&self) -> ::std::os::fd::BorrowedFd<'_> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::fd::AsFd as_fd($this)) $struct)
            }
        });
        #[cfg(unix)]
        $crate::__derive_impl!(__impl__ ::std::os::fd::AsRawFd : $name $generics $where $ftypes #[read] {
            fn as_raw_fd(&self) -> ::std::os::fd::RawFd {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::fd::AsRawFd as_raw_fd($this)) $struct)
            }
        });
        #[cfg(windows)]
        $crate::__derive_impl!(__impl__ ::std::os::windows::io::AsRawHandle : $name $generics $where $ftypes #[read] {
            fn as_raw_handle(&self) -> ::std::os::windows::io::RawHandle {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::windows::io::AsRawHandle as_raw_handle($this)) $struct)
            }
        });
        #[cfg(windows)]
        $crate::__derive_impl!(__impl__ ::std::os::windows::io::AsHandle : $name $generics $where $ftypes #[read] {
            fn as_handle(&self) -> ::std::os::windows::io::BorrowedHandle<'_> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::windows::io::AsHandle as_handle($this)) $struct)
            }
        });
    };

    // std::os::fd::{AsFd, AsRawFd}, std::os::windows::io::{AsSocket, AsRawSocket}
    ( __generate__ AsSocketDescriptor $this:ident $generics:tt $where:tt $ftypes:tt $type:ident $name:ident $struct:tt) => {
        #[cfg(unix)]
        $crate::__derive_impl!(__impl__ ::std::os::fd::AsFd : $name $generics $where $ftypes #[read] {
            fn as_fd(&self) -> ::std::os::fd::BorrowedFd<'_> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::fd::AsFd as_fd($this)) $struct)
            }
        });
        #[cfg(unix)]
        $crate::__derive_impl!(__impl__ ::std::os::fd::AsRawFd : $name $generics $where $ftypes #[read] {
            fn as_raw_fd(&self) -> ::std::os::fd::RawFd {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::fd::AsRawFd as_raw_fd($this)) $struct)
            }
        });
        #[cfg(windows)]
        $crate::__derive_impl!(__impl__ ::std::os::windows::io::AsSocket : $name $generics $where $ftypes #[read] {
            fn as_socket(&self) -> ::std::os::windows::io::BorrowedSocket<'_> {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::windows::io::AsSocket as_socket($this)) $struct)
            }
        });
        #[cfg(windows)]
        $crate::__derive_impl!(__impl__ ::std::os::windows::io::AsRawSocket : $name $generics $where $ftypes #[read] {
            fn as_raw_socket(&self) -> ::std::os::windows::io::RawSocket {
                let $this = self;
                $crate::__derive_impl!(__foreach__ ref $this (::std::os::windows::io::AsRawSocket as_raw_socket($this)) $struct)
            }
        });
    };

    // Duplicate the $generics block. Next macro: __impl_2__
    ( __impl__ $trait:path : $name:ident $generics:tt ($($where:tt)*) ($($ftype:path)*) #[$attr:ident] $block:tt) => {
        $crate::__derive_impl!(__impl_2__ $trait : $name $generics $generics ($($where)*) ($($ftype)*) #[$attr] $block);
    };

    // Final macro. Generate the impl block.
    ( __impl_2__ $trait:path : $name:ident $generics:tt ( $( ($($generic:tt)*) ),* ) ($($where:tt)*) ($($ftype:path)*) #[$attr:ident] $block:tt) => {
        impl <$($($generic)*),*> $trait for $name <$($($generic)*),*>
            where
                // Add a where clause for each stream type. If it contains a generic, constrain it otherwise
                // use a placeholder type that implements the trait for certain.
                $(
                    $crate::__support::type_has_generic!(
                        ($ftype)
                        $generics
                        ($ftype)
                        (<() as $crate::__support::IsSupported::<&'static dyn $trait>>::Type)
                    ) : $trait,
                )*
                $($where)*
        $block
    };

    // Expand a self access pattern.
    ( __expand__ $this:ident ($case:path) $index:literal : this) => {
        {
            $this
        }
     };

    // Expand a named field to an access pattern.
    ( __expand__ $this:ident ($case:path) $index:literal $ftype:ty : $fname:tt) => {
        {
            let $case { $fname, .. } = $this else {
                unreachable!()
            };
            $fname
        }
     };

    // Expand a tuple field to an access pattern.
    ( __expand__ $this:ident ($case:path) $index:literal $ftype:ty) => {
        {
            let $crate::__support::repeat_in_parenthesis!(($case) $index (_,) ($this, .. )) = $this else {
                unreachable!()
            };
            $this
        }
    };

    ( __foreach__ $refmut:tt $this:ident $fn:tt {$(
        # $attr:tt ($case:path) => $access:expr
    )*}) =>{
        {
            match $this {
                $( $case {..} => {
                    $crate::__derive_impl!(__validate_macro__ # $attr);
                    $crate::__support::if_meta!(
                        as_ref
                        $attr
                        ({
                            let $this = $access.as_ref();
                            $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                        })
                        ($crate::__support::if_meta!(
                            deref
                            $attr
                            ({
                                let $this = ::std::ops::Deref::deref($access);
                                $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                            })
                            ($crate::__support::if_meta!(
                                duck
                                $attr
                                ({
                                    let $this = $access;
                                    $crate::__derive_impl!(__foreach_inner_duck__ unpin $refmut # $attr $fn)
                                })
                                ({
                                    let $this = $access;
                                    $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                                })
                            ))
                        ))
                    )
                } )*
            }
        }
    };

    ( __foreach_pin__ $refmut:tt $this:ident $fn:tt {$(
        # $attr:tt ($case:path) => $access:expr
    )*}) =>{
        {
            match &*$this {
                $( $case {..} => {
                    $crate::__derive_impl!(__validate_macro__ # $attr);
                    $crate::__support::if_meta!(
                        as_ref
                        $attr
                        ({
                            // NOTE: as_ref requires Unpin for safety
                            let mut $this = $this.get_mut();
                            let mut $this = ::std::pin::Pin::new($access);
                            let $this = ::std::pin::Pin::new($this.get_mut().as_mut());
                            $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                        })
                        ($crate::__support::if_meta!(
                            deref
                            $attr
                            ({
                                // NOTE: as_mut requires Unpin for safety
                                let mut $this = $this.get_mut();
                                let mut $this = ::std::ops::DerefMut::deref_mut($access);
                                let $this = ::std::pin::Pin::new($this);
                                $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                            })
                            ($crate::__support::if_meta!(
                                duck
                                $attr
                                ({
                                    // NOTE: duck typing requires Unpin for safety
                                    let mut $this = $this.get_mut();
                                    let mut $this = ::std::pin::Pin::new($access);
                                    $crate::__derive_impl!(__foreach_inner_duck__ pin $refmut # $attr $fn)
                                })
                                ({
                                    // SAFETY: we are mapping this pin to a
                                    // nested field which must be `Unpin`.
                                    // Because we are delegating to methods that
                                    // never uses mutable references that aren't
                                    // pinned, this is safe.
                                    let mut $this = (unsafe { $this.get_unchecked_mut() });
                                    let mut $this = (unsafe { ::std::pin::Pin::new_unchecked($access) });
                                    $crate::__derive_impl!(__foreach_inner__ $refmut # $attr $fn)
                                })
                            ))
                        ))
                    )
                } )*
            }
        }
    };

    ( __foreach_inner__ $refmut:tt # $attr:tt ( $( :: $fn_part:ident )+ $fn_final:ident ( $($arg:expr),* ) ) ) => {
        // needle, haystack, default
        {
            $crate::__support::extract_meta!(
                $fn_final
                $attr
                ($(::$fn_part)+ :: $fn_final )
            ) ($($arg),*)
        }
    };

    ( __foreach_inner_duck__ unpin ref # $attr:tt ( $( :: $fn_part:ident )+ $fn_final:ident ( $arg0:expr $(, $arg:expr)* ) ) ) => {
        // needle, haystack, default
        {
            $crate::__support::extract_meta!(
                $fn_final
                $attr
                ( Self :: $fn_final( ($arg0) $(, $arg)*) )
            )
        }
    };

    ( __foreach_inner_duck__ unpin mut # $attr:tt ( $( :: $fn_part:ident )+ $fn_final:ident ( $arg0:expr $(, $arg:expr)* ) ) ) => {
        // needle, haystack, default
        {
            $crate::__support::extract_meta!(
                $fn_final
                $attr
                ( Self :: $fn_final( ($arg0) $(, $arg)*) )
            )
        }
    };

    ( __foreach_inner_duck__ $pin:tt ref # $attr:tt ( $( :: $fn_part:ident )+ $fn_final:ident ( $arg0:expr $(, $arg:expr)* ) ) ) => {
        {
            // I don't think anyone uses these...
            unimplemented!("pin ref duck type is not yet supported");
        }
    };

    ( __foreach_inner_duck__ $pin:tt mut # $attr:tt ( $( :: $fn_part:ident )+ $fn_final:ident ( $arg0:expr $(, $arg:expr)* ) ) ) => {
        {
            // Choose the correct pointer for the receiver via trait.
            #[allow(non_camel_case_types)]
            trait __Derive_Io_Coerce<T, U> {
                fn __derive_io_coerce(this: T) -> U;
            }
            impl<'a, T> __Derive_Io_Coerce<::std::pin::Pin<&'a mut T>, &'a mut T> for ::std::pin::Pin<&'a mut T> where T: Unpin {
                fn __derive_io_coerce(this: ::std::pin::Pin<&'a mut T>) -> &'a mut T {
                    this.get_mut()
                }
            }
            impl<'a, T> __Derive_Io_Coerce<::std::pin::Pin<&'a mut T>, ::std::pin::Pin<&'a mut T>> for ::std::pin::Pin<&'a mut T> {
                fn __derive_io_coerce(this: ::std::pin::Pin<&'a mut T>) -> ::std::pin::Pin<&'a mut T> {
                    this
                }
            }

            let self_type = $arg0;
            let self_type = <::std::pin::Pin<&mut Self> as __Derive_Io_Coerce<_, _>>::__derive_io_coerce(self_type);

            // needle, haystack, default
            let callable = $crate::__support::extract_meta!(
                $fn_final
                $attr
                (Self :: $fn_final)
            );

            callable(self_type $(, $arg)*)
        }
    };

    ( __validate_macro__ #[read]) => {
    };

    ( __validate_macro__ #[read($( as_ref )? $(,)? $( deref )? $(,)? $( duck )? $(,)? $( poll_read=$poll_read:ident )? )]) => {
    };

    ( __validate_macro__ #[write]) => {
    };

    ( __validate_macro__ #[write($($key:ident $(=$value:ident)?),* $(,)?)]) => {
        $crate::__derive_impl!(__validate_macro_deep__ #[write($($key $(=$value)?),*)]);
    };

    ( __validate_macro_deep__ #[write(
        $( as_ref )? $(,)?
        $( deref )? $(,)?
        $( duck )? $(,)?
        $( poll_write=$poll_write:ident )? $(,)?
        $( poll_flush=$poll_flush:ident )? $(,)?
        $( poll_shutdown=$poll_shutdown:ident )? $(,)?
        $( is_write_vectored=$is_write_vectored:ident )? $(,)?
        $( poll_write_vectored=$poll_write_vectored:ident )?
    )]) => {
    };

    ( __validate_macro_deep__ # $($rest:tt)*) => {
        compile_error!(concat!("Invalid #", stringify!($($rest)*), " attribute"));
    };

    ( __validate_macro__ #[descriptor]) => {
    };

    ( __validate_macro__ #[descriptor(as_ref)]) => {
    };

    ( __validate_macro__ #[descriptor(deref)]) => {
    };

    ( __validate_macro__ # $attr:tt) => {
        compile_error!(concat!("Invalid #", stringify!($attr), " attribute"));
    };
}
