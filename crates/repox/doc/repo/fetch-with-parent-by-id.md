# **`fetch_with_parent_by_id`**

> 📍 **Method**
>
> > ```text
> > fetch_with_parent_by_id(id: ID) -> Result<
> >     (Entity, ParentEntity),
> >     repox::FetchWithParentError<Entity>,
> > >
> > ```
> >
> > If you implement the `FetchWithParentById<ChildEntity, ParentEntity>`
> > trait, you can use this method to fetch a child entity along with its
> > associated parent entity using the unique identifier of the child.
> > The method returns a tuple containing the child entity and its parent
> > entity if found, or an error if the child entity is not found.
> > This is particularly useful when you need to access related data in
> > a single fetch operation, reducing the number of queries needed to
> > retrieve associated entities.
>
> 🧩 _Detailed Example:_
>
> > **Cheese and Crackers!** It turns out that widgets were supposed to
> > each belong to certain assemblies. Nobody at WidgetCo. can remember
> > exactly what these assemblies are, but they know that they we definitely
> > need them. The head of their IT refuses to talk to you, and has simply
> > instructed you to "figure it out". We don't understand this binary
> > format so for now we'll just house it in a `Vec<u8>` and hope for the best.
> >
> > ```rust
> > use repox::Repo;
> >
> > // Our tried and true entity definition, but now with a parent relationship!
> > // The `belongs_to` macro tag is how we acknowledge this relationship in
> > // our code.
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[belongs_to(Assembly, assembly_id)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub assembly_id: u32,
> >     pub name: String,
> > }
> >
> > // The fabled assembly entity.  We know nothing of its kind;
> > // however, we do know that it has a relationship with widgets.
> > // Here we acknowledge this relationship with the `has_many`
> > // macro tag pointing to the `assembly_id` field of the
> > // `Widget` struct.
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[has_many(Widget.assembly_id)]
> > pub struct Assembly {
> >     pub id: u32,
> >     pub data: Vec<u8>,
> > }
> >
> > // the repo needs to know about this relationship too, so we
> > // make a trait that can support them together
> > pub trait WidgetWithAssembly: Repo
> >     + repox::FetchWithParentById<Widget, Assembly>
> > {
> > }
> >
> > // Our new high-end data store that only works with widgets
> > // freshly outfitted with the ability to hold assemblies for
> > // these widgets.
> > #[derive(Debug, Default)]
> > pub struct WidgetData {
> >     pub widgets: dashmap::DashMap<u32, Widget>,
> >     pub assemblies: dashmap::DashMap<u32, Assembly>,
> > }
> >
> > // enough gabbing, let's get this show on the road and implement
> > // our trait on this store
> > impl Repo for WidgetData {}
> > impl WidgetWithAssembly for WidgetData {}
> >
> > // I know Fred said we should switch to a real database, but we
> > // just don't have the infra budget for that right now...
> > impl repox::FetchWithParentById<Widget, Assembly> for WidgetData {
> >     async fn exec(&self, id: u32) -> Result<
> >         (Widget, Assembly),
> >         repox::FetchWithParentError<Widget>,
> >     > {
> >         let Some(widget) = self.widgets.get(&id) else {
> >             return Err(repox::FetchWithParentError::NotFound(id));
> >         };
> >         let Some(assembly) = self.assemblies.get(&widget.assembly_id) else {
> >             return Err(repox::FetchWithParentError::NotFound(id));
> >         };
> >         Ok((widget.clone(), assembly.clone()))
> >     }
> > }
> >
> > // prepare the data, so that it might be used for the tests to come
> > let data = WidgetData::default();
> > let cake = Widget {
> >     id: 1,
> >     assembly_id: 2_605_927_472,
> >     name: "Cake".into(),
> > };
> > let food = Assembly {
> >     id: 2_605_927_472,
> >     data: vec![0xF0, 0x0D],
> > };
> > data.widgets.insert(cake.id, cake.clone());
> > data.assemblies.insert(food.id, food.clone());
> >
> > // and now... the moment of truth!  Fetch a widget
> > // with its parent assembly by ID!
> > # pollster::block_on(async {
> > let (widget, assembly) = data.fetch_with_parent_by_id(1).await.unwrap();
> > assert_eq!(widget, cake);
> > assert_eq!(assembly, food);
> >
> > // That's hot 🔥
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
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[belongs_to(Widget, widget_id)]
> > pub struct Doodad {
> >     pub id: u32,
> >     pub widget_id: u32,
> > }
> >
> > #[repox::mockall]
> > pub trait ParentRepo: Repo + repox::FetchWithParentById<Doodad, Widget> {}
> >
> > let mut repo = MockParentRepo::new();
> > repo.expect_fetch_with_parent_by_id::<Doodad, Widget>()
> >     .withf(|id| *id == 67)
> >     .returning(repox::mock::ok_with(|id: u32| (
> >         Doodad { id, widget_id: 18 },
> >         Widget { id: 18, name: "WindSail".into() },
> >     )));
> >
> >
> > # pollster::block_on(async {
> > let (doodad, widget) =
> >     repo.fetch_with_parent_by_id::<Doodad, Widget>(67).await.unwrap();
> > assert_eq!(doodad, Doodad { id: 67, widget_id: 18 });
> > assert_eq!(widget, Widget { id: 18, name: "WindSail".into() });
> > # });
> >
> > // Simply Fantastic ✨
> > ```
