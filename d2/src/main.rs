use anyhow::Result;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::str::ParallelString;
use std::fs::read_to_string;
use winnow::ascii::dec_int;
use winnow::error::{ContextError, ParseError};
use winnow::{combinator::separated, Parser};

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>> {
    input
        .par_lines()
        .map(|l| {
            separated(1.., dec_int::<_, i32, _>, " ")
                .parse(l)
                .map_err(|e: ParseError<_, ContextError>| anyhow::format_err!("{e}"))
        })
        .collect::<Result<Vec<Vec<i32>>>>()
}

fn check_it(mut it: impl Iterator<Item = i32> + Clone) -> bool {
    it.clone().all(|d| (1..=3).contains(&d.abs())) && {
        let first = it.next().unwrap().signum();
        it.all(|d| d.signum() == first)
    }
}

fn task1(data: &[Vec<i32>]) -> usize {
    data.par_iter()
        .filter(|v| check_it(v.windows(2).map(|w| w[1] - w[0])))
        .count()
}

fn task2(data: &[Vec<i32>]) -> usize {
    data.par_iter()
        .filter(|v| {
            check_it(v.windows(2).map(|w| w[1] - w[0]))
                || (0..v.len()).any(|i| {
                    let it = v.iter().enumerate().filter(|&(i_, _)| i != i_);
                    check_it(it.tuple_windows().map(|((_, a), (_, b))| b - a))
                })
        })
        .count()
}

fn main() -> Result<()> {
    let data = parse_input(&read_to_string("input.txt")?)?;

    println!("Task 1: {}", task1(&data));
    println!("Task 2: {}", task2(&data));

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
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let data = parse_input(input)?;

        assert_eq!(task1(&data), 2);
        assert_eq!(task2(&data), 4);

        Ok(())
    }
}
