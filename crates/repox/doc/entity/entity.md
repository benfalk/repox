# **`#entity`**

> 🏭 **Macro**
>
> > ```text
> > #[entity(id)]
> > ```
> >
> > Provides a quick way to identify a custom field as the ID for an entity.
>
> 📝 _Macro Example:_
>
> > ```rust
> > use repox::{Entity, Identity};
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct UserPasswordHash {
> >     #[entity(id)]
> >     pub user_id: u32,
> >     pub data: Vec<u8>,
> > }
> >
> > // Quick Demo of the ID in action:
> > let hash = UserPasswordHash { user_id: 42, data: vec![1, 2, 3] };
> > assert_eq!(hash.id(), 42);
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro. 🧙‍♂️
>
> > ```rust
> > use repox::{Entity, Identity};
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct UserPasswordHash {
> >     pub user_id: u32,
> >     pub data: Vec<u8>,
> > }
> >
> > // this is what the `entity(id)` macro generates for us
> > impl Identity for UserPasswordHash {
> >     type ID = u32;
> >     fn id(&self) -> Self::ID { self.user_id }
> > }
> >
> > // Quick Demo of the ID in action:
> > let hash = UserPasswordHash { user_id: 67, data: vec![1, 2, 3] };
> > assert_eq!(hash.id(), 67);
> > ```
