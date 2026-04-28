use super::*;

#[doc = include_str!("../doc/repo.md")]
pub trait Repo: Send + Sync + 'static {
    #[doc = include_str!("../doc/repo/delete-by-id.md")]
    fn delete_by_id<T: Entity>(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<DeleteStatus, ::anyhow::Error>> + Send
    where
        Self: DeleteById<T>,
    {
        DeleteById::<T>::exec(self, id)
    }

    #[doc = include_str!("../doc/repo/fetch-by-id.md")]
    fn fetch_by_id<T: Entity>(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<T, FetchError<T>>> + Send
    where
        Self: FetchById<T>,
    {
        FetchById::<T>::exec(self, id)
    }

    #[doc = include_str!("../doc/repo/fetch-by-id-optional.md")]
    fn fetch_by_id_optional<T: Entity>(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<Option<T>, ::anyhow::Error>> + Send
    where
        Self: FetchById<T>,
    {
        async move {
            match FetchById::<T>::exec(self, id).await {
                Ok(entity) => Ok(Some(entity)),
                Err(FetchError::NotFound(_)) => Ok(None),
                Err(FetchError::Unknown(e)) => Err(e),
            }
        }
    }

    #[doc = include_str!("../doc/repo/fetch-with-parent-by-id.md")]
    fn fetch_with_parent_by_id<T: BelongsToForeignKey<O>, O: Entity>(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<(T, O), FetchWithParentError<T>>> + Send
    where
        Self: FetchWithParentById<T, O>,
    {
        FetchWithParentById::<T, O>::exec(self, id)
    }

    #[doc = include_str!("../doc/repo/fetch-with-children-by-id.md")]
    fn fetch_with_children_by_id<T: HasManyForeignKey<O>, O: Entity>(
        &self,
        id: T::ID,
    ) -> impl Future<Output = Result<(T, Vec<O>), FetchWithChildrenError<T>>> + Send
    where
        Self: FetchWithChildrenById<T, O>,
    {
        FetchWithChildrenById::<T, O>::exec(self, id)
    }

    #[doc = include_str!("../doc/repo/create-with.md")]
    fn create_with<T: Entity, P: Creatable<T>>(
        &self,
        payload: P,
    ) -> impl Future<Output = Result<T, ::anyhow::Error>> + Send
    where
        Self: CreateWith<T, P>,
    {
        CreateWith::<T, P>::exec(self, payload)
    }

    #[doc = include_str!("../doc/repo/update-by-id.md")]
    fn update_by_id<T: Entity>(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<(), UpdateError<T>>> + Send
    where
        Self: UpdateById<T>,
    {
        UpdateById::<T>::exec(self, entity)
    }

    #[doc = include_str!("../doc/repo/insert.md")]
    fn insert<T: Entity>(
        &self,
        entity: T,
    ) -> impl Future<Output = Result<(), InsertError<T>>> + Send
    where
        Self: Insert<T>,
    {
        Insert::<T>::exec(self, entity)
    }
}
