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
    let mut generics = TokenStream::new();
    let mut where_clause = TokenStream::new();
    let mut new_item = TokenStream::new();
    let mut iterator = item.into_iter();

    // Parse out generics and where clause into something easier for macros to digest:
    //  - Generic bounds are moved to where clause, leaving just types/lifetimes
    //  - All generics exist in where clause
    let mut in_generics = false;
    let mut generics_ident = false;
    let mut generics_accum = TokenStream::new();
    let mut in_where_clause = false;
    while let Some(token) = iterator.next() {
        match token {
            TokenTree::Punct(p) if p.as_char() == '<' => {
                in_generics = true;
                generics_ident = true;
            }
            TokenTree::Punct(ref p) if p.as_char() == '>' => {
                if in_generics {
                    in_generics = false;
                    if generics_ident {
                        generics.extend(std::mem::take(&mut generics_accum));
                    }
                }
            }
            TokenTree::Punct(ref p) if p.as_char() == ':' => {
                if in_generics {
                    generics_ident = false;
                    generics.extend(generics_accum.clone());
                    where_clause.extend(std::mem::take(&mut generics_accum));
                    where_clause.extend([token]);
                } else if in_where_clause {
                    where_clause.extend([token]);
                } else {
                    new_item.extend([token]);
                }
            }
            TokenTree::Punct(ref p) if p.as_char() == ',' => {
                if in_generics {
                    if generics_ident {
                        generics.extend(std::mem::take(&mut generics_accum));
                        generics.extend([token.clone()]);
                    } else {
                        where_clause.extend([token]);
                    }
                    generics_ident = true;
                } else if in_where_clause {
                    where_clause.extend([token]);
                } else {
                    new_item.extend([token]);
                }
            }
            TokenTree::Ident(l) if l.to_string() == "where" => {
                in_where_clause = true;
            }
            TokenTree::Group(ref p) if p.delimiter() == Delimiter::Brace => {
                new_item.extend([token]);
                break;
            }
            _ => {
                if in_generics {
                    if generics_ident {
                        generics_accum.extend([token]);
                    } else {
                        where_clause.extend([token]);
                    }
                } else if in_where_clause {
                    where_clause.extend([token]);
                } else {
                    new_item.extend([token]);
                }
            }
        }
    }
    new_item.extend(iterator);

    let mut inner = TokenStream::new();
    inner.extend([
        TokenTree::Group(Group::new(Delimiter::Parenthesis, new_item)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, generics)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, where_clause)),
    ]);

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
