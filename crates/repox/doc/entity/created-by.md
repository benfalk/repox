# **`#created_by`**

> 🏭 **Macro**
>
> > ```text
> > #[created_by($struct_type)]
> > ```
> >
> > This macro is a simple way to implement the `Creatable<T>` trait for
> > a custom structure you want to use as parameters when creating a new
> > entity.  Today, this macro is just a convenient way to implement
> > the `Creatable<T>`; however, in the future, it may be extended to
> > support more features which are common for creation parameters.
>
> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[created_by(PostParams)]  // <- this structure can be used to create new
> > pub struct Post {          //    Post entities for supporting repositories
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > #[derive(Debug, Clone)]
> > pub struct PostParams {
> >     pub author_id: u32,
> >     pub title: String,      // <- it will be on the repository to decide
> >     pub content: String,    //    how to use these fields to create a Post
> > }
> >
> > // It can be used by repositories to create new entities
> > async fn create_sample_post(
> >     repository: &impl repox::CreateWith<Post, PostParams>,
> >     author_id: u32
> > ) -> Result<Post, anyhow::Error> {
> >     repository.create_with(PostParams {
> >         author_id,
> >         title: "Sample Title".into(),
> >         content: "Sample content...".into(),
> >     }).await
> > }
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro to show that
> > there is no magic here and that this macro is just a convenient way to
> > implement the `Creatable<T>` trait for a custom structure 🧙‍♂️.
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > #[derive(Debug, Clone)]
> > pub struct PostParams {
> >     pub author_id: u32,
> >     pub title: String,
> >     pub content: String,
> > }
> >
> > // without the following impl the `create_sample_post`
> > // would not compile since `CreateWith<T, P>` requires
> > // that `P` be `Creatable<T>`
> > impl repox::Creatable<Post> for PostParams {}
> >
> > // It can be used by repositories to create new entities
> > async fn create_sample_post(
> >     repository: &impl repox::CreateWith<Post, PostParams>,
> >     author_id: u32
> > ) -> Result<Post, anyhow::Error> {
> >     repository.create_with(PostParams {
> >         author_id,
> >         title: "Sample Title".into(),
> >         content: "Sample content...".into(),
> >     }).await
> > }
> > ```
