use std::collections::HashMap;

use darling::FromAttributes;
use itertools::{Either, Itertools};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, Ident, ItemMod, Lit, Meta, NestedMeta,
};

use crate::channel::Channel;
use crate::start::Start;

pub struct Partition {
    channel: HashMap<Ident, Channel>,
    pre_start: Option<i32>,
    cold_start: i32,
    warm_start: i32,
    aperiodic: Vec<i32>,
    periodic: Vec<i32>,
}

impl Partition {
    fn from_mod(input: ItemMod) -> syn::Result<TokenStream> {
        let (_, items) = input.content.unwrap();
        let (functions, structs): (Vec<_>, Vec<_>) = items
            .iter()
            .filter(|f| match f {
                syn::Item::Fn(_) => true,
                syn::Item::Struct(_) => true,
                _ => false,
            })
            .partition_map(|p| match p {
                syn::Item::Fn(f) => Either::Left(f.clone()),
                syn::Item::Struct(s) => Either::Right(s.clone()),
                _ => panic!(),
            });
        let channel = Channel::from_structs(&structs)?;
        let start = Start::from_structs(&structs)?;

        todo!()
    }
}

pub fn expand_partition(input: ItemMod, hypervisor: Ident) -> syn::Result<TokenStream> {
    let mut token_stream = TokenStream::new();
    // let partition_name = input.ident;
    let part = Partition::from_mod(input)?;

    token_stream.extend(quote! {});
    Ok(token_stream)
}

///////////////////////////

fn get_name_attr(attr: &Attribute) -> syn::Result<Option<Ident>> {
    let meta = attr.parse_meta()?;
    let meta_list = match meta {
        Meta::List(list) => list,
        _ => {
            return Err(syn::Error::new_spanned(
                meta,
                "expected a list-syle attribute",
            ))
        }
    };

    let nested = match meta_list.nested.len() {
        0 => return Ok(None),
        1 => &meta_list.nested[0],
        _ => {
            return Err(syn::Error::new_spanned(
                meta_list.nested,
                "currently only a single getter attribute is supported",
            ))
        }
    };

    let name_value = match nested {
        NestedMeta::Meta(Meta::NameValue(nv)) => nv,
        _ => {
            return Err(syn::Error::new_spanned(
                nested,
                "expected `name = \"<value>\"`",
            ))
        }
    };

    if !name_value.path.is_ident("name") {
        return Err(syn::Error::new_spanned(
            &name_value.path,
            "unsupported getter attribute, expected `name`",
        ));
    }

    match &name_value.lit {
        Lit::Str(s) => syn::parse_str(&s.value()).map_err(|e| syn::Error::new_spanned(s, e)),
        lit => Err(syn::Error::new_spanned(lit, "expected string literal")),
    }
}

pub fn expand_getters(input: DeriveInput) -> syn::Result<TokenStream> {
    let input2 = input.clone();
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        data => {
            return Err(syn::Error::new_spanned(
                input2,
                "Only works for named field structs",
            ));
        }
    };

    let getters = fields
        .into_iter()
        .map(|f| {
            let attrs: Vec<_> = f
                .attrs
                .iter()
                .filter(|attr| attr.path.is_ident("getter"))
                .collect();
            let name_from_attr = match attrs.len() {
                0 => None,
                1 => get_name_attr(attrs[0])?,
                _ => {
                    let mut error =
                        syn::Error::new_spanned(attrs[1], "redundant `getter(name)` attribute");
                    error.combine(syn::Error::new_spanned(attrs[0], "note: first one here"));
                    return Err(error);
                }
            };

            let method_name =
                name_from_attr.unwrap_or_else(|| f.ident.clone().expect("a named field"));
            let field_name = f.ident;
            let field_ty = f.ty;

            Ok(quote! {
                pub fn #method_name(&self) -> &#field_ty {
                    &self.#field_name
                }
            })
        })
        .collect::<syn::Result<TokenStream>>()?;

    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #st_name #ty_generics #where_clause {
            #getters
        }
    })
}
