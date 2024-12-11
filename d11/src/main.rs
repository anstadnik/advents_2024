use anyhow::Result;
use cached::proc_macro::cached;
use std::fs::read_to_string;

type N = u64;
fn parse_input(input: &str) -> Result<Vec<N>> {
    input.trim().split(" ").map(|l| Ok(l.parse()?)).collect()
}

#[cached]
fn blink(n: N, i: usize) -> N {
    if i == 0 {
        1
    } else if n == 0 {
        blink(1, i - 1)
    } else {
        let n_digits = n.ilog10() + 1;
        let magic_n = (10 as N).pow(n_digits / 2);

        if n_digits % 2 == 0 {
            blink(n / magic_n, i - 1) + blink(n % magic_n, i - 1)
        } else {
            blink(n * 2024, i - 1)
        }
    }
}

fn task(input: &[N], i: usize) -> N {
    input.iter().map(|&n| blink(n, i)).sum()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {}", task(&input, 25));
    println!("Answer 2: {}", task(&input, 75));

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
        let input = r"125 17";

        let input = parse_input(input)?;
        assert_eq!(task(&input, 25), 55312);
        //assert_eq!(task2(&input), 81);

        Ok(())
    }
}
