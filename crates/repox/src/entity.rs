use super::*;
use std::cmp::Ord;
use std::hash::Hash;

/// Data that can be used to [`identify`] an entity.
///
/// [`identify`]: Identity::ID
/// ---
pub trait Identifier: Copy + Eq + Hash + Ord + Send + Sync + 'static {}
impl<T: Copy + Eq + Hash + Ord + Send + Sync + 'static> Identifier for T {}

/// Trait contract for the identity of an [`entity`], which is used
/// to uniquely identify it across repositories and operations.
///
/// [`entity`]: Entity
/// ---
pub trait Identity {
    type ID: Identifier;
    fn id(&self) -> Self::ID;
}

/// Anything that can be uniquely identified by ID
///
/// Entities are also clonable with a static lifetime. This allows
/// them to be used across threads and repositories.
///
/// Depending on some of the additional sub traits it implements,
/// an entity can also be related to other entities.  Often, these
/// relationships are defined with macro helpers.  Each method contains
/// more documentation on the relationships and their traits.
///
/// ---
///
#[doc = include_str!("../doc/entity.md")]
pub trait Entity: Identity + Clone + 'static {
    #[doc = include_str!("../doc/entity/belongs-to-key.md")]
    fn belongs_to_key<T: Entity>(&self) -> T::ID
    where
        Self: BelongsToForeignKey<T>,
    {
        <Self as BelongsToForeignKey<T>>::key(self)
    }

    #[doc = include_str!("../doc/entity/belongs-to.md")]
    fn belongs_to<T: Entity>(&self, entity: &T) -> bool
    where
        Self: BelongsToForeignKey<T>,
    {
        self.belongs_to_key::<T>() == entity.id()
    }

    #[doc = include_str!("../doc/entity/has-many-key.md")]
    fn has_many_key<T: Entity>(&self, entity: &T) -> Self::ID
    where
        Self: HasManyForeignKey<T>,
    {
        <Self as HasManyForeignKey<T>>::key(entity)
    }

    #[doc = include_str!("../doc/entity/is-owner-of.md")]
    fn is_owner_of<T: Entity>(&self, entity: &T) -> bool
    where
        Self: HasManyForeignKey<T>,
    {
        self.has_many_key(entity) == self.id()
    }
}
