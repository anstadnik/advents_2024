use anyhow::Result;
use itertools::Itertools;
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

fn task1(map: Vec<Vec<i32>>, start: Pos, end: Pos) -> Vec<(Pos, Pos, i32)> {
    let mut results = Vec::new();
    let mut default_map = map.clone();
    dfs(&mut default_map, start, 0);
    let default_distance = default_map[end.y][end.x];

    for (y, x) in (0..map.len()).cartesian_product(0..map[0].len()) {
        if map[y][x] == -1 {
            let mut new_map = map.clone();
            new_map[y][x] = 0;

            let mut has_adjacent_walls = false;
            for (dy, dx) in [(1, 0), (0, 1)] {
                let new_y = y as isize + dy;
                let new_x = x as isize + dx;
                if new_y >= 0 && new_x >= 0 {
                    let new_y = new_y as usize;
                    let new_x = new_x as usize;
                    if new_y < map.len() && new_x < map[0].len() && map[new_y][new_x] == -1 {
                        new_map[new_y][new_x] = 0;
                        has_adjacent_walls = true;
                    }
                }
            }

            let mut new_map_clone = new_map.clone();
            dfs(&mut new_map_clone, start, 0);
            let new_distance = new_map_clone[end.y][end.x];

            if has_adjacent_walls {
                results.push((Pos { x, y }, end, default_distance - new_distance));
            } else {
                results.push((Pos { x, y }, Pos { x, y }, default_distance - new_distance));
            }
        }
    }

    results
}

fn main() -> Result<()> {
    let (map, start, end) = parse_input(&read_to_string("input.txt")?)?;

    println!("{:?}", map);
    Ok(())
}
