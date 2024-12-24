#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
    pub d: Dir,
}

impl Pos {
    pub fn step(&self) -> Option<Pos> {
        Some(match self.d {
            Up => Pos {
                y: self.y.checked_sub(1)?,
                ..*self
            },
            Down => Pos {
                y: self.y + 1,
                ..*self
            },
            Left => Pos {
                x: self.x.checked_sub(1)?,
                ..*self
            },
            Right => Pos {
                x: self.x + 1,
                ..*self
            },
        })
    }

    pub fn turn(&self) -> [Pos; 2] {
        match self.d {
            Up => [Pos { d: Left, ..*self }, Pos { d: Right, ..*self }],
            Down => [Pos { d: Right, ..*self }, Pos { d: Left, ..*self }],
            Left => [Pos { d: Down, ..*self }, Pos { d: Up, ..*self }],
            Right => [Pos { d: Up, ..*self }, Pos { d: Down, ..*self }],
        }
    }
}
