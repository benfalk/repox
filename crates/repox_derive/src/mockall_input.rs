use syn::{
    ItemTrait,
    parse::{Parse, ParseStream},
};

pub struct MockallInput {
    pub original_trait_def: ItemTrait,
    pub trait_name: syn::Ident,
}

impl Parse for MockallInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let original_trait_def: ItemTrait = input.parse()?;
        let trait_name = original_trait_def.ident.clone();
        Ok(Self {
            original_trait_def,
            trait_name,
        })
    }
}
