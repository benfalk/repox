use crate::{BelongsToForeignKey, Entity, Repo};

/// Trait contract for the Repo [`fetch_with_parent_by_id`] method
///
/// [`fetch_with_parent_by_id`]: Repo::fetch_with_parent_by_id
/// ---
pub trait FetchWithParentById<T: BelongsToForeignKey<O>, O: Entity>: Repo {
    fn exec(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<(T, O), FetchWithParentError<T>>> + Send;
}

#[derive(Debug, ::thiserror::Error)]
pub enum FetchWithParentError<T: Entity> {
    #[error("Entity with ID {0:?} not found")]
    NotFound(T::ID),
    #[error(transparent)]
    Unknown(#[from] ::anyhow::Error),
}
