use super::EntityInput;
use ::proc_macro2::TokenStream;
use ::quote::quote;

/// # Token Generator
///
/// This is just a simple wrapper around some inputs data and provides
/// methods to generate source code tokens for the various structs and
/// impls that the input data indicates should be generated.  This is
/// just a way to split up the code generation logic into more manageable
/// pieces and to keep the code generation logic separate from the parsing
/// logic.
///
/// ---
pub struct TokenGenerator<'a, T> {
    input: &'a T,
}

impl<'a, T> TokenGenerator<'a, T> {
    pub fn new(input: &'a T) -> Self {
        Self { input }
    }
}

impl TokenGenerator<'_, EntityInput> {
    /// Creates n `repox::Identity` impl for the entity if one of the
    /// following scenarios is true:
    ///
    /// - `#[custom_id($id_type, $func_path)]`
    /// - `#[entity(id)]`
    /// - the struct has a field named `id`
    ///
    /// **NOTE** If no such field or tag is found then no impl is generated
    ///
    /// ---
    pub fn identity_impl(&self) -> TokenStream {
        let name = &self.input.name;

        if let Some(path) = self.input.custom_id_tag.as_ref() {
            let func = &path.func_path;
            let id_type = &path.id_type;
            return quote! {
                impl ::repox::Identity for #name {
                    type ID = #id_type;
                    fn id(&self) -> Self::ID {
                        #func(self)
                    }
                }
            };
        };

        let Some(id_field) = &self.input.id_field else {
            return Default::default();
        };
        let id_field_ident = &id_field.ident;
        let id_field_ty = &id_field.ty;

        quote! {
            impl ::repox::Identity for #name {
                type ID = #id_field_ty;
                fn id(&self) -> Self::ID {
                    self.#id_field_ident
                }
            }
        }
    }

    /// Converts any number of `#[belongs_to($target_type, $field_name)]` tags
    /// into implementations of `::repox::BelongsToForeignKey`.
    ///
    /// ---
    pub fn belongs_to_impls(&self) -> TokenStream {
        let name = &self.input.name;
        let impls = self.input.belongs_to_tags.iter().map(|tag| {
            let target_type = &tag.target_type;
            let field_name = &tag.field_name;

            quote! {
                impl ::repox::BelongsToForeignKey<#target_type> for #name {
                    fn key(&self) -> <#target_type as ::repox::Identity>::ID {
                        self.#field_name
                    }
                }
            }
        });

        quote! {
            #(#impls)*
        }
    }

    /// Converts any number of `#[has_many($target_type.$field_name)]` tags
    /// into impls for `::repox::HasManyForeignKey`
    ///
    /// ---
    pub fn has_many_impls(&self) -> TokenStream {
        let name = &self.input.name;
        let impls = self.input.has_many_tags.iter().map(|tag| {
            let target_type = &tag.target_type;
            let field_name = &tag.field_name;

            quote! {
                impl ::repox::HasManyForeignKey<#target_type> for #name {
                    fn key(entity: &#target_type) -> Self::ID {
                        entity.#field_name
                    }
                }
            }
        });

        quote! {
            #(#impls)*
        }
    }

    /// Converts any number of `#[create_params($struct_name, $op(...))]`
    /// tags into structs named `$struct_name` and adds fields described
    /// by the `$op`.
    ///
    /// By default if the `$op` is excluded the params struct will
    /// will include all fields from the entity **EXCEPT** the `ID`
    /// field (if it exists).  This is because the `ID` field is
    /// typically determined by the repository and should not be
    /// provided by the requester when creating a new entity. However,
    /// the `$op` all supports the following options:
    ///
    /// | Operation           | Description                             |
    /// |---------------------|-----------------------------------------|
    /// | `all()`             | Includes all fields from the entity     |
    /// | `excluding(foo_id)` | All fields except `foo_id` are included |
    /// | `only(foo, bar)`    | Only the specified fields are included  |
    /// |---------------------|-----------------------------------------|
    ///
    /// ---
    pub fn create_params_impl(&self) -> TokenStream {
        let name = &self.input.name;
        let impls = self.input.create_params_tags.iter().map(|tag| {
            let struct_name = &tag.struct_name;
            let fields = tag.selector.select(self.input).into_iter().map(|field| {
                let fname = &field.ident;
                let ftype = &field.ty;
                quote! {
                    pub #fname: #ftype,
                }
            });

            quote! {
                #[derive(Debug, Clone)]
                pub struct #struct_name {
                    #(#fields)*
                }
                impl ::repox::Creatable<#name> for #struct_name {}
            }
        });

        quote! {
            #(#impls)*
        }
    }

    /// Converts any number of `#[created_by($type)]` tags into generic
    /// impls for `::repox::Creatable`.
    ///
    /// This allows any type to be used as a "creator" for repositories
    /// that support creation of this entity, and allows the creator type
    /// to be decoupled from the standard create params struct (if one exists).
    ///
    /// ---
    pub fn created_by_impls(&self) -> TokenStream {
        let name = &self.input.name;
        let impls = self.input.created_by_tags.iter().map(|tag| {
            let target_type = &tag.creator_type;

            quote! {
                impl ::repox::Creatable<#name> for #target_type {}
            }
        });

        quote! {
            #(#impls)*
        }
    }
}
