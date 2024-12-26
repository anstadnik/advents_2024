mod enums;
use anyhow::Result;
use enums::{Dir::*, Pos};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;

fn parse_input(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut find = |c| {
        let pos = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c_)| (x, y, c_)))
            .find_map(|(x, y, c_)| (c_ == c).then_some(Pos { x, y, d: Right }))
            .unwrap();
        map[pos.y][pos.x] = '.';
        pos
    };
    let (start, end) = (find('S'), find('E'));
    (map, start, end)
}

type V = Vec<Pos>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathCost(V, u32);

impl PartialOrd for PathCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed
        other.1.cmp(&self.1)
    }
}

fn get_paths(map: &[Vec<char>], start: Pos, end: Pos) -> Vec<(Vec<Pos>, u32)> {
    let mut mem = HashMap::new();
    let mut queue: BinaryHeap<PathCost> = [PathCost([start].into(), 0)].into();
    let mut rez = Vec::new();
    let mut min_cost = None;

    while let Some(PathCost(path, cost)) = queue.pop() {
        if min_cost.is_some_and(|min| cost > min) {
            break;
        }
        let p = *path.last().unwrap();
        if let Some(&cost_) = mem.get(&p) {
            if cost > cost_ {
                continue;
            }
        }
        if map
            .get(p.y)
            .is_none_or(|row| row.get(p.x).is_none_or(|&c| c == '#'))
        {
            continue;
        }
        mem.insert(p, cost);
        if p.y == end.y && p.x == end.x {
            rez.push((path.clone(), cost));
            if min_cost.is_none() {
                min_cost = Some(cost);
            }
        }
        let turn = p.turn().into_iter().map(|p| (p, cost + 1000));
        let step = [p.step()].into_iter().filter_map(|p| Some((p?, cost + 1)));
        queue.extend(
            step.chain(turn)
                .map(|(pos, cost)| PathCost(path.iter().copied().chain([pos]).collect(), cost)),
        );
    }

    rez
}

fn task1(paths: &[(Vec<Pos>, u32)]) -> Option<u32> {
    paths.iter().map(|(_, c)| *c).min()
}

fn task2(paths: &[(Vec<Pos>, u32)]) -> Option<usize> {
    let min_cost = *paths.iter().map(|(_, cost)| cost).min()?;
    Some(
        paths
            .iter()
            .filter(|(_, cost)| *cost == min_cost)
            .flat_map(|(path, _)| path.iter().map(|pos| (pos.x, pos.y)))
            .collect::<HashSet<_>>()
            .len(),
    )
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?);
    let paths = get_paths(&map, start, end);

    println!(
        "Answer 1: {}",
        task1(&paths).ok_or(anyhow::anyhow!("No path found"))?
    );
    println!(
        "Answer 2: {}",
        task2(&paths).ok_or(anyhow::anyhow!("No path found"))?
    );

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
    fn test_example1() -> Result<()> {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        let (map, start, end) = parse_input(input);
        let paths = get_paths(&map, start, end);
        assert_eq!(task1(&paths).ok_or(anyhow::anyhow!("No path found"))?, 7036);

        assert_eq!(task2(&paths).ok_or(anyhow::anyhow!("No path found"))?, 45,);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        let (map, start, end) = parse_input(input);
        let paths = get_paths(&map, start, end);

        assert_eq!(
            task1(&paths).ok_or(anyhow::anyhow!("No path found"))?,
            11048
        );
        assert_eq!(task2(&paths).ok_or(anyhow::anyhow!("No path found"))?, 64);

        Ok(())
    }
}
