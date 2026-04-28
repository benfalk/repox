# **`has_many_key`**

> 📍 **Method**
>
> > ```text
> > has_many_key<T: Entity>(&self, entity: &T) -> Self::ID
> > ```
> >
> > Extracts the foreign key value from the entity that references the
> > current entity as its parent. This method is used in conjunction with
> > the `#[has_many]` attribute to identify which field in the child entity
> > structure and field type serves as the foreign key linking it to the
> > parent entity.  This allows you to easily access the parent entity's
> > ID when working with entities without needing to know the specific
> > field name of the foreign key.
>
> 🧩 *Detailed Example:*
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[has_many(WidgetProvision.widget_id)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct WidgetProvision {
> >     pub id: u128,
> >     pub widget_id: u32,
> >     pub float: f64,
> > }
> >
> > let foo = Widget { id: 1, name: "foo".into() };
> > let provision = WidgetProvision { id: 2, widget_id: 42, float: 1.337 };
> >
> > assert_eq!(foo.has_many_key::<WidgetProvision>(&provision), 42);
> > ```
