# **`repox`**

> **Introduction**
>
> > `repox::` is a trait interface framework for building repositories
> > with different kinds of data access needs. This crate has three main goals:
> >
> > - Provide application developers simple traits that describe what kind
> >   data access interface they need between entities and a repository.
> > - Supply tooling to make implementing and defining repositories and entities
> >   much easier by removing a lot of needless boilerplate.
> > - Maintain thorough, high-quality, documentation to lower the cognitive
> >   load on needing to remember constantly how to use features of this crate.

## Simple Blog Example

> Let's say you want to model blog posts for authors. Here is a simple example
> of how you might use `repox::` to define your entities and repository
> interface for your application. This example demonstrates how to use the
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
> > #[repox::mockall] // <- Testing as a first-class citizen with mockall
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
> >
> > // Now bear witness to the power of documentation. Behold how we
> > // can mock out the BlogRepo and test our example usage function
> > // with nary a line of implementation code written!
> > let mut blog = MockBlogRepo::new();
> >
> > // I personally like to import these helpers, but you do you
> > use repox::mock::{ok_with, ok_val};
> >
> > // an author is born
> > blog.expect_create_with()
> >     .returning(ok_with(|params: AuthorParams| {
> >         Author { id: 1, name: params.name }
> >     }));
> >
> > // their first post... will be created
> > blog.expect_create_with()
> >     .returning(ok_with(|params: PostParams| {
> >         Post {
> >             id: 1,
> >             author_id: params.author_id,
> >             title: params.title,
> >             content: params.content,
> >         }
> >     }));
> >
> > // an update will be made to the post, as it was foretold in the example
> > blog.expect_update_by_id::<Post>()
> >     .returning(ok_val(()));
> >
> > // data will be extracted for verification of the post and author
> > blog.expect_fetch_with_parent_by_id()
> >     .returning(ok_with(|id| (
> >         Post {
> >             id,
> >             author_id: 1,
> >             title: "Scary Post".into(),
> >             content: "Booooooooooooo!".into(),
> >         },
> >         Author {
> >             id: 1,
> >             name: "GhostWriter".into(),
> >         }
> >     )));
> >
> > // in a blind rage of drunken power; a deletion will occur...
> > blog.expect_delete_by_id::<Post>()
> >     .returning(ok_val(::repox::DeleteStatus::Deleted));
> >
> > // Now all will happen as it was documented, and our example
> > // usage will be verified by these tests... That's Hawt 🔥
> > # pollster::block_on(async {
> > example_usage(&blog).await.expect("Demo to work!");
> > # });
> > ```

---

> _Documentation Prayer_
>
> > ```text
> > By the documentation, they shall be known.
> > With the documentation, they will be used.
> >
> > May the documented features of this project
> > be a beacon to both man **and** machine. Let
> > us rejoice as we read detailed documentation,
> > and let us rejoice even more as we write it.
> >
> > By the Holy Documentation of the Omnissiah,
> > may our code be free of heresy and our features
> > catalogued in the Codex Repox. For the Machine
> > Spirit rejoices in well-commented functions,
> > and the Tech-Priests sing praises to the README
> > eternal!
> >
> > By the documentation, they shall be known.
> > With the documentation, they will be used.
> > ```
