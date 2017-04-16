#![feature(const_fn)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;


#[proc_macro_derive(EnumVariantCount)]
pub fn derive_enum_variant_count(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    let len = match ast.body {
        syn::Body::Enum(variants) => variants.len(),
        _ => panic!("Every type other than an enum has exactly 1 (one) variant"),
    };

    let ident = &ast.ident;
    let name = syn::Ident::from(format!("{}RSNEKDerivedLength", &ast.ident));
    let inline = syn::MetaItem::Word(syn::Ident::from("inline"));

    let gen = quote! {
        pub trait #name {

            #[#inline]
            fn count() -> usize {
                #len
            }
        }

        impl #name for #ident {}
    };

    gen.parse().unwrap()
}



