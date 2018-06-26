use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Player;

impl Component for Player {
    type Storage = VecStorage<Self>;
}
