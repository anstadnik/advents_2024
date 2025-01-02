mod pad;
use anyhow::Result;
use pad::{KeyPad, KEY_PAD1, KEY_PAD2};
use std::{fs::read_to_string, iter::repeat_n};

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn gen_task_moves<'a>(
    input: &'a str,
    keypads: &'a [KeyPad],
) -> Box<dyn Iterator<Item = char> + 'a> {
    let [first, keypads @ .., last] = keypads else {
        panic!()
    };
    let moves_it1 = first.gen_moves_str(Box::new(input.chars()));
    let moves_it2 = keypads.iter().fold(moves_it1, |moves_it, key_pad| {
        key_pad.gen_moves_str(moves_it)
    });
    last.gen_moves_str(moves_it2)
}

fn task1(input: &[String]) -> u32 {
    let keypads = [KEY_PAD1, KEY_PAD2, KEY_PAD2];
    input
        .iter()
        .map(|line| {
            gen_task_moves(line, &keypads).count() as u32
                * line.strip_suffix(['A']).unwrap().parse::<u32>().unwrap()
        })
        .sum()
}

fn task2(input: &[String]) -> u32 {
    let keypads: Vec<_> = [KEY_PAD1]
        .into_iter()
        //.chain(repeat_n(KEY_PAD2, 25))
        .chain(repeat_n(KEY_PAD2, 15))
        .collect();
    input
        .iter()
        .map(|line| {
            gen_task_moves(line, &keypads).count() as u32
                * line.strip_suffix(['A']).unwrap().parse::<u32>().unwrap()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task1(&input));
    println!("Answer 2: {}", task2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_simple() {
        let input = "029A";
        println!("{input}");

        let key_pad1 = KEY_PAD1;
        let key_pad2 = KEY_PAD2;
        let key_pad3 = KEY_PAD2;

        let keys1 = key_pad1
            .gen_moves_str(Box::new(input.chars()))
            .collect::<String>();
        let ans1 = "<A^A^^>AvvvA";
        assert_eq!(keys1, ans1);
        println!("{}\n{}", keys1, ans1);
        let keys2 = key_pad2
            .gen_moves_str(Box::new(keys1.chars()))
            .collect::<String>();
        //let ans2 = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let ans2 = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        assert_eq!(keys2, ans2);
        println!("{}\n{}", keys2, ans2);
        let keys3 = key_pad3
            .gen_moves_str(Box::new(keys2.chars()))
            .collect::<String>();
        //let ans3 = "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";
        let ans3 = "v<A<AA>>^AvAA<^A>Av<<A>>^AvA^Av<A>^Av<<A>^A>AAvA^Av<<A>A>^AAAvA<^A>A";
        //assert_eq!(keys3, ans3);
        println!("{}\n{}", keys3, ans3);
        panic!();
    }

    //#[ignore]
    #[test]
    fn test1() {
        let s = r"029A
980A
179A
456A
379A";
        //let s = r"029A";
        let input = parse_input(s);

        assert_eq!(task1(&input), 126384);
        task2(&input);
    }
}
