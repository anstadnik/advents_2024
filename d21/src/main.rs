mod pad;
use anyhow::Result;
use pad::{KeyPad, Pos};
use std::{fs::read_to_string, iter::repeat_n};

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

const KEY_PAD1: ([(char, Pos); 11], [(Pos, &str); 3]) = (
    [
        ('7', Pos { x: 0, y: 0 }),
        ('8', Pos { x: 1, y: 0 }),
        ('9', Pos { x: 2, y: 0 }),
        ('4', Pos { x: 0, y: 1 }),
        ('5', Pos { x: 1, y: 1 }),
        ('6', Pos { x: 2, y: 1 }),
        ('1', Pos { x: 0, y: 2 }),
        ('2', Pos { x: 1, y: 2 }),
        ('3', Pos { x: 2, y: 2 }),
        ('0', Pos { x: 1, y: 3 }),
        ('A', Pos { x: 2, y: 3 }),
    ],
    [
        (Pos { x: 1, y: 3 }, "<"),
        (Pos { x: 0, y: 2 }, "v"),
        (Pos { x: 2, y: 3 }, "<<"),
    ],
);

const KEY_PAD2: ([(char, Pos); 5], [(Pos, &str); 3]) = (
    [
        ('^', Pos { x: 1, y: 0 }),
        ('A', Pos { x: 2, y: 0 }),
        ('<', Pos { x: 0, y: 1 }),
        ('v', Pos { x: 1, y: 1 }),
        ('>', Pos { x: 2, y: 1 }),
    ],
    [
        (Pos { x: 0, y: 1 }, "^"),
        (Pos { x: 1, y: 0 }, "<"),
        (Pos { x: 2, y: 0 }, "<<"),
    ],
);

//trait CloneIter = Iterator + Clone;

fn gen_task_moves(input: &str, keypads: &[KeyPad]) -> String {
    let [first, keypads @ .., last] = keypads else {
        panic!()
    };
    let moves_it1 = first.gen_moves(input.chars());
    let moves_it2 = keypads.iter().fold(
        Box::new(moves_it1) as Box<dyn Iterator<Item = _>>,
        |moves_it, key_pad| Box::new(key_pad.gen_moves_iter(moves_it)),
    );
    last.gen_shortest_iter(moves_it2)
}

fn task1(input: &[String]) -> u32 {
    let keypads = [
        KeyPad::new(&KEY_PAD1.0, &KEY_PAD1.1),
        KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1),
        KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1),
    ];
    input
        .iter()
        .map(|line| {
            //println!(
            //    "{}, {}",
            //    gen_task_moves(line).len() as u32,
            //    line.strip_suffix(['A']).unwrap().parse::<u32>().unwrap()
            //);
            println!("{}", gen_task_moves(line, &keypads));
            gen_task_moves(line, &keypads).len() as u32
                * line.strip_suffix(['A']).unwrap().parse::<u32>().unwrap()
        })
        .sum()
}

fn task2(input: &[String]) -> u32 {
    let keypads: Vec<_> = [KeyPad::new(&KEY_PAD1.0, &KEY_PAD1.1)]
        .into_iter()
        .chain(repeat_n(KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1), 25))
        .chain([KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1)])
        .collect();
    input
        .iter()
        .map(|line| {
            println!("{}", gen_task_moves(line, &keypads));
            gen_task_moves(line, &keypads).len() as u32
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

        let key_pad1 = KeyPad::new(&KEY_PAD1.0, &KEY_PAD1.1);
        let key_pad2 = KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1);
        let key_pad3 = KeyPad::new(&KEY_PAD2.0, &KEY_PAD2.1);

        let keys1 = key_pad1.gen_shortest_iter([vec![input.to_string()]].into_iter());
        let ans1 = "<A^A^^>AvvvA";
        assert_eq!(keys1, ans1);
        println!("{}\n{}", keys1, ans1);
        let moves_it1 = key_pad1.gen_moves(input.chars());
        let keys2 = key_pad2.gen_shortest_iter(moves_it1.clone());
        //let ans2 = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let ans2 = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        assert_eq!(keys2, ans2);
        println!("{}\n{}", keys2, ans2);
        let moves_it2 = key_pad2.gen_moves_iter(moves_it1);
        let keys3 = key_pad3.gen_shortest_iter(moves_it2);
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
    }
}
