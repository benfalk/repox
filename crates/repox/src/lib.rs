#![doc = include_str!("../README.md")]

#[cfg(feature = "mock")]
pub mod mock;

#[doc = include_str!("../doc/entity.md")]
#[doc = include_str!("../doc/entity-derive/belongs-to.md")]
#[doc = include_str!("../doc/entity-derive/create-params.md")]
#[doc = include_str!("../doc/entity-derive/created-by.md")]
#[doc = include_str!("../doc/entity-derive/custom-id.md")]
#[doc = include_str!("../doc/entity-derive/entity.md")]
#[doc = include_str!("../doc/entity-derive/has-many.md")]
#[cfg(feature = "derive")]
pub use ::repox_derive::Entity;

#[cfg(feature = "mock")]
pub use ::repox_derive::mockall;

pub use entity::{Entity, Identifier, Identity};
pub use entity_sub_traits::*;
pub use repo::Repo;
pub use repo_sub_traits::*;

mod entity;
mod entity_sub_traits;
mod repo;
mod repo_sub_traits;
