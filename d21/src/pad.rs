use itertools::Itertools;
use std::collections::HashMap;

const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone)]
pub struct KeyPad {
    keys: HashMap<char, Pos>,
    forbidden: HashMap<Pos, &'static str>,
    pos: Pos,
}

impl KeyPad {
    pub fn new(keys: &[(char, Pos)], forbidden: &[(Pos, &'static str)]) -> Self {
        Self {
            keys: keys.iter().copied().collect(),
            forbidden: forbidden.iter().cloned().collect(),
            pos: keys.iter().find(|(c, _)| c == &'A').unwrap().1,
        }
    }

    pub fn gen_moves<T: Iterator<Item = char> + Clone>(
        &self,
        code: T,
    ) -> impl Iterator<Item = Vec<String>> + use<'_, T> + Clone {
        //if DEBUG {
        //    println!("Generating moves for {}", code.clone().collect::<String>());
        //}
        code.scan(self.pos, |pos, c| {
            if DEBUG {
                println!("{:?}, {:?}", pos, c);
            }
            let new_pos @ Pos { x, y } = self.keys[&c];
            let (dx, dy) = (x - pos.x, y - pos.y);
            let old_pos = *pos;
            *pos = new_pos;

            let moves: Vec<_> = [
                "^".repeat((-dy).max(0) as usize),
                "v".repeat(dy.max(0) as usize),
                "<".repeat((-dx).max(0) as usize),
                ">".repeat(dx.max(0) as usize),
            ]
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect();

            let n = moves.len();
            Some([
                moves
                    .into_iter()
                    .permutations(n)
                    .filter(|v| !v.is_empty())
                    .filter(|v| self.forbidden.get(&old_pos).is_none_or(|c| *c != v[0]))
                    .map(|v| v.into_iter().join(""))
                    .collect(),
                vec!["A".to_string()],
            ])
        })
        .flatten()
        .filter(|x| !x.is_empty())
        .inspect(|x| {
            if DEBUG {
                println!("{:?}", x)
            }
        })
        .multi_cartesian_product()
        .inspect(|x| {
            if DEBUG {
                println!("{:?}", x)
            }
        })
    }

    pub fn gen_moves_iter<T: Iterator<Item = Vec<String>>> (
        &self,
        codes: T,
    ) -> impl Iterator<Item = Vec<String>> + use<'_, T> {
        //if DEBUG {
        //    println!(
        //        "Generating moves iter for {:?}",
        //        codes.clone().collect::<Vec<_>>()
        //    );
        //}
        codes.flat_map(move |code| {
            self.gen_moves(code.into_iter().flat_map(|x| x.chars().collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        })
    }

    pub fn gen_shortest_iter<T: Iterator<Item = Vec<String>> >(&self, codes: T) -> String {
        self.gen_moves_iter(codes)
            .min_by_key(|x| x.iter().map(|x| x.len()).sum::<usize>())
            .unwrap()
            .join("")
    }
}
