# **`#custom_id`**

> 🏭 **Macro**
>
> > ```text
> > #[custom_id($id_type, $func_path)]
> > ```
> >
> > At the heart of the entity concept is the `Identifier` trait.  Every
> > entity **must** have an identifier, and the `Identifier` trait defines
> > what type that is and a method of `id(&self)` to retrieve it.  By
> > default the `Entity` macro selects a field named `id`; however, this
> > macro is a convenient shortcut to specify a custom function that
> > can implement the `Identifier` trait for you.  The first argument is
> > the ID type, and the second is a path to a function that takes a
> > reference to the entity and returns the ID.
>
> 📝 _Macro Example:_
>
> > In this example, we have a `UserRole` entity which has a composite
> > key of `UserRoleID` consisting of `user_id` and `role_id`.  We use the
> > defined `From<&UserRole>` implementation to convert a `&UserRole`
> > into a `UserRoleID` and specify that as the custom ID function in
> > the macro.
> >
> > ```rust
> > use repox::{Entity, Identity};
> >
> > #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash, PartialOrd)]
> > pub struct UserRoleID {
> >     pub user_id: u32,
> >     pub role_id: u32,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > #[custom_id(UserRoleID, UserRoleID::from)]
> > pub struct UserRole {
> >     pub user_id: u32,
> >     pub role_id: u32,
> >     pub created_at: u64,
> > }
> >
> > impl From<&UserRole> for UserRoleID {
> >     fn from(user_role: &UserRole) -> Self {
> >         Self { user_id: user_role.user_id, role_id: user_role.role_id }
> >     }
> > }
> >
> > // Quick Demo of the custom ID in action:
> > let user_role = UserRole { user_id: 1, role_id: 2, created_at: 123456789 };
> > assert_eq!(user_role.id(), UserRoleID { user_id: 1, role_id: 2 });
> > ```
>
> 🔬 _Macro Details:_
>
> > Here is the same example, but, without using the macro and implementing
> > it ourselves.
>
> > ```rust
> > use repox::{Entity, Identity};
> >
> > #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, Hash, PartialOrd)]
> > pub struct UserRoleID {
> >     pub user_id: u32,
> >     pub role_id: u32,
> > }
> >
> > #[derive(Debug, Clone, PartialEq, Entity)]
> > pub struct UserRole {
> >     pub user_id: u32,
> >     pub role_id: u32,
> >     pub created_at: u64,
> > }
> >
> > impl From<&UserRole> for UserRoleID {
> >     fn from(user_role: &UserRole) -> Self {
> >         Self { user_id: user_role.user_id, role_id: user_role.role_id }
> >     }
> > }
> >
> > // this is what the `custom_id` macro generates for us
> > impl Identity for UserRole {
> >     type ID = UserRoleID;
> >     fn id(&self) -> Self::ID { UserRoleID::from(self) }
> > }
> >
> > // Quick Demo of the custom ID in action:
> > let user_role = UserRole { user_id: 1, role_id: 2, created_at: 123456789 };
> > assert_eq!(user_role.id(), UserRoleID { user_id: 1, role_id: 2 });
> > ```
