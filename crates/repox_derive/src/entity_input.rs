use ::syn::{
    Attribute, Data, DeriveInput, Fields, Ident, Meta,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
};

pub type StructFields = Punctuated<syn::Field, Comma>;

/// # Derive Entity Input
///
/// This is all of the captured state needed to produce the final
/// token-stream for a derived Entity. It is meant to be parsed
/// from the token stream provided to the `Entity` derive macro.
/// If you want to understand what information is captured for
/// an input struct this is the place to look at.
///
/// [derive input]: DeriveInput
/// ---
#[derive(Debug)]
pub struct EntityInput {
    /// Name of the struct for which repox::Entity is derived
    pub name: Ident,

    /// All fields of the struct for which repox::Entity is derived
    pub fields: StructFields,

    /// Field found to be the ID field for repox::Identity
    ///
    /// Either tagged by a field with an attribute `#[entity(id)]`
    /// or a field named `id`. If both are present the tagged field
    /// will be used and the untagged `id` field will be ignored.
    pub id_field: Option<syn::Field>,

    /// Provides information for: `#[custom_id($id_type, $func_path)]`
    ///
    /// Only one of these tags is allowed.  This is used to identify a
    /// custom function to generate an ID for this entity along with the
    /// type of that ID.
    pub custom_id_tag: Option<CustomIdTag>,

    /// Tags for: `#[created_by($creator_type)]`
    ///
    /// These are used to identify existing structs that can be used
    /// to create instances of this entity from a repo
    pub created_by_tags: Vec<CreatedByTag>,

    /// Tags for: `#[create_params($params_type, $field_selection_strategy)]`
    ///
    /// These are used to identify structs to generate along with
    /// which fields to select from those structs.  These structs
    /// should be able to create instances of this entity from a repo.
    pub create_params_tags: Vec<CreateParamsTag>,

    /// Tags for: `#[belongs_to($target_type, $field_name)]`
    ///
    /// These identity other entities that this entity belongs to along
    /// with the fields this host entity uses to reference the target
    /// entities IDs.  This is the inverse of a `has_many` tag.
    pub belongs_to_tags: Vec<BelongsToTag>,

    /// Tags for: `#[has_many($target_type.$field_name)]`
    ///
    /// These identify other entities that this entity has many of
    /// along with the the fields on the target entities that reference
    /// this entity's ID.  This is the inverse of a `belongs_to` tag.
    pub has_many_tags: Vec<HasManyTag>,
}

/// Tag for: `#[created_by($creator_type)]`
#[derive(Debug)]
pub struct CreatedByTag {
    /// Value for: `#[created_by($creator_type)]`
    pub creator_type: syn::Type,
}

/// Tag for: `#[belongs_to($target_type, $field_name)]`
#[derive(Debug)]
pub struct BelongsToTag {
    pub target_type: syn::Type,
    pub field_name: Ident,
}

/// Tag for: `#[has_many($target_type.$field_name)]`
#[derive(Debug)]
pub struct HasManyTag {
    pub target_type: syn::Type,
    pub field_name: Ident,
}

/// Tag for: `#[create_params($params_type, $field_selection_strategy)]`
#[derive(Debug)]
pub struct CreateParamsTag {
    pub struct_name: Ident,
    pub selector: FieldSelectionStrategy,
}

/// Tag for: `#[custom_id($id_type, $func_path)]`
#[derive(Debug)]
pub struct CustomIdTag {
    pub id_type: syn::Type,
    pub func_path: syn::Path,
}

/// Determines fields to use from the Entity struct when
/// it needs to use some potentially reduced set of fields
/// for a generated struct (e.g. the create params struct).
#[derive(Debug)]
pub enum FieldSelectionStrategy {
    /// Takes all fields from the Entity ( including the ID field )
    All,

    /// Keeps all fields except the ID field (if it exists)
    ExcludingId,

    /// Selects all fields found in this list
    Only(Vec<Ident>),

    /// Selects all fields **NOT** found in this list
    Excluding(Vec<Ident>),
}

impl Parse for EntityInput {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        // TODO: Better error handling
        let input = input.parse::<DeriveInput>()?;
        let name = input.ident.clone();
        let fields = parse_struct_fields(&input)?;
        let id_field = parse_id_field(&fields)?;

        let mut created_by_tags = Vec::new();
        let mut belongs_to_tags = Vec::new();
        let mut has_many_tags = Vec::new();
        let mut create_params_tags = Vec::new();
        let mut custom_id_tag = None;

        for attr in &input.attrs {
            match attr.path().get_ident() {
                Some(ident) if ident == "belongs_to" => {
                    belongs_to_tags.push(attr.parse_args()?);
                }
                Some(ident) if ident == "has_many" => {
                    has_many_tags.push(attr.parse_args()?);
                }
                Some(ident) if ident == "created_by" => {
                    created_by_tags.push(attr.parse_args()?);
                }
                Some(ident) if ident == "create_params" => {
                    create_params_tags.push(attr.parse_args()?);
                }
                Some(ident) if ident == "custom_id" => {
                    if custom_id_tag.is_some() {
                        return Err(::syn::Error::new_spanned(
                            attr,
                            "Only one #[custom_id] tag is allowed",
                        ));
                    }
                    custom_id_tag = Some(attr.parse_args()?);
                }
                _ => {}
            }
        }

