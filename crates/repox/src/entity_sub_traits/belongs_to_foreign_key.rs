use crate::Entity;

pub trait BelongsToForeignKey<T: Entity>: Entity {
    fn key(&self) -> T::ID;
}
