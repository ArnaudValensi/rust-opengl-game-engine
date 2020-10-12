use voxel::direction::Direction::*;
use std::slice::Iter;

#[derive(Copy, Clone, Debug)]
pub enum Direction { North, South, East, West, Up, Down }

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction;  6] = [North, South, East, West, Up, Down];
        DIRECTIONS.iter()
    }
}
