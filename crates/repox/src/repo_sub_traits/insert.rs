use crate::{Entity, Repo};

/// Trait contract for the Repo [`insert`] method
///
/// [`insert`]: Repo::insert
/// ---
pub trait Insert<T: Entity>: Repo {
    fn exec(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<(), InsertError<T>>> + Send;
}

#[derive(Debug, ::thiserror::Error)]
pub enum InsertError<E: Entity> {
    #[error("Entity with ID {0:?} taken")]
    IdTaken(E::ID),
    #[error(transparent)]
    Unknown(#[from] ::anyhow::Error),
}
