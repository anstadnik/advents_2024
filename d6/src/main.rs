use anyhow::{anyhow, Result};
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, fs::read_to_string};
use Direction::*;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Guard {
    pos: (usize, usize),
    dir: Direction,
}

impl Guard {
    fn new(pos: (usize, usize), dir: char) -> Self {
        let dir = match dir {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => unreachable!(),
        };
        Self { pos, dir }
    }

    fn step(&mut self) {
        self.pos = self.get_next_pos().unwrap();
    }

    fn get_next_pos(&self) -> Option<(usize, usize)> {
        Some(match self.dir {
            Up => (self.pos.0.checked_sub(1)?, self.pos.1),
            Down => (self.pos.0 + 1, self.pos.1),
            Left => (self.pos.0, self.pos.1.checked_sub(1)?),
            Right => (self.pos.0, self.pos.1 + 1),
        })
    }

    fn turn(&mut self) {
        self.dir = match self.dir {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
    }
}

fn parse_input(input: &str) -> Result<(Vec<Vec<char>>, Guard)> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let pos = grid
        .iter()
        .enumerate()
        .find_map(|(i, l)| {
            l.iter()
                .position(|&c| c == '^' || c == '>' || c == 'v' || c == '<')
                .map(|j| (i, j))
        })
        .ok_or(anyhow!("Cannot find!"))?;
    let guard = Guard::new(pos, grid[pos.0][pos.1]);
    grid[guard.pos.0][guard.pos.1] = '.';
    Ok((grid, guard))
}

fn task1(grid: &[Vec<char>], mut guard: Guard) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut history = HashSet::new();
    loop {
        visited.insert(guard.pos);
        if !history.insert(guard) {
            return None;
        }
        let Some(next_pos) = guard.get_next_pos() else {
            break;
        };
        match grid.get(next_pos.0).and_then(|l| l.get(next_pos.1)) {
            Some('#') => guard.turn(),
            Some('.') => guard.step(),
            None => break,
            _ => unreachable!(),
        }
    }
    Some(visited.len())
}

//fn task2_slow(grid: &[Vec<char>], guard: Guard) -> usize {
//    let (n, m) = (grid.len(), grid[0].len());
//    let mut grid = grid.to_vec();
//    (0..n)
//        .into_iter()
//        .progress()
//        .flat_map(|i| (0..m).into_iter().map(move |j| (i, j)))
//        .filter(|&(i, j)| (i, j) != guard.pos)
//        //.filter(|&(i, j)| grid[i][j] == '.')
//        .filter(|&(i, j)| {
//            if grid[i][j] != '.' {
//                return false;
//            }
//            //let mut grid = grid.to_vec();
//            grid[i][j] = '#';
//            let ret = task1(&grid, guard).is_none();
//            grid[i][j] = '.';
//            ret
//        })
//        .count()
//}

fn task2(grid: &[Vec<char>], guard: Guard) -> usize {
    let (n, m) = (grid.len(), grid[0].len());
    (0..n)
        .into_par_iter()
        .progress()
        .flat_map(|i| (0..m).into_par_iter().map(move |j| (i, j)))
        .filter(|&(i, j)| (i, j) != guard.pos)
        .filter(|&(i, j)| grid[i][j] == '.')
        .filter(|&(i, j)| {
            let mut grid = grid.to_vec();
            grid[i][j] = '#';
            task1(&grid, guard).is_none()
        })
        .count()
}

fn main() -> Result<()> {
    let (grid, guard) = parse_input(&read_to_string("input.txt")?)?;

    println!(
        "Answer 1: {}",
        task1(&grid, guard).ok_or(anyhow!("Got looped"))?
    );
    println!("Answer 2: {}", task2(&grid, guard));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let (grid, guard) = parse_input(input)?;

        assert_eq!(task1(&grid, guard).ok_or(anyhow!("Get looped"))?, 41);
        assert_eq!(task2(&grid, guard), 6);

        Ok(())
    }
}
