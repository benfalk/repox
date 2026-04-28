# **`#create_params`**

> 🏭 **Macro**
>
> > ```text
> > #[create_params($struct_name, $op(...) = excluding_id())]
> > ```
> >
> > This macro generates a new struct which can be used as parameters for
> > creating a new entity.  The generated struct's fields are the same as
> > the entity but without the ID field by default.  The following table
> > describes the operations that can be used in the macro:
> >
> > | operation      | description                                    |
> > | -------------- | ---------------------------------------------- |
> > | all()          | includes all fields from entity (including ID) |
> > | excluding_id() | includes all fields except the ID field        |
> > | only(...)      | will only contain these fields                 |
> > | excluding(...) | includes all fields except these fields        |
>
> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[create_params(PostParams)] // <- create with default `excluding_id()`
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // It can be used by repositories to create new entities
> > async fn create_sample_post(
> >     repository: &impl repox::CreateWith<Post, PostParams>,
> >     author_id: u32
> > ) -> Result<Post, anyhow::Error> {
> >     repository.create_with(PostParams { // <- missing `id` field since
> >         author_id,                      // we used `excluding_id()`
> >         data: "Sample Post".into(),
> >     }).await
> > }
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro and
> > implementing it ourselves.  This highlights the fact that this
> > macro is just a convenient way to implement the `CreateWith<T, P>`
> > trait for a freshly minted structure with the same fields as the
> > entity, but without the ID field by default. There is no magic here 🧙‍♂️!
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
> > // This is the exact same structure the macro above would generate
> > #[derive(Debug, Clone)]
> > pub struct PostParams {
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // Parameters need to implement `Creatable` for the entity
> > // they are compatible with.  For now this trait is empty,
> > // but in the future it may contain some useful functionality.
> > impl ::repox::Creatable<Post> for PostParams {}
> >
> > // It can be used by repositories to create new entities
> > async fn create_sample_post(
> >     repository: &impl repox::CreateWith<Post, PostParams>,
> >     author_id: u32
> > ) -> Result<Post, anyhow::Error> {
> >     repository.create_with(PostParams { // <- missing `id` field since
> >         author_id,                      // we used `excluding_id()`
> >         data: "Sample Post".into(),
> >     }).await
> > }
> > ```

## **`all()`**

> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[create_params(PostParams, all())]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // All of the fields are included
> > let post = PostParams { id: 100, author_id: 1, data: "Hello World".into() };
> > ```

## **`excluding(...)`**

> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[create_params(PostPlaceholder, excluding(id, data))]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // only has `author_id` since we excluded `id` and `data`
> > let post = PostPlaceholder { author_id: 1 };
> > ```

## **`only(...)`**

> 📝 _Macro Example:_
>
> > ```rust
> > use repox::Entity;
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[create_params(AnonPostData, only(data))]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub data: String,
> > }
> >
> > // only has `data` field
> > let post = AnonPostData { data: "You too can example.".into() };
> > ```
