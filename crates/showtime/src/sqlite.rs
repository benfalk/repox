use crate::schema::*;
use ::anyhow::Context as _;
use ::sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

pub struct Sqlite {
    pub(crate) pool: ::sqlx::SqlitePool,
}

impl Sqlite {
    pub fn memory() -> Self {
        let opts = ":memory:".parse::<SqliteConnectOptions>().unwrap();
        let pool = SqlitePoolOptions::new().connect_lazy_with(opts);
        Self { pool }
    }

    pub async fn migrate(&self) -> Result<(), ::anyhow::Error> {
        ::sqlx::migrate!()
            .run(&self.pool)
            .await
            .context("failed to run migrations")?;
        Ok(())
    }

    pub fn new(pool: ::sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

impl repox::Repo for Sqlite {}
impl crate::repo::Repo for Sqlite {}

impl repox::CreateWith<actor::Actor, actor::ActorParams> for Sqlite {
    async fn exec(
        &self,
        payload: actor::ActorParams,
    ) -> Result<actor::Actor, ::anyhow::Error> {
        let result = sqlx::query(
            "INSERT INTO actors \
                (first_name, last_name) VALUES (?, ?)",
        )
        .bind(payload.first_name.as_str())
        .bind(payload.last_name.as_str())
        .execute(&self.pool)
        .await?;

        let last_id = result.last_insert_rowid();
        let id = actor::ActorID::try_from(last_id)
            .context("failed to convert last inserted id to ActorID")?;

        Ok(actor::Actor {
            id,
            first_name: payload.first_name,
            last_name: payload.last_name,
        })
    }
}

impl repox::FetchWithChildrenById<actor::Actor, role::Role> for Sqlite {
    async fn exec(
        &self,
        id: actor::ActorID,
    ) -> Result<
        (actor::Actor, Vec<role::Role>),
        repox::FetchWithChildrenError<actor::Actor>,
    > {
        let maybe_actor_row: Option<row::Actor> = sqlx::query_as(
            "SELECT id, first_name, last_name FROM actors WHERE id = ?",
        )
        .bind(id.as_number())
        .fetch_optional(&self.pool)
        .await
        .with_context(|| format!("failed to fetch actor with id {:?}", id))?;

        let Some(actor_row) = maybe_actor_row else {
            return Err(repox::FetchWithChildrenError::NotFound(id));
        };

        let role_rows: Vec<row::Role> = sqlx::query_as(
            "SELECT character_id, actor_id, show_id FROM roles WHERE actor_id = ?",
        )
        .bind(id.as_number())
        .fetch_all(&self.pool)
        .await
        .with_context(|| {
            format!("failed to fetch roles for actor with id {:?}", id)
        })?;

        let actor = actor_row
            .try_into()
            .context("failed to convert actor row to Actor")?;

        let roles = role_rows
            .into_iter()
            .map(|r| {
                role::Role::try_from(r).context("failed to convert role row to Role")
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((actor, roles))
    }
}

impl repox::CreateWith<show::Show, show::ShowParams> for Sqlite {
    async fn exec(
        &self,
        payload: show::ShowParams,
    ) -> Result<show::Show, ::anyhow::Error> {
        let result = sqlx::query(
            "INSERT INTO shows \
                (title, year_released, year_ended) VALUES (?, ?, ?)
            ",
        )
        .bind(payload.title.as_str())
        .bind(payload.year_released.as_number())
        .bind(payload.year_ended.map(|y| y.as_number()))
        .execute(&self.pool)
        .await?;

        let last_id = result.last_insert_rowid();
        let id = show::ShowID::try_from(last_id)
            .context("failed to convert last inserted id to ShowID")?;

        Ok(show::Show {
            id,
            title: payload.title,
            year_released: payload.year_released,
            year_ended: payload.year_ended,
        })
    }
}

impl repox::FetchWithChildrenById<show::Show, role::Role> for Sqlite {
    async fn exec(
        &self,
        id: show::ShowID,
    ) -> Result<
        (show::Show, Vec<role::Role>),
        repox::FetchWithChildrenError<show::Show>,
    > {
        let maybe_show_row: Option<row::Show> = sqlx::query_as(
            "SELECT id, title, year_released, year_ended FROM shows WHERE id = ?",
        )
        .bind(id.as_number())
        .fetch_optional(&self.pool)
        .await
        .with_context(|| format!("failed to fetch show with id {:?}", id))?;

        let Some(show_row) = maybe_show_row else {
            return Err(repox::FetchWithChildrenError::NotFound(id));
        };

        let role_rows: Vec<row::Role> = sqlx::query_as(
            "SELECT character_id, actor_id, show_id FROM roles WHERE show_id = ?",
        )
        .bind(id.as_number())
        .fetch_all(&self.pool)
        .await
        .with_context(|| {
            format!("failed to fetch roles for show with id {:?}", id)
        })?;

        let show = show_row
            .try_into()
            .context("failed to convert show row to Show")?;

        let roles = role_rows
            .into_iter()
            .map(|r| {
                role::Role::try_from(r).context("failed to convert role row to Role")
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok((show, roles))
    }
}

impl repox::CreateWith<character::Character, character::CharacterParams> for Sqlite {
    async fn exec(
        &self,
        payload: character::CharacterParams,
    ) -> Result<character::Character, ::anyhow::Error> {
        let result = sqlx::query(
            "INSERT INTO characters \
                (show_id, title) VALUES (?, ?)",
        )
        .bind(payload.show_id.as_number())
        .bind(payload.title.as_str())
        .execute(&self.pool)
        .await
        .with_context(|| {
            format!(
                "failed to insert character for show with id {}",
                payload.show_id.as_number()
            )
        })?;

        let last_id = result.last_insert_rowid();
        let id = character::CharacterID::try_from(last_id)
            .context("failed to convert last inserted id to CharacterID")?;

        Ok(character::Character {
            id,
            show_id: payload.show_id,
            title: payload.title,
        })
    }
}

impl repox::FetchWithParentById<character::Character, show::Show> for Sqlite {
    async fn exec(
        &self,
        id: character::CharacterID,
    ) -> Result<
        (character::Character, show::Show),
        repox::FetchWithParentError<character::Character>,
    > {
        let maybe_character_row: Option<row::Character> =
            sqlx::query_as("SELECT id, show_id, title FROM characters WHERE id = ?")
                .bind(id.as_number())
                .fetch_optional(&self.pool)
                .await
                .with_context(|| {
                    format!("failed to fetch character with id {:?}", id)
                })?;

        let Some(character_row) = maybe_character_row else {
            return Err(repox::FetchWithParentError::NotFound(id));
        };

        let maybe_show_row: Option<row::Show> = sqlx::query_as(
            "SELECT id, title, year_released, year_ended FROM shows WHERE id = ?",
        )
        .bind(character_row.show_id)
        .fetch_optional(&self.pool)
        .await
        .with_context(|| {
            format!(
                "failed to fetch show with id {:?} for character with id {:?}",
                character_row.show_id, id
            )
        })?;

        let Some(show_row) = maybe_show_row else {
            return Err(repox::FetchWithParentError::NotFound(id));
        };

        Ok((character_row.try_into()?, show_row.try_into()?))
    }
}

impl repox::CreateWith<role::Role, role::RoleParams> for Sqlite {
    async fn exec(
        &self,
        payload: role::RoleParams,
    ) -> Result<role::Role, ::anyhow::Error> {
        let (show_id,): (i64,) = sqlx::query_as(
            "INSERT INTO roles \
                (character_id, actor_id, show_id) \
            SELECT \
                ? as character_id, \
                ? as actor_id, \
                show_id
            FROM characters WHERE id = ?
            RETURNING show_id",
        )
        .bind(payload.character_id.as_number())
        .bind(payload.actor_id.as_number())
        .bind(payload.character_id.as_number())
        .fetch_one(&self.pool)
        .await
        .context("failed to insert role")?;

        let show_id = show::ShowID::try_from(show_id)
            .context("failed to convert show_id to ShowID")?;

        Ok(role::Role {
            character_id: payload.character_id,
            actor_id: payload.actor_id,
            show_id,
        })
    }
}

impl repox::DeleteById<role::Role> for Sqlite {
    async fn exec(
        &self,
        id: role::RoleID,
    ) -> Result<repox::DeleteStatus, ::anyhow::Error> {
        let result =
            sqlx::query("DELETE FROM roles WHERE character_id = ? AND actor_id = ?")
                .bind(id.character_id.as_number())
                .bind(id.actor_id.as_number())
                .execute(&self.pool)
                .await
                .context("failed to delete role")?;

        Ok(if result.rows_affected() == 0 {
            repox::DeleteStatus::NotFound
        } else {
            repox::DeleteStatus::Deleted
        })
    }
}

mod row {
    use crate::schema::*;

    #[derive(Debug, sqlx::FromRow)]
    pub struct Actor {
        pub id: i64,
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Debug, sqlx::FromRow)]
    pub struct Show {
        pub id: i64,
        pub title: String,
        pub year_released: i64,
        pub year_ended: Option<i64>,
    }

    #[derive(Debug, sqlx::FromRow)]
    pub struct Character {
        pub id: i64,
        pub show_id: i64,
        pub title: String,
    }

    #[derive(Debug, sqlx::FromRow)]
    pub struct Role {
        pub character_id: i64,
        pub actor_id: i64,
        pub show_id: i64,
    }

    // - Conversion impls

    impl TryFrom<Actor> for actor::Actor {
        type Error = ::anyhow::Error;

        fn try_from(value: Actor) -> Result<Self, Self::Error> {
            Ok(Self {
                id: actor::ActorID::try_from(value.id)?,
                first_name: value.first_name.into(),
                last_name: value.last_name.into(),
            })
        }
    }

    impl TryFrom<Show> for show::Show {
        type Error = ::anyhow::Error;

        fn try_from(value: Show) -> Result<Self, Self::Error> {
            Ok(Self {
                id: show::ShowID::try_from(value.id)?,
                title: value.title.into(),
                year_released: show::Year::try_from(value.year_released)?,
                year_ended: value
                    .year_ended
                    .map(show::Year::try_from)
                    .transpose()?,
            })
        }
    }

    impl TryFrom<Character> for character::Character {
        type Error = ::anyhow::Error;

        fn try_from(value: Character) -> Result<Self, Self::Error> {
            Ok(Self {
                id: character::CharacterID::try_from(value.id)?,
                show_id: show::ShowID::try_from(value.show_id)?,
                title: value.title.into(),
            })
        }
    }

    impl TryFrom<Role> for role::Role {
        type Error = ::anyhow::Error;

        fn try_from(value: Role) -> Result<Self, Self::Error> {
            Ok(Self {
                character_id: character::CharacterID::try_from(value.character_id)?,
                actor_id: actor::ActorID::try_from(value.actor_id)?,
                show_id: show::ShowID::try_from(value.show_id)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use repox::*;

    async fn sqlite() -> Sqlite {
        let sqlite = Sqlite::memory();
        sqlite.migrate().await.unwrap();
        sqlite
    }

    #[tokio::test]
    async fn create_actor() {
        let sqlite = sqlite().await;
        let create_ted = actor::params()
            .first_name("Ted")
            .last_name("Danson")
            .build();

        let ted = sqlite.create_with(create_ted).await.unwrap();

        assert_eq!(ted.first_name, "Ted".into());
        assert_eq!(ted.last_name, "Danson".into());
        assert_eq!(ted.id, actor::ActorID::try_from(1).unwrap());
    }

    #[tokio::test]
    async fn create_show() {
        let sqlite = sqlite().await;
        let nineteen_eighty_two = show::Year::try_from(1982).unwrap();
        let nineteen_ninety_three = show::Year::try_from(1993).unwrap();
        let create_cheers = show::params()
            .title("Cheers")
            .year_released(nineteen_eighty_two)
            .year_ended(nineteen_ninety_three)
            .build();

        let cheers = sqlite.create_with(create_cheers).await.unwrap();

        assert_eq!(cheers.id, show::ShowID::try_from(1).unwrap());
        assert_eq!(cheers.title, "Cheers".into());
        assert_eq!(cheers.year_released, nineteen_eighty_two);
        assert_eq!(cheers.year_ended, Some(nineteen_ninety_three));
    }

    #[tokio::test]
    async fn create_character() {
        let sqlite = sqlite().await;
        let nineteen_eighty_two = show::Year::try_from(1982).unwrap();
        let create_cheers = show::params()
            .title("Cheers")
            .year_released(nineteen_eighty_two)
            .build();

        let cheers = sqlite.create_with(create_cheers).await.unwrap();
        let create_sam = character::params()
            .show_id(cheers.id)
            .title("Sam Malone")
            .build();

        let sam = sqlite.create_with(create_sam).await.unwrap();
        assert_eq!(sam.id, character::CharacterID::try_from(1).unwrap());
        assert_eq!(sam.show_id, cheers.id);
        assert_eq!(sam.title, "Sam Malone".into());
    }

    #[tokio::test]
    async fn create_role() {
        let sqlite = sqlite().await;
        let nineteen_eighty_two = show::Year::try_from(1982).unwrap();
        let create_cheers = show::params()
            .title("Cheers")
            .year_released(nineteen_eighty_two)
            .build();

        let cheers = sqlite.create_with(create_cheers).await.unwrap();

        let create_sam = character::params()
            .show_id(cheers.id)
            .title("Sam Malone")
            .build();

        let sam = sqlite.create_with(create_sam).await.unwrap();

        let create_ted = actor::params()
            .first_name("Ted")
            .last_name("Danson")
            .build();

        let ted = sqlite.create_with(create_ted).await.unwrap();

        let create_role =
            role::params().actor_id(ted.id).character_id(sam.id).build();

        let role = sqlite.create_with(create_role).await.unwrap();

        assert_eq!(role.actor_id, ted.id);
        assert_eq!(role.character_id, sam.id);
        assert_eq!(role.show_id, cheers.id);
    }
}
