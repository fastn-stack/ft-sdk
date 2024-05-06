#[proc_macro_attribute]
pub fn handle_http(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = &sig.ident;
    let expanded = quote::quote! {
        #[no_mangle]
        pub extern "C" fn main_ft() {
            let req = ft_sdk::http::current_request();
            let resp = match #fn_name(req) {
                Ok(resp) => resp,
                Err(e) => {
                    ft_sdk::println!("Error: {:?}", e);
                    e
                }
            };
            ft_sdk::http::send_response(resp);
        }

        #(#attrs)*
        #vis #sig {
            #block
        }
    };

    // println!("{}", expanded.to_string());
    proc_macro::TokenStream::from(expanded)
}
