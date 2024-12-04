use anyhow::Result;
use itertools::Itertools;
use std::{fs::read_to_string, iter::successors};

fn parse_data(s: &str) -> Result<Vec<Vec<char>>> {
    s.lines().map(|l| Ok(l.chars().collect())).collect()
}

fn count_xmas(it: impl Iterator<Item = char> + Clone) -> usize {
    let p = |&(x, m, a, s): &_| {
        (x, m, a, s) == ('X', 'M', 'A', 'S') || (x, m, a, s) == ('S', 'A', 'M', 'X')
    };
    it.tuple_windows().filter(p).count()
}

fn task1(a: &[Vec<char>]) -> usize {
    let (n, m) = (a.len(), a[0].len());
    let lines = a.iter().map(|l| l.iter().copied());
    let cols = (0..m).map(|j| (0..n).map(move |i| a[i][j]));
    let diagonals = (0..n + m - 1).map(|k| {
        let (r, c) = if k < n { (k, 0) } else { (0, k - n + 1) };
        successors(Some((r, c)), |&(i, j)| Some((i + 1, j + 1)))
            .map_while(|(i, j)| a.get(i)?.get(j).copied())
    });
    let diagonals2 = (0..n + m - 1).map(|k| {
        let (r, c) = if k < n { (k, 0) } else { (n - 1, k - n + 1) };
        successors(Some((r, c)), |&(i, j)| Some((i.checked_sub(1)?, j + 1)))
            .map_while(|(i, j)| a.get(i)?.get(j).copied())
    });
    lines.map(count_xmas).sum::<usize>()
        + cols.map(count_xmas).sum::<usize>()
        + diagonals.map(count_xmas).sum::<usize>()
        + diagonals2.map(count_xmas).sum::<usize>()
}

fn task2(a: &[Vec<char>]) -> usize {
    let map = |c| match c {
        'M' => Some('S'),
        'S' => Some('M'),
        _ => None,
    };
    let f = |r: usize, c: usize| {
        let f = |dr: usize, dc| a[r + dr][c + dc];
        Some((f(1, 1) == 'A' && map(f(0, 0))? == f(2, 2) && map(f(2, 0))? == f(0, 2)) as usize)
    };
    (0..a.len() - 2)
        .flat_map(|r| (0..a[0].len() - 2).filter_map(move |c| f(r, c)))
        .sum()
}

fn main() -> Result<()> {
    let data = parse_data(&read_to_string("input.txt")?)?;

    println!("Answer1: {}", task1(&data));
    println!("Answer1: {}", task2(&data));

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
    fn test_example() -> Result<()> {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let data = parse_data(input)?;

        assert_eq!(task1(&data), 18);
        assert_eq!(task2(&data), 9);

        Ok(())
    }
}
