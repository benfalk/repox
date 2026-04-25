use ::proc_macro::TokenStream;
use ::quote::quote;

mod entity_input;
#[cfg(feature = "mock")]
mod mockall_input;
mod token_generator;

use entity_input::EntityInput;
#[cfg(feature = "mock")]
use mockall_input::MockallInput;
use token_generator::TokenGenerator;

#[proc_macro_derive(
    Entity,
    attributes(entity, has_many, belongs_to, created_by, create_params, custom_id)
)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let input = ::syn::parse_macro_input!(input as EntityInput);
    let tokens = TokenGenerator::new(&input);

    let identity_impl = tokens.identity_impl();
    let belongs_to_impls = tokens.belongs_to_impls();
    let has_many_impls = tokens.has_many_impls();
    let created_by_impls = tokens.created_by_impls();
    let create_params_impls = tokens.create_params_impl();

    let expanded = quote! {
        #identity_impl
        #belongs_to_impls
        #has_many_impls
        #created_by_impls
        #create_params_impls
    };

    TokenStream::from(expanded)
}

#[cfg(feature = "mock")]
#[proc_macro_attribute]
pub fn mockall(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = ::syn::parse_macro_input!(item as MockallInput);
    let tokens = TokenGenerator::new(&input);

    let original_trait_def = tokens.original_trait_def();
    let manual_mockall_block = tokens.manual_mockall_block();

    let expanded = quote! {
        #original_trait_def
        #manual_mockall_block
    };

    TokenStream::from(expanded)
}
