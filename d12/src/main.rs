mod pos;

use anyhow::Result;
use glam::ivec2;
use itertools::Itertools;
use pos::{IPos, IndexPoint, VVc};
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Hash)]
enum Side {
    Vertical(i32, i32, i32),
    Horizontal(i32, i32, i32),
}

use Side::*;

const OFFSETS: [IPos; 4] = [ivec2(0, 1), ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)];

fn parse_input(input: &str) -> VVc {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn flood_fill(field: &mut VVc, pos: IPos, positions: &mut Vec<IPos>) {
    let c = field.get_p(pos).unwrap();
    *field.get_mut_p(pos).unwrap() = '.';
    positions.push(pos);

    for d in &OFFSETS {
        if field.get_p(pos + d) == Some(c) {
            flood_fill(field, pos + d, positions)
        }
    }
}

fn islands(mut field: VVc) -> Vec<Vec<IPos>> {
    let (n, m) = (field.len() as i32, field[0].len() as i32);
    let mut islands = vec![];

    for pos in (0..n).cartesian_product(0..m).map(|(y, x)| ivec2(x, y)) {
        if field.get_p(pos) != Some('.') {
            let mut positions = vec![];
            flood_fill(&mut field, pos, &mut positions);
            islands.push(positions);
        }
    }
    islands
}

fn sides<'a>(field: &'a VVc, island: &'a [IPos]) -> impl Iterator<Item = Side> + 'a {
    island.iter().flat_map(move |&p| {
        let c = field.get_p(p);
        OFFSETS
            .iter()
            .filter(move |&d| field.get_p(p + d) != c)
            .map(move |&d| match d.y == 0 {
                true => Vertical(p.x, p.x + d.x, p.y),
                false => Horizontal(p.y, p.y + d.y, p.x),
            })
    })
}

fn task1(field: &VVc) -> usize {
    islands(field.clone())
        .iter()
        .map(|island| island.len() * sides(field, island).count())
        .sum()
}

fn partition_sides(mut island: Vec<Side>) -> usize {
    let mut count = 0;

    while let Some(side) = island.pop() {
        count += 1;

        match side {
            Vertical(x1, x2, y) => {
                let mut f = |y_offset| {
                    let pred = |s| s == &Vertical(x1, x2, y_offset);
                    island.iter().position(pred).map(|p| island.swap_remove(p))
                };
                (y + 1..).map_while(&mut f).for_each(drop);
                (0..y).rev().map_while(&mut f).for_each(drop);
            }
            Horizontal(y1, y2, x) => {
                let mut f = |x_offset| {
                    let pred = |s| s == &Horizontal(y1, y2, x_offset);
                    island.iter().position(pred).map(|p| island.swap_remove(p))
                };
                (x + 1..).map_while(&mut f).for_each(drop);
                (0..x).rev().map_while(&mut f).for_each(drop);
            }
        }
    }
    count
}

fn task2(field: &VVc) -> usize {
    islands(field.clone())
        .iter()
        .map(|island| island.len() * partition_sides(sides(field, island).collect()))
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

    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_example1() -> Result<()> {
        let input = r"AAAA
BBCD
BBCC
EEEC";

        let input = parse_input(input);
        assert_eq!(task1(&input), 140);
        assert_eq!(task2(&input), 80);

        Ok(())
    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        let input = parse_input(input);
        assert_eq!(task1(&input), 772);
        assert_eq!(task2(&input), 436);

        Ok(())
    }

    #[test]
    fn test_example3() -> Result<()> {
        let input = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let input = parse_input(input);
        assert_eq!(task2(&input), 368);

        Ok(())
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let input = parse_input(input);
        assert_eq!(task1(&input), 1930);
        assert_eq!(task2(&input), 1206);

        Ok(())
    }
}
