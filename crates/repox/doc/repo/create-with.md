# **`create_with`**

> 📍 **Method Info**
>
> > ```text
> > create_with(params: Params) -> Result<Entity, anyhow::Error>
> > ```
> >
> > Provided to any repository that implements `CreateWith`, this method
> > allows you to create a new entity using a separate parameters `struct`.
> > This is particularly useful when the creation of an entity requires
> > additional information that is not part of the entity itself,
> > such as auto-generated IDs or timestamps.
> >
> > **TODO:** briefly explain the quick ways to tag and define these
> > parameter structs with the derive macro attributes.
>
> 🧩 *Detailed Example:*
>
> > I just got off the phone with the manager of our local widget
> > factory and it turns out they need to start storing their
> > widget data immediately!  Let's see how we can use `create_with`
> > to get them up and running.
> >
> > ```rust
> > use repox::Repo;
> > use std::sync::atomic::Ordering;
> >
> > // Define an entity, take note of the `create_params` attribute
> > // as it creates a new struct called `WidgetParams` that we can
> > // use with the `create_with` interface
> > #[derive(Debug, Clone, PartialEq, repox::Entity)]
> > #[create_params(WidgetParams)]
> > pub struct Widget {
> >     pub id: u32,
> >     pub name: String,
> > }
> >
> > // an interface to start building these puppies
> > pub trait WidgetCreator: Repo + repox::CreateWith<Widget, WidgetParams> {}
> >
> > // simple structure to implement WidgetCreator and start making
> > // bank on this sweet deal.
> > #[derive(Debug, Default)]
> > pub struct WidgetRepo {
> >     pub data: dashmap::DashMap<u32, Widget>,
> >     pub next_num: std::sync::atomic::AtomicU32,
> > }
> >
> > // lets make bank on this sweet deal by implementing our trait on
> > // on this sick new repo trait Claude code made for us:
> > impl Repo for WidgetRepo {}
> > impl WidgetCreator for WidgetRepo {}
> >
> > // this is where we pull in that sweet money with this ID tracking
> > impl repox::CreateWith<Widget, WidgetParams> for WidgetRepo {
> >     async fn exec(&self, params: WidgetParams) -> anyhow::Result<Widget> {
> >         let widget = Widget {
> >             id: self.next_num.fetch_add(1, Ordering::SeqCst),
> >             name: params.name,
> >         };
> >         self.data.insert(widget.id, widget.clone());
> >         Ok(widget)
> >     }
> > }
> >
> > // we better test this... just to make sure we're guchi
> > let repo = WidgetRepo::default();
> > let params = WidgetParams { name: "RamRod".into() };
> >
> > // pump one and make sure it made it in
> > # pollster::block_on(async {
> > let ram_rod = repo.create_with(params.clone()).await.unwrap();
> > assert_eq!(ram_rod.name, "RamRod");
> > assert_eq!(ram_rod.id, 0);
> >
> > // better pump in another and make sure the ids are working right
> > let another_rod = repo.create_with(params.clone()).await.unwrap();
> > assert_eq!(another_rod.name, "RamRod");
> > assert_eq!(another_rod.id, 1);
> >
> > // nice!, but did the data make it in?
> > let first_rod = repo.data.get(&0).unwrap();
> > let second_rod = repo.data.get(&1).unwrap();
> > assert_eq!(*first_rod, ram_rod);
> > assert_eq!(*second_rod, another_rod);
> >
> > // Sick! 🤘
> > # });
> > ```
