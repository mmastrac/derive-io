//! Support macros for `derive-io`. This is not intended to be used directly and
//! has no stable API.

use std::collections::HashSet;

use proc_macro::*;

/// `#[derive(Read)]`
///
/// Derives `std::io::Read` for the given struct.
///
/// Supported attributes:
///
/// - `#[read]`: Marks the field as a read stream.
/// - `#[read(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[read(deref)]`: Delegates the field to the inner type using `Deref`/`DerefMut`.
/// - `#[read(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(Read, attributes(read, write, descriptor))]
pub fn derive_io_read(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_read", input)
}

/// `#[derive(BufRead)]`
///
/// Derives `std::io::BufRead` for the given struct. `std::io::Read` must also
/// be implemented.
///
/// Unsupported methods:
///
/// - `split` (std-internal implementation)
/// - `lines` (std-internal implementation)
/// - `has_data_left` (unstable feature)
///
/// Supported attributes:
///
/// - `#[read]`: Marks the field as a read stream.
/// - `#[read(as_ref)]`: Delegates the field to the inner type using
///   `AsRef`/`AsMut`.
/// - `#[read(deref)]`: Delegates the field to the inner type using
///   `Deref`/`DerefMut`.
/// - `#[read(<function>=<override>)]`: Overrides the default `<function>`
///   method with the given override function.
#[proc_macro_derive(BufRead, attributes(read, write, descriptor))]
pub fn derive_io_bufread(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_bufread", input)
}

/// `#[derive(Write)]`
///
/// Derives `std::io::Write` for the given struct.
///
/// Supported attributes:
///
/// - `#[write]`: Marks the field as a write stream.
/// - `#[write(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[write(deref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[write(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(Write, attributes(read, write, descriptor))]
pub fn derive_io_write(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_write", input)
}

/// `#[derive(AsyncRead)]`:
///
/// Derives `tokio::io::AsyncRead` for the given struct.
///
/// Supported attributes:
///
/// - `#[read]`: Marks the field as a read stream.
/// - `#[read(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[read(deref)]`: Delegates the field to the inner type using `Deref`/`DerefMut`.
/// - `#[read(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(AsyncRead, attributes(read, write, descriptor))]
pub fn derive_io_async_read(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_async_read", input)
}

/// `#[derive(AsyncWrite)]`:
///
/// Derives `tokio::io::AsyncWrite` for the given struct.
///
/// Supported attributes:
///
/// - `#[write]`: Marks the field as a write stream.
/// - `#[write(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[write(deref)]`: Delegates the field to the inner type using `Deref`/`DerefMut`.
/// - `#[write(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(AsyncWrite, attributes(read, write, descriptor))]
pub fn derive_io_async_write(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_async_write", input)
}

/// `#[derive(AsFileDescriptor)]`
///
/// Derives `std::os::fd::{AsFd, AsRawFd}` and `std::os::windows::io::{AsHandle, AsRawHandle}` for the given struct.
///
/// Supported attributes:
///
/// - `#[descriptor]`: Marks the field as a file descriptor.
/// - `#[descriptor(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[descriptor(deref)]`: Delegates the field to the inner type using `Deref`/`DerefMut`.
/// - `#[descriptor(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(AsFileDescriptor, attributes(read, write, descriptor))]
pub fn derive_io_as_file_descriptor(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_as_file_descriptor", input)
}

