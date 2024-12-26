use anyhow::Result;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<i32>>, Pos, Pos)> {
    let mut start: Option<Pos> = None;
    let mut end: Option<Pos> = None;

    let mut f = |(x, y, ch)| match ch {
        '#' => -1,
        '.' => 0,
        'S' => {
            start = Some(Pos { x, y });
            0
        }
        'E' => {
            end = Some(Pos { x, y });
            0
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

fn dfs(map: &mut Vec<Vec<i32>>, Pos { x, y }: Pos, d: i32) {
    if map[y][x] <= d {
        return;
    }
    map[y][x] = d;

    for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let get_new_coord = |Pos { x, y }: Pos, (dx, dy): (isize, isize)| {
            let (x, y) = (x.checked_add_signed(dx)?, y.checked_add_signed(dy)?);
            map.get(y)?
                .get(x)
                .filter(|&&v| v != -1)
                .map(|&_| Pos { x, y })
        };
        if let Some(new_pos) = get_new_coord(Pos { x, y }, (dx, dy)) {
            dfs(map, new_pos, d + 1);
        }
    }
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?)?;

    println!("{:?}", map);
    Ok(())
}
