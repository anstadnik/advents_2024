use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
use Direction::*;

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    pub fn get_new_pos(&self, Pos { y, x }: Pos) -> Pos {
        match self {
            Up => Pos { y: y - 1, x },
            Right => Pos { y, x: x + 1 },
            Down => Pos { y: y + 1, x },
            Left => Pos { y, x: x - 1 },
        }
    }
}

pub mod prelude {
    pub use super::{Direction, Direction::*, Pos};
}
