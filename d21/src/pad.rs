use indicatif::{ProgressBar, ProgressIterator};
use itertools::Itertools;

const DEBUG: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPad {
    keys: &'static [(char, Pos)],
    forbidden: &'static [(Pos, &'static str)],
    pos: Pos,
}

pub const KEY_PAD1: KeyPad = KeyPad {
    keys: &[
        ('7', Pos { x: 0, y: 0 }),
        ('8', Pos { x: 1, y: 0 }),
        ('9', Pos { x: 2, y: 0 }),
        ('4', Pos { x: 0, y: 1 }),
        ('5', Pos { x: 1, y: 1 }),
        ('6', Pos { x: 2, y: 1 }),
        ('1', Pos { x: 0, y: 2 }),
        ('2', Pos { x: 1, y: 2 }),
        ('3', Pos { x: 2, y: 2 }),
        ('0', Pos { x: 1, y: 3 }),
        ('A', Pos { x: 2, y: 3 }),
    ],
    forbidden: &[
        (Pos { x: 1, y: 3 }, "<"),
        (Pos { x: 0, y: 2 }, "v"),
        (Pos { x: 2, y: 3 }, "<<"),
    ],
    pos: Pos { x: 2, y: 3 },
};

pub const KEY_PAD2: KeyPad = KeyPad {
    keys: &[
        ('^', Pos { x: 1, y: 0 }),
        ('A', Pos { x: 2, y: 0 }),
        ('<', Pos { x: 0, y: 1 }),
        ('v', Pos { x: 1, y: 1 }),
        ('>', Pos { x: 2, y: 1 }),
    ],
    forbidden: &[
        (Pos { x: 0, y: 1 }, "^"),
        (Pos { x: 1, y: 0 }, "<"),
        (Pos { x: 2, y: 0 }, "<<"),
    ],
    pos: Pos { x: 2, y: 0 },
};

impl KeyPad {
    //pub fn new(keys: &[(char, Pos)], forbidden: &[(Pos, &'static str)]) -> Self {
    //    Self {
    //        keys: keys.iter().copied().collect(),
    //        forbidden: forbidden.iter().cloned().collect(),
    //        pos: keys.iter().find(|(c, _)| c == &'A').unwrap().1,
    //    }
    //}

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
            let new_pos @ Pos { x, y } = self.keys.iter().find(|(k, _)| k == &c).unwrap().1;
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
                    .filter(|v| {
                        self.forbidden
                            .iter()
                            .all(|(p, c)| !(*p == old_pos && *c == v[0]))
                    })
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

    pub fn gen_moves_iter<T: Iterator<Item = Vec<String>>>(
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

    pub fn gen_shortest_iter<T: Iterator<Item = Vec<String>>>(&self, codes: T) -> String {
        self.gen_moves_iter(codes)
            .progress_with(ProgressBar::new_spinner())
            .min_by_key(|x| x.iter().map(|x| x.len()).sum::<usize>())
            .unwrap()
            .join("")
    }
}
