extern crate self as ft_derive;

#[proc_macro_attribute]
pub fn processor(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle(item, "processor", "handler")
}

#[proc_macro_attribute]
pub fn raw_data(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle(item, "data", "raw_bytes_handler")
}

#[proc_macro_attribute]
pub fn wrapped_processor(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle(item, "processor", "wrapped_processor")
}

#[proc_macro_attribute]
pub fn data(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle(item, "data", "handler")
}

#[proc_macro_attribute]
pub fn form(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    handle(item, "form", "handler")
}

fn handle(item: proc_macro::TokenStream, kind: &str, handler: &str) -> proc_macro::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = &sig.ident;
    let fn_name_entrypoint =
        syn::Ident::new(format!("{}__entrypoint", fn_name).as_str(), fn_name.span());
    let return_type: syn::Type =
        syn::parse_str(format!("ft_sdk::{kind}::Result").as_str()).unwrap();
    let handler: syn::Path =
        syn::parse_str(format!("ft_sdk::from_request::{handler}::handle").as_str()).unwrap();

    match sig.output {
        syn::ReturnType::Default => {
            return compiler_error(
                format!("The return type must be ft_sdk::{kind}::Result").as_str(),
            );
        }
        syn::ReturnType::Type(_, ref ty) => {
            if ty.as_ref() != &return_type {
                return compiler_error(
                    format!(
                        "The return type must be ft_sdk::{kind}::Result, not {}.",
                        proc_macro::TokenStream::from(quote::quote! { #ty })
                    )
                    .as_str(),
                );
            }
        }
    };

    let expanded = quote::quote! {
        #[no_mangle]
        pub extern "C" fn #fn_name_entrypoint() {
            #handler(#fn_name)
        }

        #(#attrs)*
        #vis #sig {
            #block
        }
    };

    // println!("{expanded}");
    proc_macro::TokenStream::from(expanded)
}

fn compiler_error(msg: &str) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote::quote! {
        compile_error!(#msg);
    })
}
