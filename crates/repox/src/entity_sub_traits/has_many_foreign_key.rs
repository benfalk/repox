use crate::Entity;

pub trait HasManyForeignKey<T: Entity>: Entity {
    fn key(entity: &T) -> Self::ID;
}
