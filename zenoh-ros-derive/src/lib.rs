#[proc_macro_derive(ZBytesCdr)]
pub fn zbytes_cdr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input token stream into a syntax tree
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // Extract the struct's name
    let name = input.ident;

    let expanded = quote::quote! {
        impl From<#name> for zenoh::bytes::ZBytes {
            fn from(msg: #name) -> Self {
                let payload = cdr::serialize::<_, _, cdr::CdrLe>(&msg, cdr::Infinite).unwrap();
                payload.into()
            }
        }
        impl From<&#name> for zenoh::bytes::ZBytes {
            fn from(msg: &#name) -> Self {
                let payload = cdr::serialize::<_, _, cdr::CdrLe>(msg, cdr::Infinite).unwrap();
                payload.into()
            }
        }

        impl From<zenoh::bytes::ZBytes> for #name {
            fn from(bytes: zenoh::bytes::ZBytes) -> Self {
                cdr::deserialize(&bytes.to_bytes()).unwrap()
            }
        }
        impl From<&zenoh::bytes::ZBytes> for #name {
            fn from(bytes: &zenoh::bytes::ZBytes) -> Self {
                cdr::deserialize(&(*bytes).to_bytes()).unwrap()
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
