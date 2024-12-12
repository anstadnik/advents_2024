use anyhow::Result;
use itertools::Itertools;
use nalgebra::{try_convert, vector, Scalar, Vector2};
use std::fs::read_to_string;

type Pos = Vector2<usize>;
type IPos = Vector2<isize>;
type VVc = Vec<Vec<char>>;

#[derive(PartialEq, Eq, Hash)]
enum Side {
    Vertical(isize, isize, isize),
    Horizontal(isize, isize, isize),
}

use Side::*;

const OFFSETS: [IPos; 4] = [vector!(0, 1), vector!(0, -1), vector!(1, 0), vector!(-1, 0)];

trait IndexPoint<T: Scalar> {
    fn get_p(&self, p: Vector2<T>) -> Option<char>;
    fn get_mut_p(&mut self, p: Vector2<T>) -> Option<&mut char>;
}

impl IndexPoint<usize> for VVc {
    fn get_p(&self, p: Pos) -> Option<char> {
        self.get(p.y)?.get(p.x).copied()
    }

    fn get_mut_p(&mut self, p: Pos) -> Option<&mut char> {
        self.get_mut(p.y)?.get_mut(p.x)
    }
}

impl IndexPoint<isize> for VVc {
    fn get_p(&self, p: IPos) -> Option<char> {
        self.get_p(try_convert::<_, Pos>(p)?)
    }

    fn get_mut_p(&mut self, p: IPos) -> Option<&mut char> {
        self.get_mut_p(try_convert::<_, Pos>(p)?)
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn flood_fill(field: &mut VVc, pos: IPos, positions: &mut Vec<IPos>) {
    let c = field.get_p(pos).unwrap();
    *field.get_mut_p(pos).unwrap() = '.';
    positions.push(pos);
    for d in OFFSETS {
        if field.get_p(pos + d) == Some(c) {
            flood_fill(field, pos + d, positions)
        }
    }
}

fn gen_islands(mut field: Vec<Vec<char>>) -> Vec<Vec<IPos>> {
    let (n, m) = (field.len() as isize, field[0].len() as isize);
    let mut islands = vec![];
    for pos in (0..n).cartesian_product(0..m).map(|(y, x)| vector!(x, y)) {
        if field.get_p(pos) != Some('.') {
            let mut positions = vec![];
            flood_fill(&mut field, pos, &mut positions);
            islands.push(positions);
        }
    }
    islands
}

fn gen_sides<'a>(field: &'a VVc, island: &'a [IPos]) -> impl Iterator<Item = Side> + use<'a> {
    island.iter().flat_map(move |&p| {
        let c = field.get_p(p);
        OFFSETS
            .iter()
            .filter(move |&&d| field.get_p(p + d) != c)
            .map(move |&d| match (d.y, d.x) {
                (0, 1) => Vertical(p.x, p.x + 1, p.y),
                (0, -1) => Vertical(p.x, p.x - 1, p.y),
                (1, 0) => Horizontal(p.y, p.y + 1, p.x),
                (-1, 0) => Horizontal(p.y, p.y - 1, p.x),
                _ => unreachable!(),
            })
    })
}

fn task1(field: &VVc) -> usize {
    gen_islands(field.clone())
        .iter()
        .map(|i| i.len() * gen_sides(field, i).count())
        .sum()
}

fn partition_sides(mut island: Vec<Side>) -> usize {
    let mut count = 0;

    while let Some(s) = island.pop() {
        count += 1;

        match s {
            Vertical(x1, x2, y) => {
                let mut f = |y_| {
                    let pred = |s| s == &Vertical(x1, x2, y_);
                    island.iter().position(pred).map(|p| island.swap_remove(p))
                };
                (y + 1..).map_while(&mut f).for_each(drop);
                (0..y).rev().map_while(&mut f).for_each(drop);
            }
            Horizontal(y1, y2, x) => {
                let mut f = |x_| {
                    let pred = |s| s == &Horizontal(y1, y2, x_);
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
    gen_islands(field.clone())
        .iter()
        .map(|i| i.len() * partition_sides(gen_sides(field, i).collect()))
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
