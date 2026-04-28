# **`#has_many`**

> 🏭 **Macro**
>
> > ```text
> > #[has_many($struct_name.$struct_foreign_key)]
> > ```
> >
> > Implements a has-many relationship between the current entity and
> > another entity.  The `$struct_name` is the name of the other entity
> > and the `$struct_foreign_key` is the field on that entity which
> > should equal the ID of this entity.  This macro is just a convenient
> > way to set up the relationship by implementing the `HasManyForeignKey`.
>
> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[has_many(Post.author_id)]   // <- Author has many Posts via Post.author_id
> > pub struct Author {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // Quick Demo of the relationship in action:
> > let alice = Author { id: 1, name: "Alice".to_string() };
> > let darin = Author { id: 2, name: "Darin".to_string() };
> > let post = Post { id: 100, author_id: 1, data: "Hello World".to_string() };
> > assert!(alice.is_owner_of(&post));
> > assert!(!darin.is_owner_of(&post));
> > assert_eq!(alice.has_many_key::<Post>(&post), 1);
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro.  You can see
> > that it's just a convenient way to set up the relationship by implementing
> > `HasManyForeignKey` for you.
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Author {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // this is what the #[has_many(Post.author_id)] macro expands to:
> > impl repox::HasManyForeignKey<Post> for Author {
> >     fn key(entity: &Post) -> <Author as repox::Identity>::ID {
> >         entity.author_id
> >     }
> > }
> >
> > // Quick Demo of the relationship in action:
> > let alice = Author { id: 1, name: "Alice".to_string() };
> > let darin = Author { id: 2, name: "Darin".to_string() };
> > let post = Post { id: 100, author_id: 1, data: "Hello World".to_string() };
> > assert!(alice.is_owner_of(&post));
> > assert!(!darin.is_owner_of(&post));
> > assert_eq!(alice.has_many_key::<Post>(&post), 1);
> > ```
