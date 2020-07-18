use proc_macro::TokenStream;
use syn;
use quote;


#[proc_macro]
pub fn create_build_array(input: TokenStream) -> TokenStream {
    let arr_type = syn::parse_macro_input!(input as syn::TypeArray);
    let elem_type = &arr_type.elem;
    let elem_count = &arr_type.len;
    TokenStream::from(quote::quote!{
        fn build_array() -> #arr_type {
            let mut i = 0;
            arr_macro::arr![<#elem_type>::new({i += 1; i}); #elem_count]
        }
    })
}
