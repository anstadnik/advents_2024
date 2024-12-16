use anyhow::{anyhow, Result};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use glam::IVec2;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::{self, Write};
use std::iter::once;
use std::time::Duration;
use std::{fs::read_to_string, str::FromStr};
use winnow::{ascii::dec_int, seq, PResult, Parser};

type N = i32;
type V = IVec2;
type Field = Vec<Vec<N>>;
const USE_CROSSTERM: bool = false;

#[derive(Debug, Clone)]
struct Robot {
    p: V,
    v: V,
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
        let (p, v) = (V::new(p1, p2), V::new(v1, v2));
        Ok(Self { p, v })
    }
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(Robot::from_str).collect()
}

fn print_field(m: &[Vec<N>]) -> impl Iterator<Item = char> + use<'_> {
    m.iter().flat_map(|row| {
        row.iter()
            .map(|&v| match v {
                0 => '.',
                _ => char::from_digit(v as _, 10).unwrap(),
            })
            .chain(once('\n'))
    })
}

fn gen_positions(robots: &[Robot], size: V, n: N) -> Vec<V> {
    robots
        .iter()
        .map(|Robot { p, v }| (*p + *v * n).rem_euclid(size))
        .collect()
}

fn task1(robots: &[Robot], size: [N; 2], i: N) -> Result<(Field, usize)> {
    let mut field = vec![vec![0; size[0] as usize]; size[1] as usize];
    let size = V::new(size[0], size[1]);
    let positions = gen_positions(robots, size, i);

    let (a, b, c, d) = positions
        .iter()
        .inspect(|p| field[p.y as usize][p.x as usize] += 1)
        .fold((0, 0, 0, 0), |(a, b, c, d), p| {
            match (p.y.cmp(&(size.y / 2)), p.x.cmp(&(size.x / 2))) {
                (Equal, _) | (_, Equal) => (a, b, c, d),
                (Less, Less) => (a + 1, b, c, d),
                (Less, Greater) => (a, b + 1, c, d),
                (Greater, Less) => (a, b, c + 1, d),
                (Greater, Greater) => (a, b, c, d + 1),
            }
        });

    Ok((field, a * b * c * d))
}

fn task2(robots: &[Robot], size: [N; 2]) -> Result<N> {
    if USE_CROSSTERM {
        let _ = enable_raw_mode();
    }
    let (mut i, mut run) = (0, true);

    loop {
        let (field, _) = task1(robots, size, i)?;
        let s = print_field(&field);

        if USE_CROSSTERM {
            write!(
                io::stdout(),
                "{}",
                s.collect::<String>().replace("\n", "\r\n")
            )?;
            write!(io::stdout(), "{i}")?;
            io::stdout().flush()?;
            if poll(Duration::from_millis(200))? {
                if let Event::Key(KeyEvent { code, .. }) = read()? {
                    match code {
                        KeyCode::Right => i += 1,
                        KeyCode::Left => i -= 1,
                        KeyCode::Enter => {
                            let _ = disable_raw_mode();
                            return Ok(i);
                        }
                        KeyCode::Char(' ') => run = !run,
                        _ => (),
                    }
                }
            } else if run {
                i += 1;
            }
        } else {
            let s: String = s.collect();
            let max = s
                .lines()
                .map(|l| l.chars().filter(|c| *c != '.').count())
                .max()
                .unwrap();
            if max > 31 {
                println!("{s}");
                return Ok(i);
            }
            i += 1;
        }
    }
}

fn main() -> Result<()> {
    let robots = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {}", task1(&robots, [101, 103], 100)?.1);
    println!("Answer 2: {}", task2(&robots, [101, 103])?);

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
        assert_eq!(task1(&input, [11, 7], 100)?.1, 12);

        //assert_eq!(task2(&input), 81);

        Ok(())
    }
}
