# **`update_by_id`**

> 📍 **Method**
>
> > ```text
> > update_by_id(entity: Entity) -> Result<(), repox::UpdateError<Entity>>
> > ```
> >
> > This method allows you to update an existing entity in the repository
> > using the exact structure of the entity itself. The data store will
> > use the unique identifier of the entity to find the existing record and
> > update it with the new data provided in the entity. This is available when
> > a repository implements the `UpdateById<T>` trait for the entity type `T`
> > you want to support.
>
> 🧩 _Detailed Example:_
>
> > Old man Ferguson just came back from from the assembly
> > line pissed off that his widgets are getting messed up in
> > transit. He said we'll have a "big problem" if we don't
> > figure out how to update these defective widgets. Hopefully
> > this `update_by_id` method is the solution to his problems.
> >
> > ```rust
> > use repox::Repo;
> >
> > // our trusty ole widget entity, but now we
> > // need to be able to update it by ID
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > // some of these widgets are getting messed up
> > // in transit, we need to be able to update them by ID
> > pub trait WidgetRepo: Repo + repox::UpdateById<Widget> {}
> >
> > // Our new high-end data store that only works with widgets
> > #[derive(Debug, Default)]
> > pub struct WidgetData {
> >     pub data: dashmap::DashMap<u32, Widget>,
> > }
> >
> > // We've got our top AI specialist Claude to help us
> > // out with this one, it's ready to give us edits for
> > // the defective widgets, so let's implement the trait
> > // and make sure it's working for us:
> > impl Repo for WidgetData {}
> > impl WidgetRepo for WidgetData {}
> >
> > // this is the trait we need so we can update by id
> > impl repox::UpdateById<Widget> for WidgetData {
> >     async fn exec(&self, widget: Widget)
> >     -> Result<(), repox::UpdateError<Widget>>
> >     {
> >         if !self.data.contains_key(&widget.id) {
> >             return Err(repox::UpdateError::NotFound(widget.id));
> >         }
> >         self.data.insert(widget.id, widget);
> >         Ok(())
> >     }
> > }
> >
> > // This is for all the marbles, we just implemented
> > // an update by ID method for our widget repo!  You
> > // know what to do, wire up a test to make sure it
> > // works and we can show it off to old man Ferguson!
> > # pollster::block_on(async {
> > let repo = WidgetData::default();
> > let mut widget = Widget {
> >     id: 42,
> >     name: "BeeSauce".into(),
> > };
> > repo.data.insert(widget.id, widget.clone());
> >
> > // Can you imagine the look on the shocked customers faces?
> > // I doubt Bee and Beef sauce are going to be all that similar.
> > widget.name.clear();
> > widget.name.push_str("BeefSauce");
> > repo.update_by_id(widget.clone()).await.unwrap();
> >
> > // And now the big reveal, let's make sure it actually updated
> > let hopefully_updated = repo.data.get(&widget.id).unwrap().clone();
> > assert_eq!(hopefully_updated, widget);
> >
> > // It's alive! 🧟‍♂️
> > # });
> > ```
>
> 🧪 _Mock Example:_
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
> > pub trait Updater: Repo + repox::UpdateById<Widget> {}
> >
> > let mut repo = MockUpdater::new();
> > repo.expect_update_by_id::<Widget>()
> >     .withf(|widget|{
> >         *widget == Widget { id: 3, name: "DooHickey".into() }
> >     })
> >     .returning(repox::mock::ok_val(()));
> >
> > # pollster::block_on(async {
> > repo.update_by_id(Widget { id: 3, name: "DooHickey".into() })
> >     .await
> >     .unwrap();
> > # });
> >
> > // Power Overwhelming 👾
> > ```
