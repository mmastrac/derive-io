use derive_io::{AsyncRead, AsyncWrite};

/// [`ComplexStream`] - Tests complex type structures with multiple generic parameters, lifetimes, and derive macros.
#[derive(AsyncRead, AsyncWrite, derive_more::Debug)]
#[allow(unused)]
pub enum ComplexStream<'a, S: std::fmt::Debug, D: std::any::Any = ()> {
    #[debug("A")]
    A(
        #[read]
        #[write]
        S,
        Option<&'a D>,
    ),
    #[debug("B")]
    B(
        #[read]
        #[write]
        crate::generic_structs::GenericUnrelated<D, S>,
        Option<&'a D>,
    ),
}
