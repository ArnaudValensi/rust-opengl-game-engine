use specs::{Component, DenseVecStorage, Entity};

pub struct Parent {
    pub entity: Entity,
}

impl Component for Parent {
    type Storage = DenseVecStorage<Self>;
}
