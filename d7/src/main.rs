use anyhow::{anyhow, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::read_to_string;
use winnow::combinator::{separated, separated_pair};
use winnow::{ascii::dec_uint, PResult, Parser};

fn parse_line(line: &mut &str) -> PResult<(u64, Vec<u64>)> {
    separated_pair(dec_uint, ": ", separated(1.., dec_uint::<_, u64, _>, " ")).parse_next(line)
}

fn parse_input(input: &str) -> Result<Vec<(u64, Vec<u64>)>> {
    input
        .lines()
        .map(|line| parse_line.parse(line).map_err(|e| anyhow!("{e}")))
        .collect()
}

fn solve_eq1(n: u64, rez: u64, ns: &[u64]) -> bool {
    match ns {
        [n_, ns @ ..] => solve_eq1(n, rez + n_, ns) || solve_eq1(n, rez.max(1) * n_, ns),
        [] => n == rez,
    }
}

fn task1(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        .filter(|(n, ns)| solve_eq1(*n, 0, ns))
        .map(|&(n, _)| n)
        .sum()
}

fn solve_eq2(n: u64, rez: u64, ns: &[u64]) -> bool {
    match ns {
        [n_, ns @ ..] => {
            solve_eq2(n, rez + n_, ns)
                || solve_eq2(n, rez.max(1) * n_, ns)
                || solve_eq2(n, rez * 10_u64.pow(n_.ilog10() + 1) + *n_, ns)
        }
        _ => n == rez,
    }
}

fn task2(input: &[(u64, Vec<u64>)]) -> u64 {
    input
        .par_iter()
        .filter(|(n, ns)| solve_eq2(*n, 0, ns))
        .map(|&(n, _)| n)
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?)?;

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
    fn test_example() -> Result<()> {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let input = parse_input(input)?;
        assert_eq!(task1(&input), 3749);
        assert_eq!(task2(&input), 11387);

        Ok(())
    }
}
