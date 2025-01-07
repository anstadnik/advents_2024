use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, fs::read_to_string, iter::successors};

type N = i64;
const SECRET: N = 16777216;

fn parse_input(s: &str) -> Vec<N> {
    s.lines().map(|line| line.parse().unwrap()).collect()
}

fn gen_secrets(input: &[N], n: usize) -> Vec<Vec<N>> {
    let step = |&s: &N| {
        let s = ((s * 64) ^ s) % SECRET;
        let s = ((s / 32) ^ s) % SECRET;
        Some(((s * 2048) ^ s) % SECRET)
    };
    input
        .iter()
        .map(|&s| successors(Some(s), step).take(n).collect())
        .collect()
}

fn task1(input: &[N], n: usize) -> N {
    gen_secrets(input, n).iter().map(|s| s[n - 1]).sum()
}

fn task2(input: &[N], n: usize, thr: usize) -> N {
    let secrets: Vec<Vec<_>> = gen_secrets(input, n)
        .into_iter()
        .map(|v| v.into_iter().map(|s| s % 10).collect())
        .collect();
    let diffs: Vec<Vec<_>> = secrets
        .iter()
        .map(|v| v.windows(2).map(|w| w[1] - w[0]).collect())
        .collect();

    diffs
        .iter()
        .flat_map(|v| v.windows(4))
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
        .into_par_iter()
        .filter(|&(_, v)| v >= thr)
        .map(|(opt, _)| {
            diffs
                .iter()
                .zip(&secrets)
                .filter_map(|(d, s)| {
                    d.windows(4)
                        .zip(&s[4..])
                        .find_map(|(w, &s)| (w == opt).then_some(s))
                })
                .sum()
        })
        .max()
        .unwrap()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task1(&input, 2000));
    println!("Answer 2: {}", task2(&input, 2000, 250));

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
    fn test_simple() {
        let s = "123";
        let input = parse_input(s);
        assert_eq!(task2(&input, 10, 1), 6);
    }

    #[test]
    fn test_task1() {
        let s = r"1
10
100
2024";
        let input = parse_input(s);
        assert_eq!(task1(&input, 2001), 37327623);
    }

    #[test]
    fn test_task2() {
        let s = r"1
2
3
2024";
        let input = parse_input(s);
        assert_eq!(task2(&input, 2001, 0), 23);
    }
}
