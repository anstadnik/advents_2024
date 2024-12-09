use anyhow::Result;
use std::{fs::read_to_string, iter::repeat_n};

type N = u64;
fn parse_input(s: &str) -> Vec<N> {
    s.chars().map(|c| c.to_digit(10).unwrap() as _).collect()
}

fn task1(input: &[N]) -> N {
    let it = input.iter().step_by(2);
    let n = it.clone().sum::<N>() as usize;
    let mut files = it.enumerate().flat_map(|(i, &f)| repeat_n(i, f as _));
    let mut files_rev = files.clone().rev();
    input
        .iter()
        .enumerate()
        .flat_map(|(i, &n)| repeat_n(i % 2 == 0, n as usize))
        .map(|is_even| {
            if is_even {
                files.by_ref().next().unwrap()
            } else {
                files_rev.by_ref().next().unwrap()
            }
        })
        .take(n)
        .enumerate()
        .map(|(i, f)| i as N * f as N)
        .sum()
}

fn task2(input: &[N]) -> N {
    let it = input.iter().scan(0, |acc, &n| {
        let ret = (*acc, n);
        *acc += n;
        Some(ret)
    });

    let calc = |i, id, n| (i..i + n).map(|i| i * id).sum::<N>();
    let mut free: Vec<_> = it.clone().skip(1).step_by(2).collect();
    it.step_by(2)
        .enumerate()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|(id, (i, n))| {
            if let Some(free_i) = free.iter().position(|&(i_, n_)| i_ < i && n_ >= n) {
                let (i_, n_) = free[free_i];
                free[free_i] = (i_ + n, n_ - n);
                calc(i_, id as N, n)
            } else {
                calc(i, id as N, n)
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(read_to_string("input.txt")?.trim());

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
    fn test_example0() -> Result<()> {
        let input = r"12345";

        let input = parse_input(input);
        assert_eq!(task1(&input), 60);
        task2(&input);
        //assert_eq!(task::<true>(&input), 34);

        Ok(())
    }

    #[test]
    fn test_example1() -> Result<()> {
        let input = r"2333133121414131402";

        let input = parse_input(input);
        assert_eq!(task1(&input), 1928);
        assert_eq!(task2(&input), 2858);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = r"233313312141413140233";

        let input = parse_input(input);
        assert_eq!(task1(&input), 2584);
        //assert_eq!(task::<true>(&input), 34);

        Ok(())
    }
}