/// `#[derive(AsSocketDescriptor)]`
///
/// Derives `std::os::fd::{AsFd, AsRawFd}` and `std::os::windows::io::{AsSocket, AsRawSocket}` for the given struct.
///
/// Supported attributes:
///
/// - `#[descriptor]`: Marks the field as a socket descriptor.
/// - `#[descriptor(as_ref)]`: Delegates the field to the inner type using `AsRef`/`AsMut`.
/// - `#[descriptor(deref)]`: Delegates the field to the inner type using `Deref`/`DerefMut`.
/// - `#[descriptor(<function>=<override>)]`: Overrides the default `<function>` method with the given override function.
#[proc_macro_derive(AsSocketDescriptor, attributes(read, write, descriptor))]
pub fn derive_io_as_socket_descriptor(input: TokenStream) -> TokenStream {
    generate("derive_io", "derive_io_as_socket_descriptor", input)
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
    //  - If a generic has no bounds, we don't add it to the where clause
    let mut in_generics = false;
    let mut generics_ident = false;
    let mut generics_accum = TokenStream::new();
    let mut in_where_clause = false;
    let mut in_generic_default = false;
    let mut in_generic_const = false;
    for token in iterator.by_ref() {
        match token {
            TokenTree::Punct(p) if !in_where_clause && p.as_char() == '<' => {
                in_generics = true;
                generics_ident = true;
            }
            TokenTree::Punct(ref p) if !in_where_clause && p.as_char() == '>' => {
                if in_generics {
                    in_generics = false;
                    if generics_ident {
                        generics.extend([TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            std::mem::take(&mut generics_accum),
                        ))]);
                    }
                    if !generics_accum.is_empty() {
                        panic!();
                    }
                }
            }
            TokenTree::Punct(ref p) if p.as_char() == ':' => {
                if in_generics {
                    if in_generic_const {
                        generics_accum.extend([token]);
                    } else {
                        if generics_ident {
                            generics.extend([TokenTree::Group(Group::new(
                                Delimiter::Parenthesis,
                                generics_accum.clone(),
                            ))]);
                        }
                        generics_ident = false;
                        where_clause.extend(std::mem::take(&mut generics_accum));
                        where_clause.extend([token]);
                    }
                } else if in_where_clause {
                    where_clause.extend([token]);
                } else {
                    new_item.extend([token]);
                }
            }
            TokenTree::Punct(ref p) if p.as_char() == ',' => {
                if in_generics {
                    if generics_ident {
                        generics.extend([TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            std::mem::take(&mut generics_accum),
                        ))]);
                    } else if !in_generic_const {
                        where_clause.extend([token.clone()]);
                    }
                    generics.extend([token]);
                    generics_ident = true;
                    in_generic_default = false;
                    in_generic_const = false;
                } else if in_where_clause {
                    where_clause.extend([token]);
                } else {
                    new_item.extend([token]);
                }
            }
            TokenTree::Punct(ref p) if p.as_char() == '=' => {
                if in_generics {
                    generics_ident = false;
                    in_generic_default = true;
                    if in_generic_const {
                        generics.extend([TokenTree::Group(Group::new(
                            Delimiter::Parenthesis,
                            std::mem::take(&mut generics_accum),
                        ))]);
                    }
                }
            }
            TokenTree::Ident(ref l) if l.to_string() == "const" => {
                panic!("const not yet supported");
                // if in_generics {
                //     generics_ident = true;
                //     in_generic_const = true;
                //     generics_accum.extend([token.clone()]);
                // }
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
                    } else if !in_generic_default {
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
            &format!("{macro_type}_parse"),
            Span::call_site(),
        )),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, inner)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);

    invoke
}

fn expect_any(named: &str, iterator: &mut impl Iterator<Item = TokenTree>) -> TokenTree {
    let next = iterator.next();
    let Some(token) = next else {
        panic!("Expected {named} token, got end of stream");
    };
    token
}

fn expect_group(named: &str, iterator: &mut impl Iterator<Item = TokenTree>) -> Group {
    let next = iterator.next();
    let Some(TokenTree::Group(group)) = next else {
        panic!("Expected {named} group, got {next:?}");
    };
    group
}

fn expect_ident(named: &str, iterator: &mut impl Iterator<Item = TokenTree>) -> Ident {
    let next = iterator.next();
    let Some(TokenTree::Ident(ident)) = next else {
        panic!("Expected {named} ident, got {next:?}");
    };
    ident
}

fn expect_literal(named: &str, iterator: &mut impl Iterator<Item = TokenTree>) -> Literal {
    let next = iterator.next();
    if let Some(TokenTree::Group(ref group)) = next {
        if group.delimiter() == Delimiter::None {
            let mut iter = group.stream().into_iter();
            return expect_literal(named, &mut iter);
        }
    }
    let Some(TokenTree::Literal(literal)) = next else {
        panic!("Expected {named} literal, got {next:?}");
    };
    literal
}

