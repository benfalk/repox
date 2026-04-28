use crate::{Entity, HasManyForeignKey, Repo};

/// Trait contract for the Repo [`fetch_with_children_by_id`] method
///
/// [`fetch_with_children_by_id`]: Repo::fetch_with_children_by_id
/// ---
pub trait FetchWithChildrenById<T: HasManyForeignKey<O>, O: Entity>: Repo {
    fn exec(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<(T, Vec<O>), FetchWithChildrenError<T>>> + Send;
}

#[derive(Debug, ::thiserror::Error)]
pub enum FetchWithChildrenError<T: Entity> {
    #[error("Entity with ID {0:?} not found")]
    NotFound(T::ID),
    #[error(transparent)]
    Unknown(#[from] ::anyhow::Error),
}
