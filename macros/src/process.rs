use syn::ItemFn;

pub enum Process {}

impl Process {
    pub fn from_structs<'a>(items: &[ItemFn]) -> syn::Result<Vec<Process>> {
        todo!("Process")
    }
}
