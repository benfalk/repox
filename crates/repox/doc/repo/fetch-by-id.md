# **`fetch_by_id`**

> 📍 **Method**
>
> > ```text
> > fetch_by_id(id: ID) -> Result<Entity, repox::FetchError<Entity>>
> > ```
> >
> > Fetches a single entity from the repository using its unique
> > identifier. This will return `Ok(entity)` if an entity with
> > the given ID exists.  The `Err` variant will be a `FetchError`
> > which can be used to determine if the fetch failed because
> > the entity was not found.
>
> 🧩 *Detailed Example:*
>
> > The suites at the top are getting a little worried at not being
> > able to to recall widgets that have been created.  Let's give
> > them access to the `fetch_by_id` method by implementing the
> > [`FetchById<T>`] trait for our `WidgetData` repository.
> >
> > This will allow us to retrieve widgets using their unique
> > identifier, which is essential for many applications where you
> > need to access specific entities without fetching the entire
> > collection.
> >
> > ```rust
> > use repox::Repo;
> >
> > // Define an entity
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > // We can fetch them by ID and enjoy them for all time
> > pub trait FetchingRepo: Repo + repox::FetchById<Widget> {}
> >
> > // Our new high-end data store that only works with widgets
> > #[derive(Debug, Default)]
> > pub struct WidgetData {
> >     pub data: dashmap::DashMap<u32, Widget>,
> > }
> >
> > // lets put all the worry to rest by implementing the necessary
> > // traits to make this a fetching repo for any occasion
> > impl Repo for WidgetData {}
> > impl FetchingRepo for WidgetData {}
> >
> > impl repox::FetchById<Widget> for WidgetData {
> >     async fn exec(&self, id: u32)
> >     -> Result<Widget, repox::FetchError<Widget>>
> >     {
> >         match self.data.get(&id) {
> >             Some(widget_ref) => Ok(widget_ref.clone()),
> >             None => Err(repox::FetchError::NotFound(id)),
> >         }
> >     }
> > }
> >
> > // okay, cool, but we should make sure it works ya know?
> > # pollster::block_on(async {
> > let repo = WidgetData::default();
> > let widget = Widget {
> >     id: 42,
> >     name: "FistFullOfDollars".into(),
> > };
> > repo.data.insert(widget.id, widget.clone());
> >
> > // Hold on to your butts, because we're about to
> > // fetch this fist full of dollars by ID!
> > let ffod = repo.fetch_by_id(42).await.unwrap();
> > assert_eq!(ffod, widget);
> >
> > // Tight! 🕺
> > # });
> > ```
>
> 🧪 *Mock Example:*
>
> > ```rust
> > use repox::{Repo, Entity};
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[repox::mockall]
> > pub trait WidgetFetcher: Repo + repox::FetchById<Widget> {}
> >
> > let mut repo = MockWidgetFetcher::new();
> > repo.expect_fetch_by_id::<Widget>()
> >     .withf(|id| *id == 42)
> >     .returning(repox::mock::ok_with(|id: u32| Widget {
> >         id,
> >         name: "TableRocker".into(),
> >     }));
> >
> > # pollster::block_on(async {
> > let widget = repo.fetch_by_id::<Widget>(42).await.unwrap();
> > assert_eq!(widget.id, 42);
> > assert_eq!(widget.name, "TableRocker");
> > # });
> >
> > // I'd buy that for a dollar! 💵
> > ```
