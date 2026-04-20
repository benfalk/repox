# **`repox`**

> **Introduction**
>
> > `repox::` is a trait interface framework for building repositories
> > with different kinds of data access needs. This crate has three main goals:
> >
> > - Provide application developers simple traits that describe what kind
> >   data access interface they need between entities and a repository.
> >
> > - Supply tooling to make implementing and defining repositories and entities
> >   much easier by removing a lot of needless boilerplate.
> >
> > - Maintain thorough, high-quality, documentation to lower the cognitive
> >   load on needing to remember constantly how to use features of this crate.

## Simple Blog Example

> Let's say you want to model blog posts for authors.  Here is a simple example
> of how you might use `repox::` to define your entities and repository
> interface for your application.  This example demonstrates how to use the
> various traits and how they are used.
>
> > ```rust
> > // Define some simple entities for a blog application
> >
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[has_many(Post.author_id)]
> > #[create_params(AuthorParams)]
> > pub struct Author {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[belongs_to(Author, author_id)]
> > #[create_params(PostParams)]
> > pub struct Post {
> >     pub id: u64,
> >     pub author_id: u32,
> >     pub title: String,
> >     pub content: String,
> > }
> >
> > // Define the repository interface for the blog entities
> >
> > pub trait BlogRepo:
> >     repox::Repo
> >
> >     // Only create is needed for authors in this example
> >     + repox::CreateWith<Author, AuthorParams>
> >
> >     // Posts can be created, read with an author, updated, and deleted
> >     + repox::CreateWith<Post, PostParams>
> >     + repox::FetchWithParentById<Post, Author>
> >     + repox::UpdateById<Post>
> >     + repox::DeleteById<Post>
> > {
> > }
> >
> > // Example usage of the BlogRepo
> >
> > async fn example_usage(repo: &impl BlogRepo) -> anyhow::Result<()> {
> >     // creating an author
> >     let author_params = AuthorParams { name: "GhostWriter".into() };
> >     let ghosty = repo.create_with(author_params).await?;
> >
> >     // giving them a post
> >     let post_params = PostParams {
> >         author_id: ghosty.id,
> >         title: "Scary Post".into(),
> >         content: "Booo!".into(),
> >     };
> >     let boo = repo.create_with(post_params).await?;
> >
> >     // updating the post
> >     let mut more_boo = boo.clone();
> >     more_boo.content = "Booooooooooooo!".into();
> >     repo.update_by_id(more_boo.clone()).await?;
> >
> >     // fetching post with its author
> >     let (post, author) = repo.fetch_with_parent_by_id(boo.id).await?;
> >     assert_eq!(post, more_boo);
> >     assert_eq!(author, ghosty);
> >
> >     // removing the post
> >     repo.delete_by_id(boo.id).await?;
> >
> >     // realizing this code compiles 🤯
> >     Ok(())
> > }
> > ```
