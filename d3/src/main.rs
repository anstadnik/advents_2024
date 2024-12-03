use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
enum Instuction {
    Mult(u32, u32),
    Do,
    Dont,
}

use Instuction::*;

fn parse_input<const IGNORE_CONDITIONALS: bool>(line: &str) -> Result<u32> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let mults = re.captures_iter(line).map(|c| {
        Ok((
            c.get(0).unwrap().start(),
            Mult(c[1].parse::<u32>()?, c[2].parse::<u32>()?),
        ))
    });

    let re = Regex::new(r"do\(\)")?;
    let dos = re
        .captures_iter(line)
        .map(|c| Ok((c.get(0).unwrap().start(), Do)));

    let re = Regex::new(r"don't\(\)")?;
    let donts = re
        .captures_iter(line)
        .map(|c| Ok((c.get(0).unwrap().start(), Dont)));

    let mut instructions = mults.chain(dos).chain(donts).collect::<Result<Vec<_>>>()?;
    instructions.sort_unstable_by_key(|(i, _)| *i);

    let mut enabled = true;
    let mut sum = 0;
    for (_, instruction) in instructions {
        match instruction {
            Mult(a, b) if enabled || IGNORE_CONDITIONALS => sum += a * b,
            Do => enabled = true,
            Dont => enabled = false,
            _ => (),
        }
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;

    println!("Answer to part 1: {}", parse_input::<true>(&s)?);
    println!("Answer to part 2: {}", parse_input::<false>(&s)?);

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
        let input1 = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_input::<true>(input1)?, 161);

        let input2 = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(parse_input::<false>(input2)?, 48);

        Ok(())
    }
}
