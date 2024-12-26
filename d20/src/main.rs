use anyhow::Result;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<i32>>, Pos, Pos)> {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;

    let mut f = |(x, y, ch)| match ch {
        '#' => -1,
        '.' => 0,
        'S' => {
            start = Some(Pos { x, y });
            0
        }
        'E' => {
            end = Some(Pos { x, y });
            0
        }
        _ => unreachable!(),
    };
    let map: Vec<Vec<i32>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| l.chars().enumerate().map(|(x, ch)| f((x, y, ch))).collect())
        .collect();

    start.zip(end).map_or(
        Err(anyhow::anyhow!("Start or end position not found")),
        |(start, end)| Ok((map, start, end)),
    )
}

fn dfs(map: &mut Vec<Vec<i32>>, Pos { x, y }: Pos, d: i32) {
    if map[y][x] <= d {
        return;
    }
    map[y][x] = d;

    for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let get_new_coord = |Pos { x, y }: Pos, (dx, dy): (isize, isize)| {
            let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
            map.get(y)?
                .get(x)
                .filter(|&&v| v != -1)
                .map(|&_| Pos { x, y })
        };
        if let Some(new_pos) = get_new_coord(Pos { x, y }, (dx, dy)) {
            dfs(map, new_pos, d + 1);
        }
    }
}

fn task1(map: Vec<Vec<i32>>, start: Pos, end: Pos) -> Vec<(Pos, Pos, i32)> {
    //let mut results = Vec::new();
    let mut default_map = map.clone();
    dfs(&mut default_map, start, 0);
    let default_distance = default_map[end.y][end.x];

    //(0..map.len())
    //    .cartesian_product(0..map[0].len())
    //.par_bridge()
    (0..map.len())
        .into_par_iter()
        .flat_map(|y| (0..map[0].len()).into_par_iter().map(move |x| (y, x)))
        //.progress()
        .filter(|&(y, x)| map[y][x] == -1)
        .map(|(y, x)| {
            let below = map
                .get(y + 1)
                .and_then(|row| row.get(x))
                .is_some_and(|&v| v == -1);
            let right = map
                .get(y)
                .and_then(|row| row.get(x + 1))
                .is_some_and(|&v| v == -1);
            match (below, right) {
                (true, true) => [
                    Some((Pos { x, y }, Pos { x, y: y + 1 })),
                    Some((Pos { x, y }, Pos { x: x + 1, y })),
                ],
                (true, false) => [Some((Pos { x, y }, Pos { x, y: y + 1 })), None],
                (false, true) => [Some((Pos { x, y }, Pos { x: x + 1, y })), None],
                (false, false) => [Some((Pos { x, y }, Pos { x, y })), None],
            }
        })
        .flatten()
        .flatten()
        .map(|(s, e)| {
            let mut new_map = map.clone();
            new_map[s.y][s.x] = 0;
            new_map[e.y][e.x] = 0;

            dfs(&mut new_map, start, 0);
            (s, e, default_distance - new_map[end.y][end.x])
        })
        .collect()
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {:?}", task1(map.clone(), start, end));
    Ok(())
}