/// Unwrap a grouped meta element to its final group.
fn expect_is_meta(named: &str, mut attr: TokenTree) -> Group {
    let outer = attr.clone();
    while let TokenTree::Group(group) = attr {
        let mut iter = group.clone().stream().into_iter();
        let first = iter
            .next()
            .expect("Expected attr group to have one element");
        if let TokenTree::Ident(_) = first {
            return Group::new(Delimiter::Bracket, group.stream());
        }
        attr = first;
    }
    panic!("Expected meta group {named}, got {outer}");
}

/// [
///   (__next__) (args) expected_attr {on_error}
///   ((attr attr) (item)) ((attr attr) (item))
/// ] -> __next__!((args) (item))
#[proc_macro]
pub fn find_annotated(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();

    let next_macro = expect_group("__next__ macro", &mut iterator);
    let args = expect_group("__next__ arguments", &mut iterator);
    let expected_attr = expect_ident("expected_attr", &mut iterator);
    let on_error = expect_group("on_error", &mut iterator);

    for token in iterator {
        let TokenTree::Group(check) = token else {
            panic!("Expected check group");
        };
        let mut iter = check.stream().into_iter();
        let attrs = expect_group("attrs", &mut iter);
        let item = expect_any("item", &mut iter);
        for (index, attr) in attrs.stream().into_iter().enumerate() {
            let attr = expect_is_meta("attr", attr);
            let first = expect_ident("first attr", &mut attr.clone().stream().into_iter());
            if first.to_string() == expected_attr.to_string() {
                let mut next = next_macro.stream();
                next.extend([
                    TokenTree::Punct(Punct::new('!', Spacing::Alone)),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        TokenStream::from_iter([
                            TokenTree::Group(args),
                            TokenTree::Literal(Literal::usize_unsuffixed(index)),
                            TokenTree::Group(attr),
                            item,
                        ]),
                    )),
                    TokenTree::Punct(Punct::new(';', Spacing::Alone)),
                ]);
                return next;
            }
        }
    }

    on_error.stream()
}

/// [(__next__) (args) expected_attr {on_error}
///  ( (id) (([attr] [attr]) (item)) (([attr] [attr]) (item)) )
///  ( (id) (([attr] [attr]) (item)) (([attr] [attr]) (item)) )
/// ] -> __next__!((args) ((id) (item)))
#[proc_macro]
pub fn find_annotated_multi(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();

    let next_macro = expect_group("__next__ macro", &mut iterator);
    let args = expect_group("__next__ arguments", &mut iterator);
    let expected_attr = expect_ident("expected_attr", &mut iterator);
    let on_error = expect_group("on_error", &mut iterator);
    let mut output = TokenStream::new();

    'outer: for token in iterator {
        let TokenTree::Group(id) = token else {
            panic!("Expected id group");
        };
        let mut iter = id.stream().into_iter();
        let id = expect_group("id", &mut iter);
        let mut index = 0;
        for token in iter {
            let TokenTree::Group(check) = token else {
                panic!("Expected check group");
            };
            let mut iter = check.stream().into_iter();
            let attrs = expect_group("attrs", &mut iter);
            let item = expect_any("item", &mut iter);
            for attr in attrs.stream().into_iter() {
                let attr = expect_is_meta("attr", attr);
                let first = expect_ident("first attr", &mut attr.clone().stream().into_iter());
                if first.to_string() == expected_attr.to_string() {
                    output.extend([TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        TokenStream::from_iter([
                            TokenTree::Group(id.clone()),
                            TokenTree::Literal(Literal::usize_unsuffixed(index)),
                            TokenTree::Group(attr),
                            item.clone(),
                        ]),
                    ))]);
                    continue 'outer;
                }
            }
            index += 1;
        }
        return on_error.stream();
    }

    let mut next = next_macro.stream();
    next.extend([
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter([
                TokenTree::Group(args),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, output)),
            ]),
        )),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ]);
    next
}

