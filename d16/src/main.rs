mod enums;
use anyhow::Result;
use enums::{Dir::*, Pos};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::read_to_string;
use std::rc::Rc;

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

/// A node in our singly-linked path list
#[derive(Debug, PartialEq, Eq)]
struct Node {
    parent: Option<Rc<Node>>,
    pos: Pos,
}

/// Collects all positions from this node up to the start by following `.parent` links
fn collect_path(mut node: &Node) -> Vec<Pos> {
    let mut rev = Vec::new();
    // Walk up until parent is None
    loop {
        rev.push(node.pos);
        if let Some(ref parent) = node.parent {
            node = parent.as_ref();
        } else {
            break;
        }
    }
    rev.reverse();
    rev
}

/// Instead of storing `Vec<Pos>`, we store the head node (an Rc<Node>) and a cost
#[derive(Debug, Clone, PartialEq, Eq)]
struct PathCost(Rc<Node>, u32);

impl PartialOrd for PathCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We want a min-heap by cost, so we reverse
        other.1.cmp(&self.1)
    }
}

fn get_paths(map: &[Vec<char>], start: Pos, end: Pos) -> Vec<(Vec<Pos>, u32)> {
    let start_node = Rc::new(Node {
        parent: None,
        pos: start,
    });

    let mut mem = HashMap::new();
    // Instead of pushing `PathCost(vec![start], 0)`, push a single-node path
    let mut queue: BinaryHeap<PathCost> = [PathCost(start_node.clone(), 0)].into();
    let mut rez = Vec::new();
    let mut min_cost = None;

    while let Some(PathCost(node, cost)) = queue.pop() {
        // If we've found a minimum cost, prune anything bigger
        if min_cost.is_some_and(|min| cost > min) {
            break;
        }

        let p = node.pos;
        // Standard BFS / D'ijkstra skipping if cost is worse
        if let Some(&old_cost) = mem.get(&p) {
            if cost > old_cost {
                continue;
            }
        }
        // Check boundaries, walls, etc.
        if map
            .get(p.y)
            .is_none_or(|row| row.get(p.x).is_none_or(|&c| c == '#'))
        {
            continue;
        }
        mem.insert(p, cost);

        // If this is end, add to results
        if p.y == end.y && p.x == end.x {
            // Convert the singly-linked list into Vec<Pos>
            let path_vec = collect_path(&node);
            rez.push((path_vec, cost));
            // Track the minimum cost
            if min_cost.is_none() {
                min_cost = Some(cost);
            }
        }

        // Otherwise, explore next steps
        //   - turning yields cost+1000
        //   - stepping forward yields cost+1
        let turn = p.turn().into_iter().map(|new_pos| (new_pos, cost + 1000));
        let step = [p.step()]
            .into_iter()
            .filter_map(|o| o.map(|np| (np, cost + 1)));

        for (pos, new_cost) in turn.chain(step) {
            // Extend the singly-linked list with an O(1) new node
            let new_node = Rc::new(Node {
                parent: Some(node.clone()),
                pos,
            });
            queue.push(PathCost(new_node, new_cost));
        }
    }

    rez
}

/// The cheapest cost among the paths
fn task1(paths: &[(Vec<Pos>, u32)]) -> Option<u32> {
    paths.iter().map(|(_, c)| *c).min()
}

/// The count of unique coordinates in *all* minimal-cost paths
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
