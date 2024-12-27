use anyhow::Result;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, fs::read_to_string, iter::successors};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<i32>>, Pos, Pos)> {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;

    let mut f = |(x, y, ch)| match ch {
        '#' => -1,
        '.' => i32::MAX,
        'S' => {
            start = Some(Pos { x, y });
            i32::MAX
        }
        'E' => {
            end = Some(Pos { x, y });
            i32::MAX
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

    for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let get_new_coord = |Pos { x, y }: Pos, (dx, dy): (isize, isize)| {
            let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
            (*map.get(y)?.get(x)? != -1).then_some(Pos { x, y })
        };

        if let Some(new_pos) = get_new_coord(Pos { x, y }, (dx, dy)) {
            dfs(map, new_pos, d + 1);
        }
    }
}

fn print_map(map: &[Vec<i32>]) {
    for row in map {
        for &v in row {
            print!(
                "{}",
                match v {
                    -1 => " ## ".to_string(),
                    i32::MAX => " .. ".to_string(),
                    //_ => v.to_string().chars().next().unwrap(),
                    _ => format!(" {: ^2} ", &v.to_string()),
                }
            );
        }
        println!();
    }
}

fn get_correct_track(map: &[Vec<i32>], end: Pos) -> Vec<Pos> {
    let mut ret: Vec<_> = successors(Some(end), |&Pos { x, y }| {
        let v = map[y][x];
        if v == 0 {
            return None;
        }
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .find_map(|(dx, dy)| {
                let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
                (*map.get(y)?.get(x)? == v - 1).then_some(Pos { x, y })
            })
    })
    .collect();
    ret.reverse();
    ret
}

fn gen_cheats(map: &[Vec<i32>], start: Pos, end: Pos) -> Vec<(Pos, Pos, i32)> {
    let mut default_map = map.to_vec();
    dfs(&mut default_map, start, 0);
    let default_distance = default_map[end.y][end.x];
    let track = get_correct_track(&default_map, end);

    (0..map.len())
        .into_par_iter()
        .flat_map(|y| (0..map[0].len()).into_par_iter().map(move |x| (x, y)))
        .progress_count((map.len() * map[0].len()) as u64)
        .flat_map(|(x, y)| {
            [
                (Pos { x, y }, Pos { x, y: y + 1 }),
                (Pos { x, y }, Pos { x: x + 1, y }),
            ]
        })
        .filter_map(|cheat @ (Pos { x, y }, e)| {
            (*map.get(y)?.get(x)? == -1 && track.contains(&e)).then_some(cheat)
        })
        //.filter(|&(x, y)| map[y][x] == -1)
        //.flat_map(|(x, y)| {
        //    let below = map.get(y + 1).and_then(|row| row.get(x)).is_some();
        //    //.is_some_and(|&v| v == -1);
        //    let right = map.get(y).and_then(|row| row.get(x + 1)).is_some();
        //    //.is_some_and(|&v| v == -1);
        //    match (below, right) {
        //        (true, true) => [
        //            //Some((Pos { x, y }, Pos { x, y })),
        //            Some((Pos { x, y }, Pos { x, y: y + 1 })),
        //            Some((Pos { x, y }, Pos { x: x + 1, y })),
        //        ],
        //        (true, false) => [
        //            //Some((Pos { x, y }, Pos { x, y })),
        //            Some((Pos { x, y }, Pos { x, y: y + 1 })),
        //            None,
        //        ],
        //        (false, true) => [
        //            //Some((Pos { x, y }, Pos { x, y })),
        //            Some((Pos { x, y }, Pos { x: x + 1, y })),
        //            None,
        //        ],
        //        //(false, false) => [Some((Pos { x, y }, Pos { x, y })), None, None],
        //        (false, false) => [None, None],
        //    }
        //})
        //.flatten()
        //[
        //    //(Pos { x: 8, y: 1 }, Pos { x: 9, y: 1 }),
        //    //(Pos { x: 10, y: 7 }, Pos { x: 11, y: 7 }),
        //    //(Pos { x: 10, y: 7 }, Pos { x: 11, y: 7 }),
        //].into_iter()
        .map(|(s, e)| {
            let mut new_map = map.to_vec();
            new_map[s.y][s.x] = i32::MAX;
            new_map[e.y][e.x] = i32::MAX;

            dfs(&mut new_map, start, 0);
            get_correct_track(&new_map, end)
            //(s, e, default_distance - new_map[end.y][end.x])
        })
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|v| (start, end, (track.len() - v.len()) as _))
        .filter(|(_, _, d)| *d > 0)
        .collect()
}

fn task1(map: &[Vec<i32>], start: Pos, end: Pos) -> usize {
    gen_cheats(map, start, end)
        .iter()
        .filter(|(_, _, d)| *d >= 100)
        .count()
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {:?}", task1(&map, start, end));
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_task1() -> Result<()> {
        let input = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

        let answer: HashMap<_, _> = [
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]
        .into();

        let (map, start, end) = parse_input(input)?;
        let cheats = gen_cheats(&map, start, end);
        let my_answer = cheats
            .iter()
            .map(|(_, _, d)| *d)
            .fold(HashMap::new(), |mut acc, d| {
                acc.entry(d).and_modify(|v| *v += 1).or_insert(1);
                acc
            });

        //assert_eq!(my_answer, answer);
        let mut answer: Vec<_> = answer.iter().collect();
        let mut my_answer: Vec<_> = my_answer.iter().collect();
        println!(
            "my_ans: {}, ans: {}",
            my_answer.iter().map(|(_, &v)| v).sum::<usize>(),
            answer.iter().map(|(_, &v)| v).sum::<usize>()
        );
        answer.sort();
        my_answer.sort();
        assert_eq!(my_answer, answer);
        Ok(())
    }
}
