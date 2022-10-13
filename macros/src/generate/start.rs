use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Item, ItemFn, TypePath};

use crate::parse::process::{Aperiodic, Periodic};
use crate::partition::Partition;

pub fn start_context_from_partition(part: &Partition) -> Item {
    let inits = init_process_fns(part);

    parse_quote! {
        mod start {
            pub(crate) struct Context{

            }

            impl Context{
                #(#inits)*
            }
        }
    }
}

fn init_process_fns(part: &Partition) -> Vec<ItemFn> {
    let mut fns = vec![];
    for (f, ap) in &part.processes {
        let name = format_ident!("init_{}", f.sig.ident);
        fns.push(parse_quote! {
            pub fn #name(&self) -> Result<(), apex_rs::prelude::Error>{
                todo!()
            }
        })
    }

    fns
}
