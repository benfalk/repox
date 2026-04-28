# **`belongs_to_key`**

> 📍 **Method**
>
> > ```text
> > belongs_to_key<T: Entity>(&self) -> T::ID
> > ```
> >
> > Extracts the foreign key value from the entity that references its
> > parent entity. This method is used in conjunction with the `#[belongs_to]`
> > attribute to identify which field in the entity struct serves as the
> > foreign key linking it to its parent entity. The method returns the
> > value of this foreign key, which is typically the unique identifier
> > of the parent entity. This allows you to easily access the parent
> > entity's ID when working with child entities that belong to it.
>
> 🧩 *Detailed Example:*
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[belongs_to(Widget, widget_id)]
> > pub struct WidgetProvision {
> >     pub id: u128,
> >     pub widget_id: u32,
> >     pub float: f64,
> > }
> >
> > let provision = WidgetProvision { id: 2, widget_id: 42, float: 1.337 };
> >
> > assert_eq!(provision.belongs_to_key::<Widget>(), 42);
> > ```
