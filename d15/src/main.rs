mod enums;
mod field;
use anyhow::{anyhow, Result};
use enums::prelude::*;
use field::prelude::*;
use std::fs::read_to_string;

fn parse_input(s: &str) -> Result<(Field, Vec<Direction>)> {
    let (field, moves) = s.trim().split_once("\n\n").ok_or(anyhow!("Cannot split"))?;
    Ok((
        Field::new(field),
        moves
            .chars()
            .filter(|&c| c != '\n')
            .map(From::from)
            .collect(),
    ))
}

fn task1(mut field: Field, directions: &[Direction]) -> usize {
    for &m in directions {
        field.step(m, None);
    }
    field.get_gps_coords().map(|Pos { y, x }| y * 100 + x).sum()
}

fn task2(mut field: Field, directions: &[Direction]) -> usize {
    field.double();
    for &m in directions {
        field.step(m, None);
    }
    field.get_gps_coords().map(|Pos { y, x }| y * 100 + x).sum()
}

fn main() -> Result<()> {
    let (field, directions) = parse_input(&read_to_string("input.txt")?)?;

    println!("Answer 1: {}", task1(field.clone(), &directions));
    println!("Answer 2: {}", task2(field, &directions));

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
    fn test_example1_task1() -> Result<()> {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        let (field, directions) = parse_input(input)?;
        assert_eq!(task1(field, &directions), 2028);

        Ok(())
    }

    #[test]
    fn test_example1_task2() -> Result<()> {
        let input = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

        let (field, directions) = parse_input(input)?;
        //assert_eq!(task2(field, &directions), 2028);
        task2(field, &directions);

        Ok(())
    }

    #[test]
    fn test_example2_task1() -> Result<()> {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let (field, directions) = parse_input(input)?;
        assert_eq!(task1(field, &directions), 10092);

        Ok(())
    }

    #[test]
    fn test_example2_task2() -> Result<()> {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let (field, directions) = parse_input(input)?;

        assert_eq!(task2(field, &directions), 9021);

        Ok(())
    }
}
