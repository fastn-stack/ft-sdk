extern crate self as ft_derive;

mod handler;

#[proc_macro_attribute]
pub fn processor(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ft_derive::handler::handle(item, "processor")
}

#[proc_macro_attribute]
pub fn data(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ft_derive::handler::handle(item, "data")
}

#[proc_macro_attribute]
pub fn form(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    ft_derive::handler::handle(item, "form")
}
