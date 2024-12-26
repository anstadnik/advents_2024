use anyhow::{anyhow, Result};
use std::fs::read_to_string;
use winnow::combinator::{separated, separated_pair};
use winnow::{ascii::dec_uint, PResult, Parser};

type Pos = (usize, usize);
type Input = Vec<Pos>;

fn parse_input(input: &mut &str) -> PResult<Input> {
    separated(1.., separated_pair(dec_uint, ',', dec_uint), '\n').parse_next(input)
}

fn dfs(map: &mut Vec<Vec<i32>>, (x, y): Pos, d: i32) {
    if map[y][x] <= d {
        return;
    }
    map[y][x] = d;

    for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let get_new_coord = |(x, y): Pos, (dx, dy)| {
            let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
            map.get(y)?.get(x).filter(|&&v| v != -1).map(|&_| (x, y))
        };
        if let Some((x, y)) = get_new_coord((x, y), (dx, dy)) {
            dfs(map, (x, y), d + 1)
        }
    }
}

fn task1(input: &Input, size: (usize, usize), n: usize) -> i32 {
    let mut map = vec![vec![i32::MAX; size.1 + 1]; size.0 + 1];
    input.iter().take(n).for_each(|&(x, y)| map[y][x] = -1);

    dfs(&mut map, (0, 0), 0);
    map[size.0][size.1]
}

fn task2(input: &Input, size: (usize, usize)) -> Pos {
    input[(1..input.len())
        .collect::<Vec<_>>()
        .partition_point(|&n| task1(input, size, n) != i32::MAX)]
}

fn main() -> Result<()> {
    let input = parse_input
        .parse(read_to_string("input.txt")?.trim())
        .map_err(|e| anyhow!("{e}"))?;

    println!("Answer 1: {}", task1(&input, (70, 70), 1024));
    println!("Answer 2: {:?}", task2(&input, (70, 70)));

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
    fn test_task() {
        let input_str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let mut input = input_str;
        let parsed_input = parse_input(&mut input).unwrap();
        let size = (6, 6);
        let n = 12;
        assert_eq!(task1(&parsed_input, size, n), 22);
        assert_eq!(task2(&parsed_input, size), (6, 1));
    }
}
