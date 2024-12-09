use anyhow::Result;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn task1<const TASK1: bool>(input: &[Vec<char>]) -> usize {
    let locations = input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &c)| ((y, x), c)))
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (pos, c)| {
            if c != '.' {
                acc.entry(c).or_default().push(pos)
            }
            acc
        });

    let pred = |(y, x): (Option<usize>, Option<usize>)| input.get(y?)?.get(x?);
    locations
        .into_iter()
        .flat_map(|(_, v)| v.into_iter().combinations(2))
        .flat_map(|p| -> Box<dyn Iterator<Item = _>> {
            let (a, b) = (p[0], p[1]);
            let d = ((b.0 as isize - a.0 as isize), (b.1 as isize - a.1 as isize));
            if TASK1 {
                let a_ = (a.0.checked_add_signed(-d.0), a.1.checked_add_signed(-d.1));
                let b_ = (b.0.checked_add_signed(d.0), b.1.checked_add_signed(d.1));
                Box::new(
                    [pred(a_).and(Some(a_)), pred(b_).and(Some(b_))]
                        .into_iter()
                        .flatten(),
                )
            } else {
                let a_it = (0..).map_while(move |n| {
                    let a_ = (
                        a.0.checked_add_signed(-d.0 * n),
                        a.1.checked_add_signed(-d.1 * n),
                    );
                    pred(a_).and(Some(a_))
                });
                let b_it = (0..).map_while(move |n| {
                    //let b_ = (b.0 as i32 + d.0 * n, b.1 as i32 + d.1 * n);
                    let b_ = (
                        b.0.checked_add_signed(d.0 * n),
                        b.1.checked_add_signed(d.1 * n),
                    );
                    pred(b_).and(Some(b_))
                });
                Box::new(a_it.chain(b_it))
            }
        })
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> Result<()> {
    let input = parse_input(&read_to_string("input.txt")?);

    println!("Answer 1: {}", task1::<true>(&input));
    println!("Answer 2: {}", task1::<false>(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test_main() -> Result<()> {
    //    main()
    //}

    //#[test]
    //    fn test_example1() -> Result<()> {
    //        let input = r"............
    //........0...
    //.....0......
    //.......0....
    //....0.......
    //......A.....
    //............
    //............
    //........A...
    //.........A..
    //............
    //............";
    //
    //        let input = parse_input(input);
    //        assert_eq!(task1(&input), 14);
    //        //assert_eq!(task2(&input), 11387);
    //        Ok(())
    //    }

    #[test]
    fn test_example2() -> Result<()> {
        let input = r"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

        let input = parse_input(input);
        assert_eq!(task1::<false>(&input), 2);
        assert_eq!(task1::<true>(&input), 34);

        Ok(())
    }
}
