use std::fs::read_to_string;

use anyhow::Result;

type N = u64;
fn parse_input(input: &str) -> Result<Vec<N>> {
    input.trim().split(" ").map(|l| Ok(l.parse()?)).collect()
}

type BIt = Box<dyn Iterator<Item = N>>;
fn blink(input: BIt) -> BIt {
    Box::new(input.flat_map(|n| {
        if n == 0 {
            [Some(1), None]
        } else {
            let n_digits = n.ilog10() + 1;
            let magic_n = (10 as N).pow(n_digits / 2);

            if n_digits % 2 == 0 {
                [Some(n / magic_n), Some(n % magic_n)]
            } else {
                [Some(n * 2024), None]
            }
        }
        .into_iter()
        .flatten()
    }))
}

fn task1(input: &[N], n: usize) -> usize {
    (0..n)
        //.progress()
        .fold(Box::new(input.to_vec().into_iter()) as BIt, |it, _| {
            blink(it)
        })
        .count()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {}", task1(&input, 25));
    println!("Answer 2: {}", task1(&input, 75));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_example1() -> Result<()> {
        let input = r"125 17";

        let input = parse_input(input)?;
        assert_eq!(task1(&input, 25), 55312);
        //assert_eq!(task2(&input), 81);

        Ok(())
    }
}
