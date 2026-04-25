pub mod schema;
pub mod sqlite;
pub mod types;

mod repo {
    use super::schema::*;

    #[cfg_attr(test, ::repox::mockall)]
    pub trait Repo:
        repox::Repo

        // the actors guild
        + repox::CreateWith<actor::Actor, actor::ActorParams>
        + repox::FetchWithChildrenById<actor::Actor, role::Role>

        // the show must go on
        + repox::CreateWith<show::Show, show::ShowParams>
        + repox::FetchWithChildrenById<show::Show, role::Role>

        // he's quite the character
        + repox::CreateWith<character::Character, character::CharacterParams>
        + repox::FetchWithParentById<character::Character, show::Show>

        // there's a new role in town
        + repox::CreateWith<role::Role, role::RoleParams>
        + repox::DeleteById<role::Role>
    {
    }
}

async fn do_stuff<T: repo::Repo>(repo: &T) -> ::anyhow::Result<()> {
    use crate::schema::*;
    use ::anyhow::Context as _;
    use ::repox::Entity as _;
    use ::repox::Identity as _;

    println!("Creating show and actor...");
    let (cheers, ted) = ::tokio::try_join! {
        repo.create_with(show::params()
            .title("Cheers")
            .year_released(show::Year::try_from(1982)?)
            .year_ended(show::Year::try_from(1993)?)
            .build()
        ),
        repo.create_with(actor::params()
            .first_name("Ted")
            .last_name("Danson")
            .build()
        ),
    }?;

    println!("Creating character...");
    let sam = repo
        .create_with(
            character::params()
                .title("Sam Malone")
                .show_id(cheers.id)
                .build(),
        )
        .await?;

    println!("Creating role...");
    let role = repo
        .create_with(role::params().actor_id(ted.id).character_id(sam.id).build())
        .await?;

    println!("On the correct show!");
    assert_eq!(role.show_id, cheers.id);

    println!("Fetching actor with roles...");
    let (actor, roles) = repo
        .fetch_with_children_by_id::<actor::Actor, role::Role>(ted.id)
        .await
        .context("fetching actor with roles")?;
    assert_eq!(actor, ted);
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0], role);
    assert!(ted.is_owner_of(&roles[0]));
    assert!(cheers.is_owner_of(&roles[0]));
    assert!(sam.is_owner_of(&roles[0]));

    println!("Deleting role...");
    repo.delete_by_id(role.id()).await?;

    println!("Fetching actor with roles again...");
    let (actor, roles) = repo
        .fetch_with_children_by_id::<actor::Actor, role::Role>(ted.id)
        .await
        .context("fetching actor with roles")?;
    assert_eq!(actor, ted);
    assert!(roles.is_empty());

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sqlite = sqlite::Sqlite::memory();
    println!("Migrating database...");
    sqlite.migrate().await?;

    println!("Doing the stuff...");
    do_stuff(&sqlite).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use repox::mock::{ok_val, ok_with};

    use schema::*;
    #[tokio::test]
    async fn test_mock_shows() -> anyhow::Result<()> {
        let mut repo = repo::MockRepo::new();

        repo.expect_create_with()
            .returning(ok_with(|params: show::ShowParams| show::Show {
                id: show::ShowID::try_from(1).unwrap(),
                title: params.title,
                year_released: params.year_released,
                year_ended: params.year_ended,
            }));

        repo.expect_create_with().returning(ok_with(
            |params: actor::ActorParams| actor::Actor {
                id: actor::ActorID::try_from(1).unwrap(),
                first_name: params.first_name,
                last_name: params.last_name,
            },
        ));

        repo.expect_create_with().returning(ok_with(
            |params: character::CharacterParams| character::Character {
                id: character::CharacterID::try_from(1).unwrap(),
                title: params.title,
                show_id: params.show_id,
            },
        ));

        repo.expect_create_with()
            .returning(ok_with(|params: role::RoleParams| role::Role {
                actor_id: params.actor_id,
                character_id: params.character_id,
                show_id: show::ShowID::try_from(1).unwrap(),
            }));

        repo.expect_fetch_with_children_by_id()
            .once()
            .returning(ok_val((
                actor::Actor {
                    id: actor::ActorID::try_from(1).unwrap(),
                    first_name: "Ted".into(),
                    last_name: "Danson".into(),
                },
                vec![role::Role {
                    actor_id: actor::ActorID::try_from(1).unwrap(),
                    character_id: character::CharacterID::try_from(1).unwrap(),
                    show_id: show::ShowID::try_from(1).unwrap(),
                }],
            )));

        repo.expect_delete_by_id::<role::Role>()
            .returning(ok_val(repox::DeleteStatus::Deleted));

        repo.expect_fetch_with_children_by_id()
            .once()
            .returning(ok_val((
                actor::Actor {
                    id: actor::ActorID::try_from(1).unwrap(),
                    first_name: "Ted".into(),
                    last_name: "Danson".into(),
                },
                vec![],
            )));

        do_stuff(&repo).await?;
        Ok(())
    }
}
