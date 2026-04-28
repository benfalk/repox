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

impl TokenGenerator<'_, crate::EntityInput> {
    /// Creates n `repox::Identity` impl for the entity if one of the
    /// following scenarios is true.  This also will implement the Entity
    /// trait.
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
                impl ::repox::Entity for #name {}
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
            impl ::repox::Entity for #name {}
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

#[cfg(feature = "mock")]
impl TokenGenerator<'_, crate::MockallInput> {
    /// # Original Trait Definition
    ///
    /// This is just a simple method that returns the original trait definition
    /// that was annotated with `#[repox::mockall]`.  While additional code is
    /// generated for the mock, the original trait definition is still needed
    /// and can remain unchanged.
    ///
    /// ---
    pub fn original_trait_def(&self) -> TokenStream {
        let original_trait_def = &self.input.original_trait_def;
        quote! { #original_trait_def }
    }

    /// # Manual Mockall Block
    ///
    /// This returns the full `::mockall::mock!` block that is needed to produce
    /// A fully stubbed mock implementation of the original trait.  This is a bit of
    /// a brute-force approach, but it allows us to generate a mock that is fully
    /// compatible with the original trait and all of the various repo traits that
    /// it may implement, without requiring the user to write any additional code
    /// or to worry about keeping the plumbing of the mock up to date with the
    /// original trait definition.
    ///
    /// ---
    pub fn manual_mockall_block(&self) -> TokenStream {
        let trait_name = &self.input.trait_name;

        quote! {
            ::mockall::mock! {
                pub #trait_name {}
                impl #trait_name for #trait_name {}
                impl ::repox::mock::StubRepo for #trait_name {}
                impl ::repox::Repo for #trait_name {
                    fn delete_by_id<T>(&self,id: T::ID)
                    -> impl Send + Future<
                        Output = Result<::repox::DeleteStatus, ::anyhow::Error>
                    >
                    where
                        T: ::repox::Entity,
                        Self: ::repox::DeleteById<T>,
                    {
                        unreachable!()
                    }

                    fn fetch_by_id<T>(
                        &self,
                        id: T::ID,
                    ) -> impl Send + Future<
                        Output = Result<T, ::repox::FetchError<T>>
                    >
                    where
                        T: ::repox::Entity,
                        Self: ::repox::FetchById<T>,
                    {
                        unreachable!()
                    }

                    fn fetch_by_id_optional<T>(
                        &self,
                        id: T::ID,
                    ) -> impl Send + Future<
                        Output = Result<Option<T>, ::anyhow::Error>
                    >
                    where
                        T: ::repox::Entity,
                        Self: ::repox::FetchById<T>,
                    {
                        unreachable!()
                    }

                    fn fetch_with_parent_by_id<T, O>(
                        &self,
                        id: T::ID,
                    ) -> impl Send + Future<
                        Output = Result<(T, O), ::repox::FetchWithParentError<T>>
                    >
                    where
                        T: ::repox::BelongsToForeignKey<O>,
                        O: ::repox::Entity,
                        Self: ::repox::FetchWithParentById<T, O>,
                    {
                        unreachable!()
                    }

                    fn fetch_with_children_by_id<T, O>(
                        &self,
                        id: T::ID,
                    ) -> impl Send + Future<
                        Output = Result<(T, Vec<O>), ::repox::FetchWithChildrenError<T>>
                    >
                    where
                        T: ::repox::HasManyForeignKey<O>,
                        O: ::repox::Entity,
                        Self: ::repox::FetchWithChildrenById<T, O>,
                    {
                        unreachable!()
                    }

                    fn create_with<T, P>(
                        &self,
                        payload: P,
                    ) -> impl Send + Future<Output = Result<T, ::anyhow::Error>>
                    where
                        T: ::repox::Entity,
                        P: ::repox::Creatable<T>,
                        Self: ::repox::CreateWith<T, P>,
                    {
                        unreachable!()
                    }

                    fn update_by_id<T>(
                        &self,
                        entity: T,
                    ) -> impl Send + Future<
                        Output = Result<(), ::repox::UpdateError<T>>
                    >
                    where
                        T: ::repox::Entity,
                        Self: ::repox::UpdateById<T>,
                    {
                        unreachable!()
                    }

                    fn insert<T>(
                        &self,
                        entity: T,
                    ) -> impl Send + Future<
                        Output = Result<(), ::repox::InsertError<T>>
                    >
                    where
                        T: ::repox::Entity,
                        Self: ::repox::Insert<T>,
                    {
                        unreachable!()
                    }
                }
            }
        }
    }
}
