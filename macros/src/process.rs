use std::str::FromStr;
use std::string::ToString;

use darling::{FromAttributes, FromMeta};
use strum::{Display, EnumString};
use syn::spanned::Spanned;
use syn::{Attribute, ItemFn};

use crate::util::{
    contains_attribute, no_return_type, remove_attributes, single_function_argument,
    MayFromAttributes, WrappedByteSize, WrappedDuration,
};

#[derive(Debug, Clone, Display)]
pub enum ApexTime {
    Infinite,
    Normal(WrappedDuration),
}

impl FromMeta for ApexTime {
    fn from_string(value: &str) -> darling::Result<Self> {
        // TODO better suggestion for misspells
        if value.chars().any(|c| c.is_numeric()) {
            return Ok(Self::Normal(WrappedDuration::from_string(value)?));
        }

        if ApexTime::Infinite.to_string().eq_ignore_ascii_case(value) {
            return Ok(ApexTime::Infinite);
        }

        Err(darling::Error::unknown_value(
            "Expected either 'Infinite' or '<number><unit>'",
        ))
    }
}

#[derive(Debug, Clone, Display, EnumString)]
pub enum Deadline {
    Soft,
    Hard,
}

impl FromMeta for Deadline {
    fn from_string(value: &str) -> darling::Result<Self> {
        Deadline::from_str(value).map_err(|e| darling::Error::unsupported_shape(&e.to_string()))
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(aperiodic))]
pub struct Aperiodic {
    time_capacity: ApexTime,
    stack_size: WrappedByteSize,
    base_priority: usize,
    deadline: Deadline,
}

impl MayFromAttributes for Aperiodic {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("aperiodic", attrs) {
            return None;
        }
        let process = Some(Self::from_attributes(attrs));
        Some(remove_attributes("aperiodic", attrs))?.ok();
        process
    }
}

impl From<Aperiodic> for Process {
    fn from(a: Aperiodic) -> Self {
        Process::Aperiodic(a)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(periodic))]
pub struct Periodic {
    time_capacity: ApexTime,
    period: WrappedDuration,
    stack_size: WrappedByteSize,
    base_priority: usize,
    deadline: Deadline,
}

impl MayFromAttributes for Periodic {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("periodic", attrs) {
            return None;
        }
        let process = Some(Self::from_attributes(attrs));
        Some(remove_attributes("periodic", attrs))?.ok();
        process
    }
}

impl From<Periodic> for Process {
    fn from(p: Periodic) -> Self {
        Process::Periodic(p)
    }
}

#[derive(Debug, Clone, Display)]
pub enum Process {
    Aperiodic(Aperiodic),
    Periodic(Periodic),
}

impl Process {
    pub fn from_structs<'a>(items: &mut [&mut ItemFn]) -> syn::Result<Vec<(ItemFn, Process)>> {
        let mut procs = vec![];
        for item in items {
            let mut vec: Vec<Option<darling::Result<Process>>> = vec![
                Aperiodic::may_from_attributes(&mut item.attrs).map(|x| x.map(Process::from)),
                Periodic::may_from_attributes(&mut item.attrs).map(|x| x.map(Process::from)),
            ];
            let vec: Vec<_> = vec
                .drain(..)
                .flatten()
                .map(|c| c.map_err(|e| syn::Error::from(e.with_span(&item.span()))))
                .collect();
            let ch = match vec.len() {
                0 => continue,
                1 => Ok(vec[0].clone()?),
                _ => Err(syn::Error::new_spanned(
                    item.clone(),
                    "Multiple Channels defined on same struct",
                )),
            }?;

            single_function_argument(
                &syn::Type::Path(syn::parse_str(&format!("{}::Context", item.sig.ident)).unwrap()),
                &item.sig,
            )?;
            no_return_type("Process", &item.sig.output)?;

            procs.push((item.clone(), ch));
        }

        Ok(procs)
    }
}
