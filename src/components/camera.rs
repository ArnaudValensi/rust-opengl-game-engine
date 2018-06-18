use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Transform;

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
