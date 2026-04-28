# **`belongs_to`**

> 📍 **Method**
>
> > ```text
> > belongs_to<T: Entity>(&self, entity: &T) -> bool
> > ```
> >
> > A convenience method that checks if the entity belongs to a specific
> > parent entity. This method is used in conjunction with the `#[belongs_to]`
> > attribute to determine if the child entity is associated with the given
> > parent entity. It works by comparing the foreign key value extracted from
> > the child entity (using `belongs_to_key`) with the unique identifier of
> > the provided parent entity. If they match, it returns `true`, indicating
> > that the child entity belongs to the specified parent; otherwise, it
> > returns `false`.
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
> > let foo = Widget { id: 42, name: "foo".into() };
> > let bar = Widget { id: 13, name: "bar".into() };
> >
> > assert!(provision.belongs_to(&foo));
> > assert!(!provision.belongs_to(&bar));
> > ```
