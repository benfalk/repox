pub mod actor {
    use super::*;

    crate::types::num::non_zero!(ActorID, u32);
    crate::types::str::read_only!(FirstName, Box<str>);
    crate::types::str::read_only!(LastName, Box<str>);

    #[derive(Debug, Clone, PartialEq, repox::Entity)]
    #[has_many(role::Role.actor_id)]
    #[create_params(ActorParams)]
    pub struct Actor {
        pub id: ActorID,
        pub first_name: FirstName,
        pub last_name: LastName,
    }

    #[bon::builder(finish_fn = "build")]
    pub fn params(
        first_name: impl Into<FirstName>,
        last_name: impl Into<LastName>,
    ) -> ActorParams {
        ActorParams {
            first_name: first_name.into(),
            last_name: last_name.into(),
        }
    }
}

pub mod character {
    use super::*;

    crate::types::num::non_zero!(CharacterID, u32);
    crate::types::str::read_only!(Title, Box<str>);

    #[derive(Debug, PartialEq, Clone, repox::Entity)]
    #[belongs_to(show::Show, show_id)]
    #[has_many(role::Role.character_id)]
    #[create_params(CharacterParams)]
    pub struct Character {
        pub id: CharacterID,
        pub show_id: show::ShowID,
        pub title: Title,
    }

    #[bon::builder(finish_fn = "build")]
    pub fn params(
        show_id: show::ShowID,
        title: impl Into<Title>,
    ) -> CharacterParams {
        CharacterParams {
            show_id,
            title: title.into(),
        }
    }
}

pub mod role {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct RoleID {
        pub character_id: character::CharacterID,
        pub actor_id: actor::ActorID,
    }

    #[derive(Debug, Clone, PartialEq, repox::Entity)]
    #[belongs_to(show::Show, show_id)]
    #[belongs_to(character::Character, character_id)]
    #[belongs_to(actor::Actor, actor_id)]
    #[create_params(RoleParams, excluding(show_id))]
    #[custom_id(RoleID, RoleID::from)]
    pub struct Role {
        pub character_id: character::CharacterID,
        pub actor_id: actor::ActorID,
        pub show_id: show::ShowID,
    }

    impl From<&Role> for RoleID {
        fn from(role: &Role) -> Self {
            RoleID {
                character_id: role.character_id,
                actor_id: role.actor_id,
            }
        }
    }

    #[bon::builder(finish_fn = "build")]
    pub fn params(
        character_id: character::CharacterID,
        actor_id: actor::ActorID,
    ) -> RoleParams {
        RoleParams {
            character_id,
            actor_id,
        }
    }
}

pub mod show {
    use super::*;

    crate::types::num::non_zero!(ShowID, u32);
    crate::types::num::non_zero!(Year, u16);
    crate::types::str::read_only!(Title, Box<str>);

    #[derive(Debug, PartialEq, Clone, repox::Entity)]
    #[has_many(role::Role.show_id)]
    #[has_many(character::Character.show_id)]
    #[create_params(ShowParams)]
    pub struct Show {
        pub id: ShowID,
        pub title: Title,
        pub year_released: Year,
        pub year_ended: Option<Year>,
    }

    #[bon::builder(finish_fn = "build")]
    pub fn params(
        title: impl Into<Title>,
        year_released: Year,
        year_ended: Option<Year>,
    ) -> ShowParams {
        ShowParams {
            title: title.into(),
            year_released,
            year_ended,
        }
    }
}
