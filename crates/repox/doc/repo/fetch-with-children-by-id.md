# **`fetch_with_children_by_id`**

> 📍 **Method**
>
> > ```text
> > fetch_with_children_by_id(id: ID) -> Result<
> >     (Entity, Vec<ChildEntity>),
> >     repox::FetchWithChildrenError<Entity>,
> > >
> > ```
> >
> > This method allows you to fetch an entity along with its associated
> > child entities using the unique identifier of the parent entity.
> > This is available when your repository implements the
> > `FetchWithChildrenById<ParentEntity, ChildEntity>` trait. The method
> > returns a tuple containing the parent entity and a vector of its child
> > entities if found, or an error if the parent entity is not found.
>
> 🧩 *Detailed Example:*
>
> > **Fudge monkeys!**  It turns out that widgets were supposed to be able
> > to contain provisions.  Fred ran off muttering something about becoming
> > a cabbage farmer and leaving the city life behind.  All you can find
> > out is that these provisions are specific float values... but that's all
> > we know right now.  This is it, it's all up to us now.
> >
> > ```rust
> > use repox::Repo;
> >
> > // Define an entity
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[has_many(WidgetProvision.widget_id)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > // Define an entity for now
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[belongs_to(Widget, widget_id)]
> > pub struct WidgetProvision {
> >     pub id: u128,
> >     pub widget_id: u32,
> >     pub float: f64,
> > }
> >
> > // We've made provisions for our widgets, get them out there!
> > pub trait WidgetWithProvisions: Repo
> >     + repox::FetchWithChildrenById<Widget, WidgetProvision>
> > {
> > }
> >
> > // Our new high-end data store that only works with widgets
> > // and could could be done better if we had more time _<murmurs>_
> > #[derive(Debug, Default)]
> > pub struct WidgetData {
> >     // what could possibly go wrong with this? I mean... it's just a
> >     // map of widgets to their provisions, what could go wrong?
> >     pub widgets: dashmap::DashMap<u32, (Widget, Vec<WidgetProvision>)>,
> > }
> >
> > // lets make bank on this sweet deal by implementing our trait on
> > // on this sick new repo trait Claude code made for us:
> > impl Repo for WidgetData {}
> > impl WidgetWithProvisions for WidgetData {}
> >
> > // this is it; the time where claude make's it's magic and implements
> > // the interface for us and we can finally fetch widgets with their
> > // provisions by ID... take that Sqlite!!
> > impl repox::FetchWithChildrenById<Widget, WidgetProvision> for WidgetData {
> >     async fn exec(&self, id: u32) -> Result<
> >         (Widget, Vec<WidgetProvision>),
> >         repox::FetchWithChildrenError<Widget>,
> >     > {
> >         match self.widgets.get(&id) {
> >              Some(data_ref) => Ok(data_ref.clone()),
> >              None => Err(repox::FetchWithChildrenError::NotFound(id)),
> >         }
> >     }
> > }
> >
> > // now I want them to marvel at her speed, go out with a bang!
> > # pollster::block_on(async {
> > let data = WidgetData::default();
> > data.widgets.insert(1, (
> >     Widget { id: 1, name: "Horn".into() },
> >     vec![WidgetProvision { id: 7, widget_id: 1, float: 3.14 }]
> > ));
> >
> > // get on the horn and fetch this widget with its provisions by ID!
> > let (horn, povisions) = data.fetch_with_children_by_id(1).await.unwrap();
> > assert_eq!(horn, Widget { id: 1, name: "Horn".into() });
> > assert_eq!(
> >     povisions,
> >     vec![WidgetProvision { id: 7, widget_id: 1, float: 3.14 }]
> > );
> >
> > // We're the renegades of data fetching, the outlaws of the repo world,
> > // and fetch_with_children_by_id is our trusty steed! 🐎
> > # });
> > ```
>
> 🧪 *Mock Example:*
>
> > ```rust
> > use repox::{Repo, Entity};
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[has_many(Doodad.widget_id)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct Doodad {
> >     pub id: u32,
> >     pub widget_id: u32,
> > }
> >
> > #[repox::mockall]
> > pub trait ChildRepo: Repo + repox::FetchWithChildrenById<Widget, Doodad> {}
> >
> > let mut repo = MockChildRepo::new();
> > repo.expect_fetch_with_children_by_id::<Widget, Doodad>()
> >     .withf(|id| *id == 42)
> >     .returning(repox::mock::ok_with(|id: u32| (
> >         Widget {
> >             id,
> >             name: "Sprocket".into(),
> >         },
> >         vec![Doodad { id: 1, widget_id: id }],
> >     )));
> >
> > # pollster::block_on(async {
> > let (widget, doodads) = repo
> >     .fetch_with_children_by_id::<Widget, Doodad>(42).await.unwrap();
> > assert_eq!(widget, Widget { id: 42, name: "Sprocket".into() });
> > assert_eq!(doodads, vec![Doodad { id: 1, widget_id: 42 }]);
> > # });
> >
> > // Take that to the bank! 🏦
> > ```
