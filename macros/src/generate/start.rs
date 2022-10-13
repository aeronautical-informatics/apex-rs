use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Item, ItemFn, TypePath};

use crate::parse::process::{Aperiodic, Periodic};
use crate::partition::Partition;

pub fn start_context_from_partition(part: &Partition) -> Item {
    let hyp_path = &part.hypervisor;
    let inits = init_process_fns(part);

    parse_quote! {
        mod start {
            pub type Context = ContextInner< #hyp_path >;

            pub struct ContextInner<Hypervisor: apex_rs::bindings::ApexPartitionP4> {
                _p: core::marker::PhantomData<Hypervisor>,
            }

            impl<Hypervisor:
                apex_rs::bindings::ApexPartitionP4 +
                apex_rs::bindings::ApexProcessP4,
            > ContextInner<Hypervisor>{
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
