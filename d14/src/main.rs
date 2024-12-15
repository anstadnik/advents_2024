use anyhow::{anyhow, Result};
use ndarray::{arr1, Array1, Array2};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::{fs::read_to_string, str::FromStr};
use winnow::{ascii::dec_int, seq, PResult, Parser};

type N = i32;

#[derive(Debug, Clone)]
struct Robot {
    p: Array1<N>,
    v: Array1<N>,
}

fn parse_robot(s: &mut &str) -> PResult<(N, N, N, N)> {
    seq!(
        _: "p=", dec_int, _: ",", dec_int, _: " v=", dec_int, _: ",", dec_int
    )
    .parse_next(s)
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (p1, p2, v1, v2) = parse_robot.parse(s).map_err(|e| anyhow!("{e}"))?;
        let (p, v) = (arr1(&[p1, p2]), arr1(&[v1, v2]));
        Ok(Self { p, v })
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(Robot::from_str).collect()
}

fn print_field(m: &Array2<N>) {
    for row in m.rows() {
        for v in row {
            if *v == 0 {
                print!(".");
            } else {
                print!("{v}");
            }
        }
        println!();
    }
}

fn task1(robots: &[Robot], size: [N; 2]) -> Result<usize> {
    let mut field: Array2<N> = Array2::zeros((size[1] as usize, size[0] as usize));
    let size = arr1(&size);
    let (a, b, c, d) = robots
        .iter()
        //.inspect(|Robot { p, v }| {
        //    println!(
        //        "p: {}, v: {}, p+v*100: {}, (p+v*100)%s: {}",
        //        p,
        //        v,
        //        p + v * 100,
        //        (p + v * 100) % &size
        //    )
        //})
        .map(|Robot { p, v }| (p + v * 100) % &size)
        //.inspect(|p| println!("{}, {}", p[0], p[1]))
        .map(|p| {
            (p < size);
            let (x, y) = (p[0], p[1]);
            (size[0] * (x < 0) as N + x, size[1] * (y < 0) as N + y)
        })
        //.inspect(|(y, x)| println!("{y}, {x}"))
        .inspect(|&(x, y)| {
            //print_field(&field);
            field[[y as usize, x as usize]] += 1
        })
        .fold((0, 0, 0, 0), |(a, b, c, d), (x, y)| {
            match (y.cmp(&(&size[1] / 2)), x.cmp(&(&size[0] / 2))) {
                (Equal, _) | (_, Equal) => (a, b, c, d),
                (Less, Less) => (a + 1, b, c, d),
                (Less, Greater) => (a, b + 1, c, d),
                (Greater, Less) => (a, b, c + 1, d),
                (Greater, Greater) => (a, b, c, d + 1),
            }
        });
    println!("{a}, {b}, {c}, {d}");
    print_field(&field);
    Ok(a * b * c * d)
}

fn main() -> Result<()> {
    let robots = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {}", task1(&robots, [101, 103])?);
    //println!("Answer 2: {}", task2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_example1() -> Result<()> {
        let input = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        let input = parse_input(input)?;
        assert_eq!(task1(&input, [11, 7])?, 12);

        //assert_eq!(task2(&input), 81);

        Ok(())
    }
}
