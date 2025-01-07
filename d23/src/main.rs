use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use winnow::combinator::{separated, separated_pair};
use winnow::{ascii::alpha1, PResult, Parser};

fn parse_input<'a>(s: &mut &'a str) -> PResult<Vec<(&'a str, &'a str)>> {
    separated(1.., separated_pair(alpha1, '-', alpha1), "\n").parse_next(s)
}

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
fn gen_graph(s: &str) -> Result<Graph> {
    Ok(parse_input
        .parse(s)
        .map_err(|e| anyhow!("{e}"))?
        .into_iter()
        .fold(HashMap::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().push(b);
            acc.entry(b).or_default().push(a);
            acc
        }))
}

fn gen_connected<'a>(g: &'a Graph<'a>, n: usize) -> impl ParallelIterator<Item = Vec<&'a str>> {
    g.par_iter().flat_map_iter(move |(&k, v)| {
        v.iter()
            .copied()
            .combinations(n - 1)
            .filter(|v| v.iter().tuple_combinations().all(|(a, b)| g[a].contains(b)))
            .map(move |mut v| {
                v.push(k);
                v
            })
    })
}

fn task1(graph: &Graph) -> usize {
    gen_connected(graph, 3)
        .filter(|group| group.iter().any(|n| n.starts_with('t')))
        .map(|mut s| {
            s.sort_unstable();
            s
        })
        .collect::<HashSet<_>>()
        .len()
}

fn task2(graph: &Graph) -> Option<String> {
    let mut v = (2..graph.len())
        .into_par_iter()
        .find_map_last(|n| gen_connected(graph, n).find_first(|_| true))?;
    v.sort_unstable();
    Some(v.join(","))
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let graph = gen_graph(s.trim())?;

    println!("Answer 1: {}", task1(&graph));
    println!("Answer 2: {}", task2(&graph).ok_or(anyhow!("No solution"))?);

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
    fn test_task() -> Result<()> {
        let input = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let graph = gen_graph(input).unwrap();
        assert_eq!(task1(&graph), 7);
        assert_eq!(task2(&graph).ok_or(anyhow!("No solution"))?, "co,de,ka,ta");

        Ok(())
    }
}
