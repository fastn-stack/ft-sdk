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
            let mut conn = match ft_sdk::default_connection() {
                Ok(conn) => conn,
                Err(e) => {
                    ft_sdk::println!("cant get default connection: {:?}", e);
                    return ft_sdk::http::send_response(::ft_sdk::server_error!("cant get default connection: {e:?}"));
                }
            };
            let req = ft_sdk::http::current_request();
            let in_ = match ft_sdk::In::from_request(req, &mut conn) {
                Ok(in_) => in_,
                Err(e) => {
                    ft_sdk::println!("cant create In object: {:?}", e);
                    return ft_sdk::http::send_response(::ft_sdk::server_error!("cant create In object: {e:?}"));
                }
            };
            let resp = match #fn_name(in_, conn) {
                Ok(resp) => resp.into(),
                Err(e) => {
                    ft_sdk::println!("Error: {:?}", e);
                    e.into()
                }
            };
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
