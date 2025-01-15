mod parse;
mod tests;

use anyhow::{bail, Result};
use indicatif::ProgressIterator;
use parse::{parse_intput, Ops, State};
use std::{collections::HashMap, fs::read_to_string, mem::swap};

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
        if ops.len() == n {
            bail!("No progress");
        }
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
    //Ok(task1(init, ops)? == x & y)
}

fn find_fault_bit_task_2(init: State, ops: &Ops) -> Option<usize> {
    (0..init.len() / 2).find(|&i| {
        let x = (1 << i) - 1;
        let y = 1;
        //let x = 1 << i;
        //let y = 1 << i;
        !test_task_2_(init.clone(), x, y, ops).unwrap_or(false)
    })
}

fn gen_depsendencies_for<'a>(ops: &Ops<'a>, v: &'a str) -> Vec<&'a str> {
    if let Some(&(a, _, b, _)) = ops.iter().find(|a| a.3 == v) {
        [v].into_iter()
            .chain(gen_depsendencies_for(ops, a))
            .chain(gen_depsendencies_for(ops, b))
            .filter(|v| !v.starts_with('x') && !v.starts_with('y'))
            .collect()
    } else {
        vec![v]
    }
}

fn gen_dependencies<'a>(ops: &Ops<'a>) -> HashMap<usize, Vec<&'a str>> {
    ops.iter()
        .filter(|a| a.3.starts_with('z'))
        .fold(HashMap::new(), |mut acc, a| {
            acc.insert(
                a.3.strip_prefix('z').unwrap().parse().unwrap(),
                gen_depsendencies_for(ops, a.3),
            );
            acc
        })
}

fn task2_<'a>(
    init: State<'a>,
    ops: &Ops<'a>,
    prev_faulty_bit: Option<usize>,
    deps: &HashMap<usize, Vec<&'a str>>,
) -> Result<Option<Vec<&'a str>>> {
    let mut ops = ops.clone();
    if let Some(faulty_bit) = find_fault_bit_task_2(init.clone(), &ops) {
        if prev_faulty_bit.is_some_and(|prev_| faulty_bit <= prev_) {
            return Ok(None);
        }

        println!(
            "Faulty bit: {}, prev_faulty_bit: {:?}",
            faulty_bit, prev_faulty_bit
        );
        let deps_ = deps.get(&faulty_bit).unwrap();
        // TODO: Do I need ..= ?
        let deps_to_ignore: Vec<_> = (0..=faulty_bit)
            .flat_map(|i| deps.get(&i).unwrap())
            .copied()
            .collect();
        println!("Dependencies: {:?}", deps_);
        let deps_is: Vec<_> = deps_
            .iter()
            .map(|v| ops.iter().position(|a| &a.3 == v).unwrap())
            .collect();

        for mut i in deps_is.into_iter().progress() {
            for mut j in 0..ops.len() {
                if j == i || deps_to_ignore.contains(&ops[j].3) {
                    //if j == i {
                    continue;
                }

                if j < i {
                    swap(&mut j, &mut i);
                }
                //println!("Swapping {} and {}", i, j);

                let (a, b) = ops.split_at_mut(j);
                swap(&mut a[i].3, &mut b[0].3);

                if let Ok(Some(mut v)) = task2_(init.clone(), &ops, Some(faulty_bit), deps) {
                    v.extend([ops[i].3, ops[j].3]);
                    return Ok(Some(v));
                }

                let (a, b) = ops.split_at_mut(j);
                swap(&mut a[i].3, &mut b[0].3);
            }
        }
        Ok(None)
    } else {
        Ok(Some(Vec::new()))
    }
}

fn task2(init: State, ops: &Ops) -> Result<String> {
    let deps = gen_dependencies(ops);
    task2_(init, ops, None, &deps).map(|v| {
        let mut v = v.unwrap();
        v.sort_unstable();
        v.join(",")
    })
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let (init, ops) = parse_intput(s.trim())?;

    println!("Answer 1: {}", task1(init.clone(), &ops)?);
    println!("Answer 2: {}", task2(init, &ops)?);

    Ok(())
}
