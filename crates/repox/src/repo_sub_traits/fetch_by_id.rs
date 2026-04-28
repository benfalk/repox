use crate::{Entity, Repo};

/// Trait contract for the Repo [`fetch_by_id`] method
///
/// [`fetch_by_id`]: Repo::fetch_by_id
/// ---
pub trait FetchById<T: Entity>: Repo {
    fn exec(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<T, FetchError<T>>> + Send;
}

#[derive(Debug, ::thiserror::Error)]
pub enum FetchError<E: Entity> {
    #[error("Entity with ID {0:?} not found")]
    NotFound(E::ID),
    #[error(transparent)]
    Unknown(#[from] ::anyhow::Error),
}
