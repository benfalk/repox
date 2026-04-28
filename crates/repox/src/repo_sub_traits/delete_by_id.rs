use crate::{Entity, Repo};

/// Trait contract for the Repo [`delete_by_id`] method
///
/// [`delete_by_id`]: Repo::delete_by_id
/// ---
pub trait DeleteById<T: Entity>: Repo {
    fn exec(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<DeleteStatus, ::anyhow::Error>> + Send;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeleteStatus {
    NotFound,
    Deleted,
}
