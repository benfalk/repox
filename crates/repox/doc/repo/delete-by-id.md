# **`delete_by_id`**

> 📍 **Method**
>
> > ```text
> > delete_by_id(id: ID) -> Result<DeleteStatus, anyhow::Error>
> > ```
> >
> > Allows you to delete an entity from the repository using its unique
> > identifier. This method is useful when you want to remove a specific
> > entity without needing to retrieve it first. This requires that your
> > repository implements [`DeleteById<T>`] for the entity type `T` you
> > want to delete.  The [`DeleteStatus`] variant returned by this method
> > indicates whether the deletion actually removed an entity or if no
> > entity was found with the provided ID.
>
> 🧩 *Detailed Example:*
>
> > ```rust
> > use repox::DeleteStatus;
> > use repox::Repo;
> >
> > // define an entity
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > // simple repository that can only delete widgets by ID
> > pub trait WidgetDeletor: repox::Repo + repox::DeleteById<Widget> {}
> >
> > // simple new-type wrapper around DashMap to implement WidgetDeletor
> > #[derive(Debug, Clone, Default)]
> > pub struct WidgetRepo {
> >     pub data: dashmap::DashMap<u32, Widget>,
> > }
> >
> > // tag WidgetRepo as a `repox::Repo`
> > impl repox::Repo for WidgetRepo {}
> >
> > // now it can be tagged as a WidgetDeletor
> > impl WidgetDeletor for WidgetRepo {}
> >
> > // produces a compile time error stating that the following
> > // trait is not satisfied; so we here it is:
> > impl repox::DeleteById<Widget> for WidgetRepo {
> >     async fn exec(&self, id: u32) -> Result<DeleteStatus, anyhow::Error>
> >     {
> >         let widget = self.data.remove(&id);
> >         return Ok(if widget.is_some() {
> >             DeleteStatus::Deleted
> >         } else {
> >             DeleteStatus::NotFound
> >         })
> >     }
> > }
> >
> > // now we revel in the glory of being able to delete widgets by ID
> > let widget = Widget {
> >     id: 42,
> >     name: "SteamBlaster".into(),
> > };
> >
> > // start up our new awesome repo and load in our widget
> > let repo = WidgetRepo::default();
> > repo.data.insert(widget.id, widget.clone());
> >
> > // now let the world see the power of delete_by_id in action
> > # pollster::block_on(async {
> > let first_delete = repo.delete_by_id(widget.id).await.unwrap();
> > assert_eq!(first_delete, DeleteStatus::Deleted);
> > 
> > let second_delete = repo.delete_by_id(widget.id).await.unwrap();
> > assert_eq!(second_delete, DeleteStatus::NotFound);
> > # });
> >
> > // let's snoop into the data and make sure it was removed
> > assert!(repo.data.get(&widget.id).is_none());
> >
> > // Wizard! 🧙‍♂️
> > ```
