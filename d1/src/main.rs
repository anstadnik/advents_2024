use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::str::ParallelString;
use std::{collections::HashMap, fs::read_to_string, iter::zip};
use winnow::{
    ascii::{dec_uint, space1},
    combinator::separated_pair,
    error::{ContextError, ParseError},
    Parser,
};

fn parse_input(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    let (mut v1, mut v2) = input
        .par_lines()
        .map(|l| -> Result<(u32, u32)> {
            separated_pair(dec_uint, space1, dec_uint)
                .parse(l)
                .map_err(|e: ParseError<_, ContextError>| anyhow::format_err!("{e}"))
        })
        .collect::<Result<(Vec<_>, Vec<_>)>>()?;

    v1.sort_unstable();
    v2.sort_unstable();
    Ok((v1, v2))
}

fn task1(v1: &Vec<u32>, v2: &Vec<u32>) -> u32 {
    zip(v1, v2).map(|(&a, &b)| a.abs_diff(b)).sum::<u32>()
}

fn task2(v1: &[u32], v2: &[u32]) -> u32 {
    let counts = v2.iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });

    v1.par_iter()
        .map(|n| n * counts.get(&n).unwrap_or(&0))
        .sum()
}

fn main() -> Result<()> {
    let (v1, v2) = parse_input(&read_to_string("input.txt")?)?;

    println!("Task 1: {}", task1(&v1, &v2));
    println!("Task 2: {}", task2(&v1, &v2));

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
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        let (v1, v2) = parse_input(input)?;

        assert_eq!(task1(&v1, &v2), 11);
        assert_eq!(task2(&v1, &v2), 31);

        Ok(())
    }
}
