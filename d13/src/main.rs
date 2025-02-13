use anyhow::{anyhow, Result};
use glam::I64Vec2;
use std::{fs::read_to_string, str::FromStr};
use winnow::{ascii::dec_int, seq, PResult, Parser};

type N = i64;
type V = I64Vec2;

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: V,
    b: V,
    p: V,
}

fn parse_machine(input: &mut &str) -> PResult<Machine> {
    seq! {
        Machine {
            a: seq! { V::new(_: "Button A: X", dec_int, _: ", Y", dec_int,) },
            b: seq! { V::new(_: "\nButton B: X", dec_int, _: ", Y", dec_int,) },
            p: seq! { V::new(_: "\nPrize: X=", dec_int, _: ", Y=", dec_int,) },
        }
    }
    .parse_next(input)
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_machine.parse(s).map_err(|e| anyhow!("{e}"))
    }
}

fn parse_input(input: &str) -> Result<Vec<Machine>> {
    input.trim().split("\n\n").map(|s| s.parse()).collect()
}

fn task_(Machine { a, b, p }: Machine, max: N) -> Option<N> {
    let na = (p.x * b.y - p.y * b.x) / (a.x * b.y - a.y * b.x);
    let nb = (-p.x * a.y + p.y * a.x) / (a.x * b.y - a.y * b.x);
    (a * na + b * nb == p && na >= 0 && nb >= 0 && na <= max && nb <= max).then(|| na * 3 + nb)
}

fn task1(input: &[Machine]) -> N {
    input.iter().filter_map(|m| task_(*m, 100)).sum()
}

fn task2(input: &[Machine]) -> N {
    input
        .iter()
        .filter_map(|m| {
            let m = Machine {
                p: m.p + V::splat(10_000_000_000_000),
                ..*m
            };
            task_(m, N::MAX)
        })
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
    fn test_example1() -> Result<()> {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        let input = parse_input(input)?;
        assert_eq!(task1(&input), 480);

        Ok(())
    }
}
