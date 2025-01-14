mod parse;

use anyhow::{anyhow, Result};
use parse::{parse_intput, Ops, State};
use std::fs::read_to_string;

fn execute<'a>(mut wires: State<'a>, ops: &'a Ops<'a>) -> Result<State<'a>> {
    let mut ops = ops.clone();

    while !ops.is_empty() {
        let n = ops.len();
        ops.retain(|(a, op, b, c)| {
            if let Some((a, b)) = wires.get(a).zip(wires.get(b)) {
                wires.insert(*c, op.apply(*a, *b));
                false
            } else {
                true
            }
        });
        assert!(ops.len() < n, "No progress");
    }

    Ok(wires)
}

type N = i64;
fn task1(init: State, ops: &Ops) -> Result<N> {
    let wires = execute(init, ops)?;
    let mut v: Vec<_> = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();
    v.sort_by_key(|&(k, _)| k);
    v.reverse();
    Ok(v.into_iter().fold(0, |acc, (_, v)| acc * 2 + v as N))
}

fn test_task_2_(mut init: State, x: N, y: N, ops: &Ops) -> Result<bool> {
    let n_bits = init.len() / 2;
    for i in 0..n_bits {
        *init.get_mut(format!("x{i:02}").as_str()).unwrap() = (x >> i) & 1 == 1
    }
    for i in 0..n_bits {
        *init.get_mut(format!("y{i:02}").as_str()).unwrap() = (y >> i) & 1 == 1
    }
    Ok(task1(init, ops)? == x + y)
}

fn test_task_2(init: State, ops: &Ops) -> Result<N> {
    let n_bits = init.len() / 2;
    let x = 0..1 << n_bits;
    let y = 0..1 << n_bits;
    x.zip(y)
        .find_map(|(x, y)| {
            test_task_2_(init.clone(), x, y, ops)
                .ok()
                .filter(|&b| b)
                .map(|_| x + y)
        })
        .ok_or_else(|| anyhow!("No solution"))
}

fn task2(init: State, ops: &Ops) -> Result<String> {
    let n = ops.len();
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let (init, ops) = parse_intput(s.trim())?;

    println!("Answer 1: {}", task1(init.clone(), &ops)?);
    println!("Answer 2: {}", task2(init, &ops)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_task1() -> Result<()> {
        let s = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let (init, ops) = parse_intput(s)?;

        assert_eq!(init.len(), 6);
        assert_eq!(ops.len(), 3);

        assert_eq!(task1(init.clone(), &ops)?, 4);
        assert_eq!(task2(init, &ops)?, "z00,z01,z02,z05");

        Ok(())
    }
    #[test]
    fn test_task2() -> Result<()> {
        let s = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let (init, ops) = parse_intput(s)?;

        assert_eq!(task1(init.clone(), &ops)?, 2024);
        assert_eq!(task2(init, &ops)?, "aaa,aoc,bbb,ccc,eee,ooo,z24,z99");

        Ok(())
    }
}
