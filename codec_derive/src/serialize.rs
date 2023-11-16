use proc_macro;
use proc_macro2;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned, format_ident};
use syn::{punctuated::Punctuated,*, token::Comma, spanned::Spanned};
use crate::check;

pub fn impl_serialize(ast: &syn::DeriveInput) -> TokenStream {
    use syn::Data::*;
    let name: &Ident = &ast.ident;
    match ast.data {
        Struct(DataStruct{fields: syn::Fields::Named(ref field), 
        ..}) => impl_serialize_for_struct(name, &field.named, &ast.attrs, &ast.generics),
        _ => panic!("Serialize only supports non-tuple structs"),
    }
}

fn impl_serialize_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    _attrs: &[Attribute],
    _generics: &Generics,
) -> TokenStream {
    
    let gen_spans = gen_fileds(fields);
    let gen = quote! {
        impl #name {
            fn serialize(&mut self,buf: &mut bytes::BytesMut) {
                #gen_spans
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
                let func_name = format_ident!("put_{}_le", path.get_ident().unwrap());
                quote_spanned!{ field.span() =>
                    buf.#func_name(self.#field_name);
                }
            } else if path.is_ident("u8") || path.is_ident("i8") {
                let func_name = format_ident!("put_{}", path.get_ident().unwrap());
                quote_spanned!{ field.span() =>
                    buf.#func_name(self.#field_name);
                }
            } else if path.is_ident("String") {
                quote_spanned!{ field.span() =>
                    buf.put_u16_le(self.#field_name.len() as u16);
                    buf.put_slice(self.#field_name.as_bytes());
                }
            } else if path.segments[0].ident == "Vec" {
                let gen_args = gen_arguments(field_name,&path.segments.first().unwrap().arguments);
                quote_spanned!{ field.span() =>
                    let len = self.#field_name.len();
                    buf.put_i32_le(len as i32);
                    #gen_args
                }
            } else {
                quote_spanned!{ field.span() =>
                    self.#field_name.serialize(buf);
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
                let func_name = format_ident!("put_{}_le", path.get_ident().unwrap());
                quote_spanned!{ arg.span() =>
                    for i in 0..len {
                        buf.#func_name(self.#field_name[i]);
                    }
                }
            } else if path.is_ident("u8") || path.is_ident("i8") {
                let func_name = format_ident!("put_{}", path.get_ident().unwrap());
                quote_spanned!{ arg.span() =>
                    for i in 0..len {
                        buf.#func_name(self.#field_name[i]);
                    }
                }
            } else if path.is_ident("String") {
                quote_spanned!{ arg.span() =>
                    for i in 0..len {
                        buf.put_u16_le(self.#field_name[i].len() as u16);
                        buf.put_slice(self.#field_name[i].as_bytes());
                    }
                }
            } else {
                quote_spanned!{ arg.span() =>
                    for i in 0..len {
                        self.#field_name[i].serialize(buf);
                    }
                }
            }
        } else {
            quote_spanned!{ arg.span() => }
        }
    }else {
        quote_spanned!{ arg.span() => }
    }
}