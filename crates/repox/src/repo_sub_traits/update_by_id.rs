use crate::{Entity, Repo};

/// Trait contract for the Repo [`update_by_id`] method
///
/// [`update_by_id`]: Repo::update_by_id
/// ---
pub trait UpdateById<T: Entity>: Repo {
    fn exec(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<(), UpdateError<T>>> + Send;
}

#[derive(Debug, ::thiserror::Error)]
pub enum UpdateError<E: Entity> {
    #[error("Entity with ID {0:?} not found")]
    NotFound(E::ID),
    #[error(transparent)]
    Unknown(#[from] ::anyhow::Error),
}
