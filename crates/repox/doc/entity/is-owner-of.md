# **`is_owner_of`**

> 📍 **Method**
>
> > ```text
> > is_owner_of<T: Entity>(&self, entity: &T) -> bool
> > ```
> >
> > Convenience method that checks if the entity is the owner of a specific child
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
> > let foo = Widget { id: 42, name: "foo".into() };
> > let bar = Widget { id: 13, name: "bar".into() };
> > let lucky_val = WidgetProvision { id: 2, widget_id: 42, float: 1.337 };
> >
> > assert!(foo.is_owner_of(&lucky_val));
> > assert!(!bar.is_owner_of(&lucky_val));
> > ```