        Ok(Self {
            name,
            fields,
            id_field,
            created_by_tags,
            belongs_to_tags,
            has_many_tags,
            create_params_tags,
            custom_id_tag,
        })
    }
}

impl Parse for CreatedByTag {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        let creator_type = input.parse()?;
        Ok(Self { creator_type })
    }
}

impl Parse for CustomIdTag {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        let id_type: syn::Type = input.parse()?;
        let _: Comma = input.parse()?;
        let func_path = input.parse()?;
        Ok(Self { id_type, func_path })
    }
}

impl Parse for BelongsToTag {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        let target_type = input.parse()?;
        input.parse::<Comma>()?;
        let field_name = input.parse()?;
        Ok(Self {
            target_type,
            field_name,
        })
    }
}

impl Parse for HasManyTag {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        let target_type = input.parse()?;
        input.parse::<syn::token::Dot>()?;
        let field_name = input.parse()?;
        Ok(Self {
            target_type,
            field_name,
        })
    }
}

impl Parse for CreateParamsTag {
    fn parse(input: ParseStream) -> ::syn::Result<Self> {
        let struct_name = input.parse()?;
        let field_selection_strategy = if input.peek(Comma) {
            input.parse::<Comma>()?;
            let strategy_ident: Ident = input.parse()?;
            let content;
            syn::parenthesized!(content in input);
            let idents = Punctuated::<Ident, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect::<Vec<Ident>>();

            match strategy_ident.to_string().as_str() {
                "all" if idents.is_empty() => FieldSelectionStrategy::All,
                "excluding_id" if idents.is_empty() => {
                    FieldSelectionStrategy::ExcludingId
                }
                "only" => FieldSelectionStrategy::Only(idents),
                "excluding" => FieldSelectionStrategy::Excluding(idents),
                _ => {
                    return Err(::syn::Error::new_spanned(
                        strategy_ident,
                        "Expected 'only', 'excluding', 'all', or 'excluding_id'",
                    ));
                }
            }
        } else {
            FieldSelectionStrategy::ExcludingId
        };
        Ok(Self {
            struct_name,
            selector: field_selection_strategy,
        })
    }
}

impl FieldSelectionStrategy {
    pub fn select(&self, input: &EntityInput) -> Vec<syn::Field> {
        input
            .fields
            .iter()
            .filter(|field| self.can_keep(input, field))
            .cloned()
            .collect()
    }

    fn can_keep(&self, input: &EntityInput, field: &syn::Field) -> bool {
        match self {
            FieldSelectionStrategy::All => true,
            FieldSelectionStrategy::ExcludingId => input
                .id_field
                .as_ref()
                .is_none_or(|f| f.ident != field.ident),
            FieldSelectionStrategy::Only(idents) => field
                .ident
                .as_ref()
                .is_some_and(|ident| idents.contains(ident)),
            FieldSelectionStrategy::Excluding(idents) => field
                .ident
                .as_ref()
                .is_none_or(|ident| !idents.contains(ident)),
        }
    }
}

fn parse_struct_fields(input: &DeriveInput) -> ::syn::Result<StructFields> {
    let Data::Struct(data) = &input.data else {
        return Err(::syn::Error::new_spanned(
            input,
            "Entity can only be derived for structs",
        ));
    };
    match &data.fields {
        Fields::Named(fields) => Ok(fields.named.clone()),
        _ => Err(::syn::Error::new_spanned(
            input,
            "Entity can only be derived for structs with named fields",
        )),
    }
}

fn parse_id_field(fields: &StructFields) -> ::syn::Result<Option<syn::Field>> {
    let mut tagged_id: Option<syn::Field> = None;
    let mut named_id: Option<syn::Field> = None;

    // checks for #[entity(id)]
    fn has_entity_id_attr(attrs: &[Attribute]) -> bool {
        for attr in attrs {
            if !attr.path().is_ident("entity") {
                continue;
            }

            if let Ok(list) =
                attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
            {
                for meta in list {
                    let Meta::Path(path) = meta else { continue };
                    if path.is_ident("id") {
                        return true;
                    }
                }
            }
        }
        false
    }

    for field in fields {
        if has_entity_id_attr(&field.attrs) {
            if tagged_id.is_some() {
                return Err(syn::Error::new_spanned(
                    field,
                    "Only one field can be marked with #[entity(id)]",
                ));
            }
            tagged_id = Some(field.clone());
        }

        if let Some(ident) = &field.ident
            && ident == "id"
        {
            if named_id.is_some() {
                return Err(syn::Error::new_spanned(
                    field,
                    "Only one field can be named `id`",
                ));
            }
            named_id = Some(field.clone());
        }
    }

    Ok(tagged_id.or(named_id))
}