/// [prefix count repeated suffix] -> prefix (repeated*count suffix)
#[proc_macro]
pub fn repeat_in_parenthesis(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();
    let prefix = expect_group("prefix", &mut iterator);
    let count = expect_literal("count", &mut iterator);
    let count: usize = str::parse(&count.to_string()).expect("Expected count to be a number");
    let repeated = expect_group("repeated", &mut iterator);
    let suffix = expect_group("suffix", &mut iterator);
    let mut repeat = TokenStream::new();
    for _ in 0..count {
        repeat.extend(repeated.clone().stream());
    }
    repeat.extend(suffix.stream());
    let mut output = TokenStream::new();
    output.extend(prefix.stream());
    output.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, repeat))]);
    output
}

// needle haystack(key=value,key=value) default -> extracted OR default
#[proc_macro]
pub fn extract_meta(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();
    let needle = expect_ident("needle", &mut iterator);
    let haystack = expect_group("haystack", &mut iterator);
    let default = expect_group("default", &mut iterator);

    let mut haystack = haystack.stream().into_iter();

    loop {
        let attr = haystack.next();
        if let Some(TokenTree::Group(ref group)) = attr {
            haystack = group.stream().into_iter();
            continue;
        }
        break;
    }

    loop {
        let key = haystack.next();
        if let Some(TokenTree::Group(ref group)) = key {
            haystack = group.stream().into_iter();
            continue;
        }
        let Some(key) = key else {
            break;
        };
        let Some(next) = haystack.next() else {
            break;
        };
        // Ignore simple attributes
        let TokenTree::Punct(punct) = next else {
            panic!("Expected = after key, got {next:?}");
        };

        if punct.as_char() == ',' {
            continue;
        }

        let value = expect_ident("value", &mut haystack);

        if key.to_string() == needle.to_string() {
            return TokenStream::from_iter([TokenTree::Ident(value)]);
        }

        if haystack.next().is_none() {
            break;
        }
    }

    default.stream()
}

#[proc_macro]
pub fn if_meta(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();
    let needle = expect_ident("needle", &mut iterator);
    let haystack = expect_group("haystack", &mut iterator);
    let if_true = expect_group("if_true", &mut iterator);
    let if_false = expect_group("if_false", &mut iterator);

    let mut haystack = haystack.stream().into_iter();

    loop {
        let attr = haystack.next();
        if let Some(TokenTree::Group(ref group)) = attr {
            haystack = group.stream().into_iter();
            continue;
        }
        break;
    }

    loop {
        let key = haystack.next();
        if let Some(TokenTree::Group(ref group)) = key {
            haystack = group.stream().into_iter();
            continue;
        }
        let Some(key) = key else {
            break;
        };
        if key.to_string() == needle.to_string() {
            return if_true.stream();
        }
    }

    if_false.stream()
}

#[proc_macro]
pub fn type_has_generic(input: TokenStream) -> TokenStream {
    let mut iterator = input.into_iter();

    let type_ = expect_group("type", &mut iterator);
    let generic = expect_group("generics", &mut iterator);
    let if_true = expect_group("if_true", &mut iterator);
    let if_false = expect_group("if_false", &mut iterator);

    fn recursive_collect_generics(generics: &mut HashSet<String>, type_tokens: TokenStream) {
        let iterator = type_tokens.into_iter();
        for token in iterator {
            if let TokenTree::Ident(ident) = &token {
                generics.insert(ident.to_string());
            } else if let TokenTree::Group(group) = token {
                recursive_collect_generics(generics, group.stream());
            }
        }
    }

    let mut generics = HashSet::new();
    recursive_collect_generics(&mut generics, generic.stream());

    fn recursive_check_generics(generics: &HashSet<String>, type_tokens: TokenStream) -> bool {
        let iterator = type_tokens.into_iter();
        for token in iterator {
            if let TokenTree::Ident(ident) = &token {
                if generics.contains(&ident.to_string()) {
                    return true;
                }
            } else if let TokenTree::Group(group) = token {
                if recursive_check_generics(generics, group.stream()) {
                    return true;
                }
            }
        }
        false
    }

    if recursive_check_generics(&generics, type_.stream()) {
        if_true.stream()
    } else {
        if_false.stream()
    }
}
