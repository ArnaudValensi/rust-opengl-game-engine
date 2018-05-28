use std::ops::Add;
use super::direction::Direction;

pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Position {
            x,
            y,
            z,
        }
    }

    pub fn from_direction(direction: Direction) -> Position {
        match direction {
            Direction::North => Position::new(0, 0, 1),
            Direction::East => Position::new(1, 0, 0),
            Direction::South => Position::new(0, 0, -1),
            Direction::West => Position::new(-1, 0, 0),
            Direction::Up => Position::new(0, 1, 0),
            Direction::Down => Position::new(0, -1, 0),
        }
    }

    pub fn add_direction(&self, direction: Direction) -> Position {
        let direction_position = Position::from_direction(direction);

        Position {
            x: self.x + direction_position.x,
            y: self.y + direction_position.y,
            z: self.z + direction_position.z,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Position::from_direction(rhs)
    }
}
