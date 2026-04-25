//!
//! Mockall support for repox.
//!

pub use stub_repo::StubRepo;
mod stub_repo {
    use crate::*;

    /// # Stub Repopository
    ///
    /// This is just a marker trait for the main [Repo] trait.
    ///
    /// It allows us to generate a mock repository with Mockall that avoids
    /// the need to implement all of the individual traits for each method,
    /// and instead just rely on these blankets to catch all of the needs
    /// of the main [Repo] trait.
    ///
    pub trait StubRepo: Repo {}

    impl<T, E> Insert<E> for T
    where
        T: StubRepo,
        E: Entity + Send,
    {
        async fn exec(&self, _entity: E) -> Result<(), InsertError<E>> {
            unreachable!("Called `insert` without a mock prepared!")
        }
    }

    impl<T, E> FetchById<E> for T
    where
        T: StubRepo,
        E: Entity,
    {
        async fn exec(&self, _id: <E as Identity>::ID) -> Result<E, FetchError<E>> {
            unreachable!("Called `fetch_by_id` without a mock prepared!")
        }
    }

    impl<T, E> UpdateById<E> for T
    where
        T: StubRepo,
        E: Entity + Send,
    {
        async fn exec(&self, _entity: E) -> Result<(), UpdateError<E>> {
            unreachable!("Called `update_by_id` without a mock prepared!")
        }
    }

    impl<T, E> DeleteById<E> for T
    where
        T: StubRepo,
        E: Entity + Send,
    {
        async fn exec(
            &self,
            _id: <E as Identity>::ID,
        ) -> Result<DeleteStatus, ::anyhow::Error> {
            unreachable!("Called `delete_by_id` without a mock prepared!")
        }
    }

    impl<T, E, P> CreateWith<E, P> for T
    where
        T: StubRepo,
        E: Entity + Send,
        P: Creatable<E>,
    {
        async fn exec(&self, _params: P) -> Result<E, ::anyhow::Error> {
            unreachable!("Called `create_with` without a mock prepared!")
        }
    }

    impl<T, E, O> FetchWithParentById<E, O> for T
    where
        T: StubRepo,
        E: BelongsToForeignKey<O> + Send,
        O: Entity,
    {
        async fn exec(
            &self,
            _id: <E as Identity>::ID,
        ) -> Result<(E, O), FetchWithParentError<E>> {
            unreachable!("Called `fetch_with_parent_by_id` without a mock prepared!")
        }
    }

    impl<T, E, O> FetchWithChildrenById<E, O> for T
    where
        T: StubRepo,
        E: HasManyForeignKey<O> + Send,
        O: Entity,
    {
        async fn exec(
            &self,
            _id: <E as Identity>::ID,
        ) -> Result<(E, Vec<O>), FetchWithChildrenError<E>> {
            unreachable!(
                "Called `fetch_with_children_by_id` without a mock prepared!"
            )
        }
    }
}

pub use helper_fns::ok_val;
pub use helper_fns::ok_with;
mod helper_fns {
    use std::pin::Pin;

    pub fn ok_val<T, P, E>(
        value: T,
    ) -> impl FnMut(P) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>
    where
        T: 'static + Send + Clone,
        E: 'static + Send,
    {
        move |_| {
            let value = value.clone();
            Box::pin(std::future::ready(Ok(value)))
        }
    }

    pub fn ok_with<T, P, E, F>(
        mut f: F,
    ) -> impl FnMut(P) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>
    where
        T: 'static + Send,
        E: 'static + Send,
        F: FnMut(P) -> T + 'static,
    {
        move |params| {
            let value = f(params);
            Box::pin(std::future::ready(Ok(value)))
        }
    }
}
