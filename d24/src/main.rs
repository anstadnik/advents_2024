mod parse;
mod tests;

use anyhow::{bail, Result};
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressStyle};
use itertools::Itertools;
use parse::{parse_intput, Ops, State};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::{collections::HashMap, fs::read_to_string, mem::swap, sync::Mutex};

const RUN_AND: bool = false;

fn execute<'a>(mut wires: State<'a>, mut ops: Ops<'a>) -> Result<State<'a>> {
    while !ops.is_empty() {
        let n = ops.len();
        ops.retain(|(a, op, b, c)| {
            || -> Option<()> {
                wires.insert(*c, op.apply(*wires.get(a)?, *wires.get(b)?));
                Some(())
            }()
            .is_none()
        });
        if ops.len() == n {
            bail!("No progress");
        }
    }

    Ok(wires)
}

type N = i64;
fn task1<'a>(init: State<'a>, ops: Ops<'a>) -> Result<N> {
    let wires = execute(init, ops)?;
    let mut v: Vec<_> = wires
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect();
    v.sort_by_key(|&(k, _)| k);
    v.reverse();
    Ok(v.into_iter().fold(0, |acc, (_, v)| acc * 2 + v as N))
}

fn test_task_2_<'a>(mut init: State<'a>, x: N, y: N, ops: Ops<'a>) -> Result<bool> {
    let n_bits = init.len() / 2;
    for i in 0..n_bits {
        *init.get_mut(format!("x{i:02}").as_str()).unwrap() = (x >> i) & 1 == 1
    }
    for i in 0..n_bits {
        *init.get_mut(format!("y{i:02}").as_str()).unwrap() = (y >> i) & 1 == 1
    }
    Ok(task1(init, ops)? == if RUN_AND { x & y } else { x + y })
}

fn find_fault_bit_task_2<'a>(init: State<'a>, ops: &Ops<'a>) -> Option<usize> {
    (0..init.len() / 2).find(|&i| {
        let (x, y) = if RUN_AND {
            (1 << i, 1 << i)
        } else {
            ((1 << i) - 1, 1)
        };
        test_task_2_(init.clone(), x, y, ops.clone()).unwrap_or(false)
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
    ops: Ops<'a>,
    prev_faulty_bit: Option<usize>,
    deps: &HashMap<usize, Vec<&'a str>>,
    swaps: Vec<(&'a str, &'a str)>,
    multi_progress: &Mutex<MultiProgress>,
) -> Option<Vec<Vec<&'a str>>> {
    //let mut ops = ops.clone();
    if let Some(faulty_bit) = find_fault_bit_task_2(init.clone(), &ops) {
        if swaps.len() >= 4 || prev_faulty_bit.is_some_and(|prev_| faulty_bit <= prev_) {
            return None;
        }

        let deps_ = deps.get(&faulty_bit).unwrap();
        let deps_to_ignore: Vec<_> = (0..faulty_bit)
            .flat_map(|i| deps.get(&i).unwrap())
            .copied()
            .collect();
        let deps_is: Vec<_> = deps_
            .iter()
            .map(|v| ops.iter().position(|a| &a.3 == v).unwrap())
            .collect();

        let prefix = format!(
            "Faulty bit: {faulty_bit}, prev: {prev_faulty_bit:?}, n swaps: {}",
            swaps.len()
        );
        let style = ProgressStyle::default_bar()
            .template("{prefix} {wide_bar} {pos}/{len}")
            .unwrap();
        let progress = multi_progress.lock().unwrap().add(
            ProgressBar::new((ops.len() * deps_is.len()) as u64)
                .with_prefix(prefix)
                .with_style(style),
        );
        Some(
            deps_is
                .into_par_iter()
                .flat_map(|i| (0..ops.len()).par_bridge().map(move |j| (i, j)))
                //.into_iter()
                //.flat_map(|i| (0..ops.len()).map(move |j| (i, j)))
                .progress_with(progress.clone())
                .filter_map(|(mut i, mut j)| {
                    let mut ops = ops.clone();
                    if j == i || deps_to_ignore.contains(&ops[j].3) {
                        return None;
                    }

                    if j < i {
                        swap(&mut j, &mut i);
                    }

                    let (a, b) = ops.split_at_mut(j);
                    swap(&mut a[i].3, &mut b[0].3);

                    let mut swaps = swaps.clone();
                    swaps.push((a[i].3, b[0].3));
                    let init = init.clone();
                    let ret = task2_(init, ops, Some(faulty_bit), deps, swaps, multi_progress);

                    ret
                })
                .flatten()
                .collect(),
        )
    } else {
        let mut v: Vec<_> = swaps.iter().flat_map(|&(a, b)| [a, b]).collect();
        v.sort_unstable();
        //let _ = multi_progress
        //    .lock()
        //    .unwrap()
        //    .println(format!("Pushing {:?}", v.join(",")));
        Some(vec![v])
    }
}

fn task2<'a>(mut init: State<'a>, mut ops: Ops<'a>) -> Result<Vec<String>> {
    let deps = gen_dependencies(&ops);
    init.reserve(ops.len());
    let order = gen_dependencies(&ops).into_values().kmerge().collect_vec();
    ops.sort_unstable_by_key(|(_, _, _, c)| order.iter().position(|v| v == c).unwrap());

    Ok(task2_(
        init,
        ops,
        None,
        &deps,
        Default::default(),
        &Default::default(),
    )
    .unwrap()
    .iter()
    .filter(|v| v.len() == 8)
    .map(|v| v.join(","))
    .collect())
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let (init, ops) = parse_intput(s.trim())?;

    println!("Answer 1: {}", task1(init.clone(), ops.clone())?);
    println!("Answer 2: {:?}", task2(init, ops)?);

    Ok(())
}
