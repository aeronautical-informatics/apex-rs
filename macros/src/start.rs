use darling::util::Flag;
use darling::{FromAttributes, ToTokens};
use proc_macro2::{Ident, Span};
use strum::AsRefStr;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{Attribute, FnArg, ItemFn, ItemMod, PatType, Signature};

use crate::util::{contains_attribute, no_return_type, single_function_argument};

#[derive(Debug, Copy, Clone, AsRefStr)]
enum StartType {
    Pre(Span),
    Warm(Span),
    Cold(Span),
}

impl StartType {
    fn span(&self) -> &Span {
        match self {
            StartType::Pre(s) => s,
            StartType::Warm(s) => s,
            StartType::Cold(s) => s,
        }
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(start))]
struct StartFlags {
    pre: Flag,
    warm: Flag,
    cold: Flag,
}

impl TryFrom<StartFlags> for Option<StartType> {
    type Error = syn::Error;

    fn try_from(value: StartFlags) -> Result<Self, Self::Error> {
        let mut flags = vec![];
        if value.pre.is_present() {
            flags.push(StartType::Pre(value.pre.span()))
        }
        if value.warm.is_present() {
            flags.push(StartType::Warm(value.warm.span()))
        }
        if value.cold.is_present() {
            flags.push(StartType::Cold(value.cold.span()))
        }
        match flags.len() {
            0 => Ok(None),
            1 => Ok(Some(flags[0])),
            _ => {
                let mut flags = flags.iter();
                let mut err = syn::Error::new(
                    flags.next().unwrap().span().clone(),
                    "Multiple start flags attached to same function.",
                );
                for (i, flag) in flags.enumerate() {
                    err.combine(syn::Error::new(
                        flag.span().clone(),
                        format!("{}th flag", i + 2),
                    ))
                }
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Start {
    pre: Option<ItemFn>,
    warm: ItemFn,
    cold: ItemFn,
}

impl Start {
    pub fn pre(&self) -> Option<&ItemFn> {
        self.pre.as_ref()
    }

    pub fn warm(&self) -> &ItemFn {
        &self.warm
    }

    pub fn cold(&self) -> &ItemFn {
        &self.cold
    }

    fn verify_fn_form(self) -> syn::Result<Start> {
        if let Some(pre) = &self.pre {
            if !pre.sig.inputs.is_empty() {
                return Err(syn::Error::new(
                    pre.sig.inputs.span(),
                    "PreStart arguments are expected to be empty",
                ));
            }
            no_return_type("PreStart", &pre.sig.output)?;
        }

        single_function_argument(
            &syn::Type::Path(syn::parse_str("warm::Context").unwrap()),
            &self.warm.sig,
        )?;
        no_return_type("WarmStart", &self.warm.sig.output)?;

        single_function_argument(
            &syn::Type::Path(syn::parse_str("cold::Context").unwrap()),
            &self.cold.sig,
        )?;
        no_return_type("ColdStart", &self.cold.sig.output)?;

        Ok(self)
    }

    pub fn from_structs<'a>(root: &ItemMod, items: &[ItemFn]) -> syn::Result<Start> {
        let mut pre: Option<ItemFn> = None;
        let mut warm: Option<ItemFn> = None;
        let mut cold: Option<ItemFn> = None;
        for item in items {
            let start = StartFlags::from_attributes(&item.attrs)?;
            let start: Option<StartType> = start.try_into()?;
            let start = if let Some(start) = start {
                start
            } else {
                continue;
            };
            let leftover = match start {
                StartType::Pre(_) => pre.replace(item.clone()),
                StartType::Warm(_) => warm.replace(item.clone()),
                StartType::Cold(_) => cold.replace(item.clone()),
            };
            if let Some(leftover) = leftover {
                let mut err = syn::Error::new(
                    item.span().clone(),
                    format!("{}Start already defined", start.as_ref()),
                );
                err.combine(syn::Error::new(leftover.span(), "First definition here"));
                return Err(err);
            }
        }
        Start {
            pre,
            warm: warm
                .ok_or_else(|| syn::Error::new(root.span(), "No 'start(warm)' function defnied"))?,
            cold: cold
                .ok_or_else(|| syn::Error::new(root.span(), "No 'start(cold)' function defnied"))?,
        }
        .verify_fn_form()
    }
}
