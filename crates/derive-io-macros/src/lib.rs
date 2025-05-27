use proc_macro::*;

#[proc_macro_derive(Read, attributes(read))]
pub fn derive_io_read(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_read", input)
}

#[proc_macro_derive(Write, attributes(write))]
pub fn derive_io_write(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_write", input)
}

#[proc_macro_derive(AsyncRead, attributes(read))]
pub fn derive_io_async_read(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_async_read", input)
}

#[proc_macro_derive(AsyncWrite, attributes(write))]
pub fn derive_io_async_write(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_async_write", input)
}

/// Generates the equivalent of this Rust code as a TokenStream:
///
/// ```nocompile
/// ::ctor::__support::ctor_parse!(#[ctor] fn foo() { ... });
/// ::dtor::__support::dtor_parse!(#[dtor] fn foo() { ... });
/// ```
#[allow(unknown_lints, tail_expr_drop_order)]
fn generate(macro_crate: &str, macro_type: &str, item: TokenStream) -> TokenStream {
    let mut inner = TokenStream::new();

    inner.extend(item);

    let mut invoke = TokenStream::from_iter([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(macro_crate, Span::call_site())),
    ]);

    invoke.extend([
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("__support", Span::call_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new(
            &format!("{}_parse", macro_type),
            Span::call_site(),
        )),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    invoke
}
