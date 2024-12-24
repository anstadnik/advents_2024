use crate::enums::prelude::*;
use itertools::Itertools;
use std::fmt::Debug;

#[derive(PartialEq, Clone, Copy)]
pub enum Object {
    Obstacle,
    Obstacle2,
    Wall,
    Robot,
    Empty,
}
use Object::*;

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            'O' => Obstacle,
            '#' => Wall,
            '.' => Empty,
            '@' => Robot,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Field {
    m: Vec<Vec<Object>>,
    pos: Pos,
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.m {
            for obj in line {
                write!(f, "{obj:?}")?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn iter_coords(m: &[Vec<Object>]) -> impl Iterator<Item = Pos> + use<'_> {
    (0..m.len())
        .cartesian_product(0..m[0].len())
        .map(|(y, x)| Pos { y, x })
}

impl Field {
    pub fn new(field: &str) -> Self {
        let m: Vec<Vec<Object>> = field
            .lines()
            .map(|l| l.chars().map(From::from).collect())
            .collect();
        let pos = iter_coords(&m)
            .find(|&Pos { y, x }| m[y][x] == Robot)
            .unwrap();
        Self { m, pos }
    }

    pub fn double(&mut self) {
        let f = |&obj| match obj {
            Robot => Empty,
            Obstacle => Obstacle2,
            _ => obj,
        };
        self.m = self
            .m
            .iter()
            .map(|v| v.iter().copied().interleave(v.iter().map(f)).collect())
            .collect();
        self.pos.x *= 2;
    }

    pub fn get(&self, pos: Pos) -> Option<Object> {
        self.m.get(pos.y)?.get(pos.x).copied()
    }

    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut Object> {
        self.m.get_mut(pos.y)?.get_mut(pos.x)
    }

    fn swap_obj(&mut self, pos: Pos, new_pos: Pos) {
        let (obj, new_obj) = (self.get(pos).unwrap(), self.get(new_pos).unwrap());
        *self.get_mut(pos).unwrap() = new_obj;
        *self.get_mut(new_pos).unwrap() = obj;

        if obj == Robot {
            self.pos = new_pos;
        }
    }

    pub fn can_move(&self, pos: Pos, dir: Direction) -> bool {
        self.get(pos).unwrap().can_move(pos, dir, self)
    }

    pub fn move_obj(&mut self, pos: Pos, dir: Direction) {
        self.get(pos).unwrap().move_obj(pos, dir, self);
    }

    pub fn step(&mut self, dir: Direction, pos: Option<Pos>) -> bool {
        let pos = pos.unwrap_or(self.pos);
        if self.can_move(pos, dir) {
            self.move_obj(pos, dir);
            true
        } else {
            false
        }
    }

    pub fn get_gps_coords(&self) -> impl Iterator<Item = Pos> + use<'_> {
        iter_coords(&self.m).filter(|&pos| self.get(pos).unwrap() == Obstacle)
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Obstacle => write!(f, "O"),
            Obstacle2 => write!(f, "o"),
            Wall => write!(f, "#"),
            Robot => write!(f, "@"),
            Empty => write!(f, "."),
        }
    }
}

impl Object {
    fn can_move_(&self, pos: Pos, dir: Direction, field: &Field) -> Option<bool> {
        let right_pos = Right.get_new_pos(pos);
        Some(match self {
            Obstacle if field.get(right_pos) == Some(Obstacle2) => match dir {
                Up | Down => {
                    field.can_move(dir.get_new_pos(pos), dir)
                        && field.can_move(dir.get_new_pos(right_pos), dir)
                }
                Right => field.can_move(dir.get_new_pos(right_pos), dir),
                Left => field.can_move(dir.get_new_pos(pos), dir),
            },
            Obstacle | Robot => field.can_move(dir.get_new_pos(pos), dir),
            Obstacle2 => field.can_move(Left.get_new_pos(pos), dir),
            Wall => false,
            Empty => true,
        })
    }

    pub fn can_move(&self, pos: Pos, dir: Direction, field: &Field) -> bool {
        self.can_move_(pos, dir, field).is_some_and(|b| b)
    }

    pub fn move_obj(&self, pos: Pos, dir: Direction, field: &mut Field) {
        let right_pos = Right.get_new_pos(pos);
        match self {
            Obstacle if field.get(right_pos) == Some(Obstacle2) => {
                let new_right_pos = dir.get_new_pos(right_pos);
                match dir {
                    Up | Down => {
                        field.move_obj(dir.get_new_pos(pos), dir);
                        field.move_obj(dir.get_new_pos(right_pos), dir);
                        field.swap_obj(pos, dir.get_new_pos(pos));
                        field.swap_obj(right_pos, dir.get_new_pos(right_pos));
                    }
                    Right => {
                        field.move_obj(dir.get_new_pos(right_pos), dir);
                        field.swap_obj(right_pos, new_right_pos);
                        field.swap_obj(pos, dir.get_new_pos(pos));
                    }
                    Left => {
                        field.move_obj(dir.get_new_pos(pos), dir);
                        field.swap_obj(pos, dir.get_new_pos(pos));
                        field.swap_obj(right_pos, new_right_pos);
                    }
                }
            }
            Obstacle | Robot => {
                field.move_obj(dir.get_new_pos(pos), dir);
                field.swap_obj(pos, dir.get_new_pos(pos));
            }
            Obstacle2 => field.move_obj(Left.get_new_pos(pos), dir),
            Empty => (),
            Wall => unreachable!(),
        }
    }
}

pub mod prelude {
    pub use super::Field;
}
