use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;

fn parse_input<const IGNORE_CONDITIONALS: bool>(line: &str) -> Result<u32> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?
        .captures_iter(line)
        .try_fold((true, 0), |(enabled, sum), capture| {
            Ok(match &capture[0] {
                "do()" => (true, sum),
                "don't()" => (false, sum),
                _ if enabled || IGNORE_CONDITIONALS => (
                    enabled,
                    sum + capture[1].parse::<u32>()? * capture[2].parse::<u32>()?,
                ),
                _ => (enabled, sum),
            })
        })
        .map(|(_, sum)| sum)
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
