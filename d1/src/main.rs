use anyhow::Result;
use std::{collections::HashMap, fs::read_to_string, iter::zip};
use winnow::{
    ascii::{dec_uint, space1},
    combinator::separated_pair,
    error::{ContextError, ParseError},
    Parser,
};

fn main() -> Result<()> {
    let (mut v1, mut v2) = read_to_string("input.txt")?
        .lines()
        .map(|l| -> Result<(u32, u32)> {
            separated_pair(dec_uint, space1, dec_uint)
                .parse(l)
                .map_err(|e: ParseError<_, ContextError>| anyhow::format_err!("{e}"))
        })
        .collect::<Result<(Vec<_>, Vec<_>)>>()?;

    v1.sort_unstable();
    v2.sort_unstable();

    let dist = zip(&v1, &v2).map(|(&a, &b)| a.abs_diff(b)).sum::<u32>();
    println!("Distance: {dist}");

    let counts = v2.into_iter().fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_insert(0) += 1;
        acc
    });

    let dist2 = v1
        .into_iter()
        .map(|n| n * counts.get(&n).unwrap_or(&0))
        .sum::<u32>();

    println!("Distance 2: {dist2}");

    Ok(())
}
