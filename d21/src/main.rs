mod pad;
use anyhow::Result;
use cached::proc_macro::cached;
use pad::{KEY_PAD1, KEY_PAD2};
use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[cached]
fn gen_task_moves(s: String, is_keypad_1: bool, n_keypads: usize) -> u64 {
    let keypad = if is_keypad_1 { KEY_PAD1 } else { KEY_PAD2 };

    if n_keypads == 0 {
        return s.len() as u64;
    }
    keypad
        .gen_moves_str(s)
        .into_iter()
        .map(|s| gen_task_moves(s, false, n_keypads - 1))
        .sum()
}

fn task(input: &[String], n: usize) -> u64 {
    input
        .iter()
        .map(|line| {
            gen_task_moves(line.to_string(), true, n)
                * line.strip_suffix('A').unwrap().parse::<u64>().unwrap()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task(&input, 3));
    println!("Answer 2: {}", task(&input, 26));

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
    fn test1() {
        let s = r"029A
980A
179A
456A
379A";
        let input = parse_input(s);

        assert_eq!(task(&input, 3), 126384);
    }
}
