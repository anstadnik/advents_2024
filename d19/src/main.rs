use anyhow::{anyhow, Result};
use cached::{proc_macro::cached, UnboundCache};
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fs::read_to_string;
use winnow::{
    ascii::alpha1,
    combinator::{separated, separated_pair},
    PResult, Parser,
};

fn parse_input<'a>(input: &mut &'a str) -> PResult<(Vec<&'a str>, Vec<&'a str>)> {
    separated_pair(
        separated(1.., alpha1, ", "),
        "\n\n",
        separated(1.., alpha1, "\n"),
    )
    .parse_next(input)
}

#[cached(
    ty = "UnboundCache<String, usize>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{}{}", design, towels.join(",")) }"#
)]
fn can_build(design: &str, towels: &[&str]) -> usize {
    if design.is_empty() {
        return 1;
    }
    towels
        .iter()
        .filter_map(|&t| design.strip_prefix(t).map(|d_| can_build(d_, towels)))
        .sum()
}

fn get_number_of_possible_builds(towels: &[&str], designs: &[&str]) -> Vec<usize> {
    designs
        .par_iter()
        .progress()
        .map(|d| can_build(d, towels))
        .collect()
}

fn task1(number_of_possible_builds: &[usize]) -> usize {
    number_of_possible_builds.iter().filter(|&&n| n > 0).count()
}

fn task2(number_of_possible_builds: &[usize]) -> usize {
    number_of_possible_builds.iter().sum()
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let (towels, designs) = parse_input.parse(s.trim()).map_err(|e| anyhow!("{e}"))?;

    let number_of_possible_builds = get_number_of_possible_builds(&towels, &designs);

    println!("Answer 1: {}", task1(&number_of_possible_builds));
    println!("Answer 2: {}", task2(&number_of_possible_builds));

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
    fn test_task1() {
        let input = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"#;

        let (towels, designs) = parse_input(&mut input.trim()).unwrap();
        let number_of_possible_builds = get_number_of_possible_builds(&towels, &designs);
        assert_eq!(task1(&number_of_possible_builds), 6);
        assert_eq!(task2(&number_of_possible_builds), 16);
    }
}
