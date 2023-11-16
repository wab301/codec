use proc_macro;
use proc_macro::TokenStream;
mod deserialize;
mod serialize;

#[macro_use]
mod check;

#[proc_macro_derive(Deserialize)]
pub fn deserialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    deserialize::impl_deserialize(&ast)
}

#[proc_macro_derive(Serialize)]
pub fn serialize_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    serialize::impl_serialize(&ast)
}