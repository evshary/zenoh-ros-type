#[proc_macro_derive(ZBytesCdr)]
pub fn zbytes_cdr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let expanded = quote::quote! {
        #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
        struct #name;
    };

    proc_macro::TokenStream::from(expanded)
}
