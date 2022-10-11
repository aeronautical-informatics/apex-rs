use darling::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, FnArg, Signature};

pub fn contains_attribute(attr: &str, attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .flat_map(|a| a.parse_meta())
        .flat_map(|m| m.path().get_ident().cloned())
        .any(|i| i.to_string().eq(attr))
}

pub trait MayFromAttributes: Sized {
    fn may_from_attributes(attrs: &[Attribute]) -> Option<darling::Result<Self>>;
}

///
/// Verify that no return type is specified in `output`  
/// `ident` is used for the error message only, declaring that no output is allowed for `ident`
pub fn no_return_type(ident: &str, output: &syn::ReturnType) -> syn::Result<()> {
    // TODO make '-> ()' a valid function format
    if let syn::ReturnType::Type(_, _) = output {
        return Err(syn::Error::new(
            output.span(),
            format!("{ident} outputs are not allowed"),
        ));
    }
    Ok(())
}

///
/// # Example
///
/// ```
/// single_function_argument(
///     &syn::Type::Path(syn::parse_str("warm::Context").unwrap()),
///     &fnItem.sig,
/// )?;
/// ```
pub fn single_function_argument(ty: &syn::Type, sig: &Signature) -> syn::Result<()> {
    let path: String = ty
        .to_token_stream()
        .to_string()
        .split_whitespace()
        .collect();
    let msg = format!("A single input is expected: {path}");
    if let Some(FnArg::Typed(t)) = sig.inputs.first() {
        if !ty.eq(&t.ty) {
            return Err(syn::Error::new_spanned(t.ty.clone(), msg));
        }
    } else {
        return Err(syn::Error::new(sig.paren_token.span, msg));
    }
    if sig.inputs.len() > 1 {
        return Err(syn::Error::new(sig.paren_token.span, msg));
    }
    Ok(())
}
