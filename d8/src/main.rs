use anyhow::Result;
use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn task<const TASK2: bool>(input: &[Vec<char>]) -> usize {
    let mut locations = HashMap::<char, Vec<IVec2>>::new();
    for (y, row) in input.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                let p = IVec2::new(x as i32, y as i32);
                locations.entry(c).or_default().push(p);
            }
        }
    }

    let pred = |p: IVec2| {
        input
            .get(TryInto::<usize>::try_into(p.x).ok()?)?
            .get(TryInto::<usize>::try_into(p.y).ok()?)?;
        Some(p)
    };

    locations
        .into_iter()
        .flat_map(|(_, v)| v.into_iter().combinations(2).map(|p| p.try_into().unwrap()))
        .flat_map(|[a, b]: [_; 2]| -> Box<dyn Iterator<Item = _>> {
            let d = b - a;
            if !TASK2 {
                Box::new([pred(a - d), pred(b + d)].into_iter().flatten())
            } else {
                let a_it = (0..).map_while(move |n| pred(a - d * n));
                let b_it = (0..).map_while(move |n| pred(b + d * n));
                Box::new(a_it.chain(b_it))
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task::<false>(&input));
    println!("Answer 2: {}", task::<true>(&input));

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
    fn test_example1() -> Result<()> {
        let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let input = parse_input(input);
        assert_eq!(task::<false>(&input), 14);
        assert_eq!(task::<true>(&input), 34);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = r"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        let input = parse_input(input);
        assert_eq!(task::<false>(&input), 2);
        assert_eq!(task::<true>(&input), 5);

        Ok(())
    }
}
