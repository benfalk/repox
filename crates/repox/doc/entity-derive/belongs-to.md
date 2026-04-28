# **`#belongs_to`**

> 🏭 **Macro**
>
> > ```text
> > #[belongs_to($target_type, $field_name)]
> > ```
> >
> > Implements a belongs-to relationship between the current entity and
> > the target entity.  The `$target_type` is another entity which this
> > entity belongs to.  `$field_name` is the field on this structure
> > which implements the relationship.  This field must be of the same type
> > as the `$target_type`'s ID.
>
> 📝 _Macro Example:_
>
> > This is a short example of how to use the `#belongs_to` macro. Here
> > we have `Post` which belongs to `Author` via the `author_id` field.
> >
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
> > #[belongs_to(Author, author_id)] // <- Post belongs to Author via author_id
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
> > assert!(post.belongs_to(&alice));
> > assert!(!post.belongs_to(&darin));
> > assert_eq!(post.belongs_to_key::<Author>(), 1);
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro and
> > implementing it ourselves.  This highlights the fact that this
> > macro is just a convenient way to implement the `BelongsToForeignKey`
> > trait and if you have a more complex relationship, you can implement
> > it yourself without the macro.  There is no magic here 🧙‍♂️!
> >
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
> > impl ::repox::BelongsToForeignKey<Author> for Post {
> >     fn key(&self) -> <Author as ::repox::Identity>::ID {
> >         self.author_id
> >     }
> > }
> >
> > // Same Demo of the relationship in action:
> > let alice = Author { id: 1, name: "Alice".to_string() };
> > let darin = Author { id: 2, name: "Darin".to_string() };
> > let post = Post { id: 100, author_id: 1, data: "Hello World".to_string() };
> > assert!(post.belongs_to(&alice));
> > assert!(!post.belongs_to(&darin));
> > assert_eq!(post.belongs_to_key::<Author>(), 1);
> > ```
> >
> > _Note: Still deriving `Entity` to avoid extra boilerplate above._
