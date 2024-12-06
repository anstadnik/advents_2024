use anyhow::{anyhow, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::{Ordering, Ordering::*};
use std::fs::read_to_string;
use winnow::combinator::{separated, separated_pair};
use winnow::{ascii::dec_uint, PResult, Parser};

fn parse_rules(input: &mut &str) -> PResult<(u32, u32)> {
    separated_pair(dec_uint, '|', dec_uint).parse_next(input)
}

fn parse_updates(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., dec_uint::<_, u32, _>, ",").parse_next(input)
}

fn cmp_rules(rules: &[(u32, u32)]) -> impl Fn(&u32, &u32) -> Ordering + use<'_> {
    |&n1, &n2| {
        let f = |&(a, b)| match (n1, n2) {
            t if t == (a, b) => Some(Less),
            t if t == (b, a) => Some(Greater),
            _ => None,
        };
        rules.iter().find_map(f).unwrap_or(Ordering::Equal)
    }
}

fn sorted_rules(rules: &[(u32, u32)]) -> impl Fn(&u32, &u32) -> bool + use<'_> {
    |n1, n2| cmp_rules(rules)(n1, n2) != Greater
}

type Rules = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

fn parse_input(input: &str) -> Result<(Rules, Updates)> {
    let mut it = input.lines();
    let rules: Vec<(u32, u32)> = it
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| parse_rules.parse(l).map_err(|e| anyhow!("{e}")))
        .collect::<Result<_>>()?;

    let updates = it
        .map(|l| parse_updates.parse(l).map_err(|e| anyhow!("{e}")))
        .collect::<Result<_>>()?;

    Ok((rules, updates))
}

fn task1(updates: &[Vec<u32>], rules: &[(u32, u32)]) -> u32 {
    updates
        .par_iter()
        .filter(|u| u.is_sorted_by(sorted_rules(rules)))
        .map(|u| u[u.len() / 2])
        .sum()
}
fn task2(updates: &[Vec<u32>], rules: &[(u32, u32)]) -> u32 {
    updates
        .par_iter()
        .filter(|&u| !u.is_sorted_by(sorted_rules(rules)))
        .map(|u| {
            let mut u = u.clone();
            u.sort_unstable_by(cmp_rules(rules));
            u[u.len() / 2]
        })
        .sum()
}

fn main() -> Result<()> {
    let (rules, updates) = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer1: {}", task1(&updates, &rules));
    println!("Answer2: {}", task2(&updates, &rules));

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
        let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (rules, updates) = parse_input(input)?;

        assert_eq!(task1(&updates, &rules), 143);
        assert_eq!(task2(&updates, &rules), 123);

        Ok(())
    }
}
