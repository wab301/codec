use proc_macro;
use proc_macro2;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::{punctuated::Punctuated,*, token::Comma, spanned::Spanned};
use crate::check;

pub fn impl_deserialize(ast: &syn::DeriveInput) -> TokenStream {
    use syn::Data::*;
    let name: &Ident = &ast.ident;
    match ast.data {
        Struct(DataStruct{fields: syn::Fields::Named(ref field), 
        ..}) => impl_deserialize_for_struct(name, &field.named, &ast.attrs, &ast.generics),
        _ => panic!("Deserialize only supports non-tuple structs"),
    }
}

fn impl_deserialize_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    _attrs: &[Attribute],
    _generics: &Generics,
) -> TokenStream {
    
    let gen_spans = gen_fileds(fields);

    let gen = quote! {
        impl #name {
            fn deserialize(buf: &mut bytes::BytesMut) -> #name {
                let mut instance = <#name as core::default::Default>::default();
                #gen_spans
                instance
            }
        }
    };
    gen.into()
}

fn gen_fileds(
    fields: &Punctuated<Field, Comma>,
) -> proc_macro2::TokenStream {
    let field_gen = fields.iter().map(|field| {
        if let Visibility::Public(_) = field.vis {
            gen_field(field)
        } else {
            quote_spanned!{ field.span() =>}
        }
    });

    quote! {
        #( #field_gen );*
    }
}

fn gen_field(field: &Field) -> proc_macro2::TokenStream {
    if let Some(field_name) = &field.ident {
        if let Type::Path(TypePath { path ,..}) = &field.ty {
            if check::is_number(path) {
                let func_name = format_ident!("get_{}_le", path.get_ident().unwrap());
                quote_spanned!{ field.span() =>
                    instance.#field_name = buf.#func_name();
                }
            } else if path.is_ident("u8") || path.is_ident("i8") {
                let func_name = format_ident!("get_{}", path.get_ident().unwrap());
                quote_spanned!{ field.span() =>
                    instance.#field_name = buf.#func_name();
                }
            } else if path.is_ident("String") {
                quote_spanned!{ field.span() =>
                    let len = buf.get_u16_le();
                    let mut data = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        data.put_u8(buf.get_u8());
                    }
                    instance.#field_name = String::from_utf8(data).unwrap();
                }
            } else if path.segments[0].ident == "Vec" {
                let gen_args = gen_arguments(field_name,&path.segments.first().unwrap().arguments);
                quote_spanned!{ field.span() =>
                    let len = buf.get_i32_le();
                    instance.#field_name = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        #gen_args
                    }
                }
            } else {
                quote_spanned!{ field.span() =>
                    instance.#field_name = #path::deserialize(buf);
                }
            }
        } else {
            quote_spanned!{ field.span() => }
        }
    }else {
        quote_spanned!{ field.span() => }
    }
}

fn gen_arguments(field_name: &Ident,arg: &PathArguments) -> proc_macro2::TokenStream  {
    if let PathArguments::AngleBracketed(ab) = arg {
        if let GenericArgument::Type(Type::Path(TypePath { path ,..})) = &ab.args.first().unwrap() {
            if check::is_number(path) {
                let func_name = format_ident!("get_{}_le", path.get_ident().unwrap());
                quote_spanned!{ arg.span() =>
                    instance.#field_name.push(buf.#func_name())
                }
            } else if path.is_ident("u8") || path.is_ident("i8") {
                let func_name = format_ident!("get_{}", path.get_ident().unwrap());
                quote_spanned!{ arg.span() =>
                    instance.#field_name.push(buf.#func_name())
                }
            } else if path.is_ident("String") {
                quote_spanned!{ arg.span() =>
                    let len = buf.get_u16_le();
                    let mut data = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        data.put_u8(buf.get_u8());
                    }
                    instance.#field_name.push(String::from_utf8(data).unwrap());
                }
            } else {
                quote_spanned!{ arg.span() =>
                    instance.#field_name.push(#path::deserialize(buf))
                }
            }
        } else {
            quote_spanned!{ arg.span() => }
        }
    }else {
        quote_spanned!{ arg.span() => }
    }
}



