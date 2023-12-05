use proc_macro::TokenStream;

#[proc_macro]
pub fn something_else(input: TokenStream) -> TokenStream {
    input
}