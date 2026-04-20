# **`fetch_by_id_optional`**

> 📍 **Method**
>
> > ```text
> > fetch_by_id_optional(id: ID) -> Result<Option<Entity>, anyhow::Error>
> > ```
> >
> > Available when your repository implements the `FetchById<T>` trait for
> > the entity type `T` you want to fetch. This method allows you to attempt
> > to fetch an entity by its unique identifier, returning `Ok(Some(entity))`
> > if found, `Ok(None)` if not found, and `Err(error)` if there was an
> > error during the fetch operation.
> >
> > This is particularly useful when you want to handle the case of a
> > missing entity gracefully without treating it as an error condition.
>
> 🧩 *Detailed Example:*
>
> > A harrowing message just came in.  It turns out one of our vendors
> > has no idea which widgets were discontinued and which ones are still
> > in production.  For now they need to be able to **optionally** fetch
> > widgets by ID.  Thankfully all we need to do is implement the simple
> > `FetchById<T>` trait for our widget repo and we can get this for free.
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
> > // fetch this fist full of dollars by ID, but optionally!
> > let ffod = repo.fetch_by_id_optional(42).await.unwrap();
> > assert_eq!(ffod, Some(widget));
> >
> > // Now we need to make sure it returns None when the widget isn't found
> > let data = repo.fetch_by_id_optional(100).await.unwrap();
> > assert_eq!(data, None);
> >
> > // Legendary! 🪩
> > # });
> > ```
