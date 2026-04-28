use syn::{
    ItemTrait,
    parse::{Parse, ParseStream},
};

pub struct MockallInput {
    pub original_trait_def: ItemTrait,
    pub trait_name: syn::Ident,
    // TODO: capture all sub traits to write out only
    // what is needed to keep build times lower.  One
    // thing to keep in mind is we can only scan for
    // raw names, we don't have access to the compiler
    // internals to know what the actual type is. This
    // means if a developer makes short aliases we're cooked.
    // A possible escape hatch would be to have the repo
    // `mockall` macro take an optional parameter that skips
    // niche trait optimizations and loads everything.
    // This is part of a larger issue; where we see
    // the compile times go to the moon with Elon.

    // TODO: So another idea is have one static repo
    // that get's compiled for N number of repositories
    // somehow? Would require at the very least understanding
    // mockall at a deeper level than we do today 🤔
}

impl Parse for MockallInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let original_trait_def: ItemTrait = input.parse()?;
        let trait_name = original_trait_def.ident.clone();
        let _traits = original_trait_def.supertraits.clone();
        Ok(Self {
            original_trait_def,
            trait_name,
        })
    }
}
