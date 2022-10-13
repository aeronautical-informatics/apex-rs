use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Data, DataStruct, DeriveInput, Fields, Ident, Lit, Meta, NestedMeta};
