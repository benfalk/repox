#![doc = include_str!("../README.md")]

use std::cmp::Ord;
use std::hash::Hash;

#[doc = include_str!("../doc/entity.md")]
#[doc = include_str!("../doc/entity/belongs-to.md")]
#[doc = include_str!("../doc/entity/create-params.md")]
#[doc = include_str!("../doc/entity/created-by.md")]
#[doc = include_str!("../doc/entity/custom-id.md")]
#[doc = include_str!("../doc/entity/entity.md")]
#[doc = include_str!("../doc/entity/has-many.md")]
pub use ::repox_derive::Entity;

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

pub trait Identifier: Copy + Eq + Hash + Ord + Send + Sync + 'static {}
impl<T: Copy + Eq + Hash + Ord + Send + Sync + 'static> Identifier for T {}

pub trait Identity {
    type ID: Identifier;
    fn id(&self) -> Self::ID;
}

pub trait Entity: Identity + Clone {
    fn belongs_to_key<T: Entity>(&self) -> T::ID
    where
        Self: BelongsToForeignKey<T>,
    {
        <Self as BelongsToForeignKey<T>>::key(self)
    }

    fn belongs_to<T: Entity>(&self, entity: &T) -> bool
    where
        Self: BelongsToForeignKey<T>,
    {
        self.belongs_to_key::<T>() == entity.id()
    }

    fn has_many_key<T: Entity>(&self, entity: &T) -> Self::ID
    where
        Self: HasManyForeignKey<T>,
    {
        <Self as HasManyForeignKey<T>>::key(entity)
    }

    fn is_owner_of<T: Entity>(&self, entity: &T) -> bool
    where
        Self: HasManyForeignKey<T>,
    {
        self.has_many_key(entity) == self.id()
    }
}
impl<T: Identity + Clone> Entity for T {}

// CRUD Traits

// ----- Insert

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

// ----- Fetch

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

// ----- Update

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

// ----- Delete

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

// ----- Create

pub trait Creatable<T: Entity> {}

pub trait CreateWith<T: Entity, P: Creatable<T>>: Repo {
    fn exec(
        &self,
        payload: P,
    ) -> impl Future<Output = Result<T, ::anyhow::Error>> + Send;
}

// Relation Traits

pub trait HasManyForeignKey<T: Entity>: Entity {
    fn key(entity: &T) -> Self::ID;
}

pub trait BelongsToForeignKey<T: Entity>: Entity {
    fn key(&self) -> T::ID;
}

// Bulk Fetchers

// ----- Fetch with Parent

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

// ----- Fetch with Single

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

// ----- By Foreign Key
