use anyhow::Result;
use itertools::Itertools;
use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Schematic {
    Lock([u8; 5]),
    Key([u8; 5]),
}

use Schematic::*;

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut counts = s.lines().fold([0; 5], |mut counts, line| {
            for (n, c) in counts.iter_mut().zip(line.chars()) {
                *n += (c == '#') as u8;
            }
            counts
        });
        counts.iter_mut().for_each(|n| *n -= 1);
        Ok(if s.lines().next().unwrap().chars().all(|c| c == '#') {
            Lock(counts)
        } else {
            Key(counts)
        })
    }
}

fn parse_input(s: &str) -> Result<Vec<Schematic>> {
    s.split("\n\n").map(|s| s.parse()).collect()
}

fn task1(input: &[Schematic]) -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) = input.iter().partition(|s| matches!(s, Lock(_)));

    locks
        .iter()
        .cartesian_product(&keys)
        .filter(|&(l, k)| {
            let (Lock(l), Key(k)) = (l, k) else {
                unreachable!()
            };
            l.iter().zip(k.iter()).all(|(l, k)| l + k <= 5)
        })
        .count()
}

fn main() -> Result<()> {
    let input = parse_input(read_to_string("input.txt")?.trim())?;
    println!("Answer 1: {}", task1(&input));

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
    fn test_task1() -> Result<()> {
        let s = r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let input = parse_input(s.trim())?;
        assert_eq!(task1(&input), 3);

        Ok(())
    }
}
