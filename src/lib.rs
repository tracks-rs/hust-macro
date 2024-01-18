extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr, Expr};
use tracks_hust::preprocess_and_generate_rust_code;

#[proc_macro]
pub fn include_hust(input: TokenStream) -> TokenStream {
    let file_name = parse_macro_input!(input as LitStr);
    let file_contents = std::fs::read_to_string(file_name.value()).unwrap();

    let processed_code = preprocess_and_generate_rust_code(&file_contents);

    let expanded = quote! {
        {
            #processed_code
        }
    };

    TokenStream::from(expanded)
}
