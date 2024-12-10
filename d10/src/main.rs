use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs::read_to_string, iter::once};

const OFFSETS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
type Pos = (usize, usize);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|d| d.to_digit(10).unwrap()).collect())
        .collect()
}

fn count_trails(input: &[Vec<u32>], pos: Pos) -> Box<dyn Iterator<Item = Pos> + '_> {
    let (y, x) = pos;
    match input[y][x] {
        9 => Box::new(once(pos)),
        v => Box::new(
            OFFSETS
                .iter()
                .filter_map(move |&(dy, dx)| {
                    let (y_, x_) = (y.checked_add_signed(dy)?, x.checked_add_signed(dx)?);
                    (*input.get(y_)?.get(x_)? == v + 1).then(|| count_trails(input, (y_, x_)))
                })
                .flatten(),
        ),
    }
}

fn task1(input: &[Vec<u32>]) -> usize {
    let (n, m) = (input.len(), input[0].len());
    (0..n)
        .cartesian_product(0..m)
        .filter(|(y, x)| input[*y][*x] == 0)
        .map(|(y, x)| count_trails(input, (y, x)).collect::<HashSet<_>>().len())
        .sum()
}

fn task2(input: &[Vec<u32>]) -> usize {
    let (n, m) = (input.len(), input[0].len());
    (0..n)
        .cartesian_product(0..m)
        .filter(|(y, x)| input[*y][*x] == 0)
        .map(|(y, x)| count_trails(input, (y, x)).count())
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task1(&input));
    println!("Answer 2: {}", task2(&input));

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
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let input = parse_input(input);
        assert_eq!(task1(&input), 36);
        assert_eq!(task2(&input), 81);

        Ok(())
    }
}
