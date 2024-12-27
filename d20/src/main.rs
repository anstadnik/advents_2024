use anyhow::Result;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::{fs::read_to_string, iter::successors};

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

fn get_correct_track(map: &[Vec<i32>], end: Pos) -> Vec<Pos> {
    let mut ret: Vec<_> = successors(Some(end), |&Pos { x, y }| {
        let v = map[y][x];
        if v == 0 {
            return None;
        }
        let f = |(dx, dy)| {
            let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
            (*map.get(y)?.get(x)? == v - 1).then_some(Pos { x, y })
        };
        [(1, 0), (0, 1), (-1, 0), (0, -1)].into_iter().find_map(f)
    })
    .collect();
    ret.reverse();
    ret
}

fn gen_cheats(map: &[Vec<i32>], start: Pos, end: Pos, min_dist: usize) -> Vec<usize> {
    let mut default_map = map.to_vec();
    dfs(&mut default_map, start, 0);
    let track = get_correct_track(&default_map, end);

    track
        .par_iter()
        .enumerate()
        .flat_map(|(i, &s)| {
            track
                .par_iter()
                .enumerate()
                .skip(i + 4)
                .map(move |(j, &e)| (i, j, s, e))
        })
        .filter_map(|(i, j, Pos { x: sx, y: sy }, Pos { x: ex, y: ey })| {
            let dist = sx.abs_diff(ex) + sy.abs_diff(ey);
            assert!(dist > 1);
            let diff = (j - i) - dist;
            (dist <= min_dist.max(2) && diff > 0).then_some(diff)
        })
        .collect()
}

fn task(map: &[Vec<i32>], start: Pos, end: Pos, n_sec: usize) -> usize {
    gen_cheats(map, start, end, n_sec)
        .iter()
        .filter(|d| **d >= 100)
        .count()
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {:?}", task(&map, start, end, 1));
    println!("Answer 2: {:?}", task(&map, start, end, 20));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

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
        let cheats = gen_cheats(&map, start, end, 1);
        let my_answer = cheats.iter().copied().fold(HashMap::new(), |mut acc, d| {
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

    #[test]
    fn test_task2() -> Result<()> {
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
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]
        .into();

        let (map, start, end) = parse_input(input)?;
        let cheats = gen_cheats(&map, start, end, 20);
        let my_answer =
            cheats
                .iter()
                .copied()
                .filter(|&d| d >= 50)
                .fold(HashMap::new(), |mut acc, d| {
                    acc.entry(d).and_modify(|v| *v += 1).or_insert(1);
                    acc
                });

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
