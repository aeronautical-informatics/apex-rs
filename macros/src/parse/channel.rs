use std::str::FromStr;

use darling::{FromAttributes, FromMeta};
use proc_macro2::Ident;
use strum::{Display, EnumDiscriminants, EnumIter, EnumString};
// use strum::{Display, EnumString, EnumVariantNames, VariantNames};
use syn::{spanned::Spanned, Attribute, ItemStruct};

use crate::parse::util::{
    contains_attribute, remove_attributes, MayFromAttributes, WrappedByteSize, WrappedDuration,
};

#[derive(Debug, Clone, PartialEq, EnumString)]
pub enum QueuingDiscipline {
    FIFO,
    Priority,
}

impl FromMeta for QueuingDiscipline {
    fn from_string(value: &str) -> darling::Result<Self> {
        QueuingDiscipline::from_str(value)
            .map_err(|e| darling::Error::unsupported_shape(&e.to_string()))
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(sampling_out))]
pub struct SamplingOutProc {
    msg_size: WrappedByteSize,
}

impl MayFromAttributes for SamplingOutProc {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("sampling_out", attrs) {
            return None;
        }
        let port = Some(Self::from_attributes(attrs));
        Some(remove_attributes("sampling_out", attrs))?.ok();
        port
    }
}

impl From<SamplingOutProc> for Channel {
    fn from(s: SamplingOutProc) -> Self {
        Channel::SamplingOut(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(sampling_in))]
pub struct SamplingInProc {
    msg_size: WrappedByteSize,
    refresh_period: WrappedDuration,
}

impl MayFromAttributes for SamplingInProc {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("sampling_in", attrs) {
            return None;
        }
        let port = Some(Self::from_attributes(attrs));
        Some(remove_attributes("sampling_in", attrs))?.ok();
        port
    }
}

impl From<SamplingInProc> for Channel {
    fn from(s: SamplingInProc) -> Self {
        Channel::SamplingIn(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(queuing_out))]
pub struct QueuingOutProc {
    msg_size: WrappedByteSize,
    msg_count: usize,
    discipline: QueuingDiscipline,
}

impl MayFromAttributes for QueuingOutProc {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("queuing_out", attrs) {
            return None;
        }
        let port = Some(Self::from_attributes(attrs));
        Some(remove_attributes("queuing_out", attrs))?.ok();
        port
    }
}

impl From<QueuingOutProc> for Channel {
    fn from(s: QueuingOutProc) -> Self {
        Channel::QueuingOut(s)
    }
}

#[derive(Debug, Clone, FromAttributes)]
#[darling(attributes(queuing_in))]
pub struct QueuingInProc {
    msg_size: WrappedByteSize,
    msg_count: usize,
    discipline: QueuingDiscipline,
}

impl MayFromAttributes for QueuingInProc {
    fn may_from_attributes(attrs: &mut Vec<Attribute>) -> Option<darling::Result<Self>> {
        if !contains_attribute("queuing_in", attrs) {
            return None;
        }
        let port = Some(Self::from_attributes(attrs));
        Some(remove_attributes("queuing_in", attrs))?.ok();
        port
    }
}

impl From<QueuingInProc> for Channel {
    fn from(s: QueuingInProc) -> Self {
        Channel::QueuingIn(s)
    }
}

#[derive(Debug, Clone, Display, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
pub enum Channel {
    SamplingOut(SamplingOutProc),
    SamplingIn(SamplingInProc),
    QueuingOut(QueuingOutProc),
    QueuingIn(QueuingInProc),
}

impl Channel {
    pub fn from_structs<'a>(items: &mut [&mut ItemStruct]) -> syn::Result<Vec<(Ident, Channel)>> {
        // let channel = SamplingOut::from_attributes(&a.attrs).unwrap();
        let mut channel = vec![];
        for item in items {
            // let item = *item;
            let mut vec: Vec<Option<darling::Result<Channel>>> = vec![
                SamplingOutProc::may_from_attributes(&mut item.attrs).map(|x| x.map(Channel::from)),
                SamplingInProc::may_from_attributes(&mut item.attrs).map(|x| x.map(Channel::from)),
                QueuingOutProc::may_from_attributes(&mut item.attrs).map(|x| x.map(Channel::from)),
                QueuingInProc::may_from_attributes(&mut item.attrs).map(|x| x.map(Channel::from)),
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
            // item.attrs
            channel.push((item.ident.clone(), ch));
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
