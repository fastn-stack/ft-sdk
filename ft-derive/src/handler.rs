pub fn handle(item: proc_macro::TokenStream, kind: &str) -> proc_macro::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = &sig.ident;
    let fn_name_endpoint =
        syn::Ident::new(format!("{}__endpoint", fn_name).as_str(), fn_name.span());
    let return_type: syn::Type =
        syn::parse_str(format!("ft_sdk::{kind}::Result").as_str()).unwrap();

    // ensure sig.output is same as return_type

    match sig.output {
        syn::ReturnType::Default => {
            return w2(format!("The return type must be ft_sdk::{kind}::Result").as_str())
        }
        _ => todo!(),
    };

    // if let sig.output {
    //     return [
    //         proc_macro::TokenTree::Ident(proc_macro::Ident::new(
    //             "compile_error",
    //             proc_macro::Span::mixed_site(),
    //         )),
    //         proc_macro::TokenTree::Punct(proc_macro::Punct::new('!', proc_macro::Spacing::Alone)),
    //         proc_macro::TokenTree::Group(proc_macro::Group::new(
    //             proc_macro::Delimiter::Parenthesis,
    //             [proc_macro::TokenTree::Literal(proc_macro::Literal::string(
    //                 "Some error message here!",
    //             ))]
    //             .into_iter()
    //             .collect(),
    //         )),
    //     ]
    //     .into_iter()
    //     .collect();
    // }

    let expanded = quote::quote! {
        #[no_mangle]
        pub extern "C" fn #fn_name_endpoint() {
            let resp = match #fn_name(in_, conn) {
                Ok(resp) => resp.into(),
                Err(e) => {
                    ft_sdk::println!("Error: {:?}", e);
                    e.into()
                }
            };
            // resp.append_cookies(ctx);
            ft_sdk::http::send_response(resp);
        }

        #(#attrs)*
        #vis #sig {
            #block
        }
    };

    // println!("{expanded}");
    proc_macro::TokenStream::from(expanded)
}

fn w2(msg: &str) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote::quote! {
        compile_error!(#msg);
    })
}

fn warning(msg: &str) -> proc_macro::TokenStream {
    [
        proc_macro::TokenTree::Ident(proc_macro::Ident::new(
            "compile_error",
            proc_macro::Span::mixed_site(),
        )),
        proc_macro::TokenTree::Punct(proc_macro::Punct::new('!', proc_macro::Spacing::Alone)),
        proc_macro::TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            [proc_macro::TokenTree::Literal(proc_macro::Literal::string(
                msg,
            ))]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
