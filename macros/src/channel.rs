use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use bytesize::ByteSize;
use darling::{FromAttributes, FromDeriveInput, FromMeta};
use proc_macro2::Ident;
use strum::{Display, EnumDiscriminants, EnumIter, EnumString, IntoEnumIterator};
// use strum::{Display, EnumString, EnumVariantNames, VariantNames};
use syn::{spanned::Spanned, Attribute, Item, ItemStruct, Meta};

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum QueuingDiscipline {
    FIFO,
    Priority,
}

impl FromMeta for QueuingDiscipline {
    fn from_string(value: &str) -> darling::Result<Self> {
        match QueuingDiscipline::from_str(value) {
            Ok(d) => Ok(d),
            Err(e) => Err(darling::Error::unsupported_shape(&e.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WrappedByteSize(ByteSize);

impl FromMeta for WrappedByteSize {
    fn from_string(value: &str) -> darling::Result<Self> {
        match ByteSize::from_str(value) {
            Ok(s) => Ok(WrappedByteSize(s)),
            Err(e) => Err(darling::Error::unsupported_shape(&e)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WrappedDuration(Duration);

impl FromMeta for WrappedDuration {
    fn from_string(value: &str) -> darling::Result<Self> {
        match humantime::parse_duration(value) {
            Ok(d) => Ok(WrappedDuration(d)),
            Err(e) => Err(darling::Error::unsupported_shape(&e.to_string())),
        }
    }
}

fn contains_attribute(attr: &str, attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .flat_map(|a| a.parse_meta())
        .flat_map(|m| m.path().get_ident().cloned())
        .any(|i| i.to_string().eq(attr))
}

trait MayFromAttributes: Sized {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>>;
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(sampling_out))]
pub struct SamplingOut {
    msg_size: WrappedByteSize,
}

impl MayFromAttributes for SamplingOut {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>> {
        if !contains_attribute("sampling_out", attrs) {
            return None;
        }
        Some(Self::from_attributes(attrs))
    }
}

impl From<SamplingOut> for Channel {
    fn from(s: SamplingOut) -> Self {
        Channel::SamplingOut(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(sampling_in))]
pub struct SamplingIn {
    msg_size: WrappedByteSize,
    refresh_period: WrappedDuration,
}

impl MayFromAttributes for SamplingIn {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>> {
        if !contains_attribute("sampling_in", attrs) {
            return None;
        }
        Some(Self::from_attributes(attrs))
    }
}

impl From<SamplingIn> for Channel {
    fn from(s: SamplingIn) -> Self {
        Channel::SamplingIn(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(queuing_out))]
pub struct QueuingOut {
    msg_size: WrappedByteSize,
    msg_count: usize,
    discipline: QueuingDiscipline,
}

impl MayFromAttributes for QueuingOut {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>> {
        if !contains_attribute("queuing_out", attrs) {
            return None;
        }
        Some(Self::from_attributes(attrs))
    }
}

impl From<QueuingOut> for Channel {
    fn from(s: QueuingOut) -> Self {
        Channel::QueuingOut(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(queuing_in))]
pub struct QueuingIn {
    msg_size: WrappedByteSize,
    msg_count: usize,
    discipline: QueuingDiscipline,
}

impl MayFromAttributes for QueuingIn {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>> {
        if !contains_attribute("queuing_in", attrs) {
            return None;
        }
        Some(Self::from_attributes(attrs))
    }
}

impl From<QueuingIn> for Channel {
    fn from(s: QueuingIn) -> Self {
        Channel::QueuingIn(s)
    }
}

#[derive(Debug, Clone, Display, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum Channel {
    SamplingOut(SamplingOut),
    SamplingIn(SamplingIn),
    QueuingOut(QueuingOut),
    QueuingIn(QueuingIn),
}

impl Channel {
    pub fn from_structs<'a>(items: &[ItemStruct]) -> syn::Result<HashMap<Ident, Channel>> {
        // let channel = SamplingOut::from_attributes(&a.attrs).unwrap();
        let mut channel: HashMap<Ident, Channel> = HashMap::new();
        for item in items {
            let mut vec: Vec<Option<darling::Result<Channel>>> = vec![
                SamplingOut::may_from_attributes(&item.attrs).map(|x| x.map(Channel::from)),
                SamplingIn::may_from_attributes(&item.attrs).map(|x| x.map(Channel::from)),
                QueuingOut::may_from_attributes(&item.attrs).map(|x| x.map(Channel::from)),
                QueuingIn::may_from_attributes(&item.attrs).map(|x| x.map(Channel::from)),
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
                    item,
                    "Multiple Channels defined on same struct",
                )),
            }?;
            channel.insert(item.ident.clone(), ch);
        }
        Ok(channel)
    }
}

// impl TryFrom<ItemStruct> for Channel {
//     type Error = Option<syn::Error>;

//     fn try_from(item: ItemStruct) -> Result<Self, Self::Error> {
//         let attributes = item
//             .attrs
//             .iter()
//             .filter(|f| f.path.get_ident().is_some())
//             .filter(|f| {
//                 Channel::VARIANTS.contains(&f.path.get_ident().unwrap().to_string().as_str())
//             })
//             .collect::<Vec<_>>();
//         let attr = match attributes.len() {
//             0 => return Err(None),
//             1 => attributes[0],
//             _ => {
//                 return Err(Some(syn::Error::new_spanned(
//                     item,
//                     "Only a single channel attribute is supported at the moment",
//                 )))
//             }
//         };
//         let meta = attr.parse_meta()?;
//         let meta_list = match meta {
//             Meta::List(list) => list,
//             _ => {
//                 return Err(Some(syn::Error::new_spanned(
//                     meta,
//                     "expected a list-syle attribute",
//                 )))
//             }
//         };

//         panic!("A {meta_list:#?}")
//     }
// }
