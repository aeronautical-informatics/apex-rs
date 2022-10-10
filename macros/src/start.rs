use syn::ItemStruct;

pub struct Start {}

impl Start {
    pub fn from_structs<'a>(items: &[ItemStruct]) -> syn::Result<Self> {
        todo!()
    }
}
