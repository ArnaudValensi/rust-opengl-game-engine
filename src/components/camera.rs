use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Camera;

impl Component for Camera {
    type Storage = VecStorage<Self>;
}
