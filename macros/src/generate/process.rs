use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_quote, Item, TypePath};

use crate::partition::Partition;

pub fn proc_contexts_from_partition(part: &Partition) -> Vec<Item> {
    let process_name = &part.hypervisor;

    // for (f, ap) in part.aperiodic {}
    // parse_quote! {
    //     mod #process_name {
    //         pub(crate) struct Context{

    //         }
    //     }
    // }
    // TODO implement this for real
    part.processes
        .iter()
        .map(|(f, _)| &f.sig.ident)
        .map(|i| {
            parse_quote! {
                mod #i {
                    pub(crate) struct Context{

                    }
                }
            }
        })
        .collect()
}
