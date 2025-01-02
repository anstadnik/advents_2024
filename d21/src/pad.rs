#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
    // Add cached
    fn get_char(&self, c: char) -> Pos {
        self.keys.iter().find(|(k, _)| k == &c).unwrap().1
    }

    fn is_forbidden(&self, pos: Pos, move_: &str) -> bool {
        self.forbidden.iter().any(|el| el == &(pos, move_))
    }

    pub fn gen_moves(&mut self, c: char) -> String {
        let Pos { x, y } = self.get_char(c);
        let (dx, dy) = (x - self.pos.x, y - self.pos.y);
        let old_pos = self.pos;
        self.pos = Pos { x, y };

        let col_move = ["<", ">"][(dx > 0) as usize].repeat(dx.unsigned_abs());
        let row_move = ["^", "v"][(dy > 0) as usize].repeat(dy.unsigned_abs());

        (if self.is_forbidden(old_pos, &col_move) {
            row_move + &col_move
        } else if self.is_forbidden(old_pos, &row_move) || col_move.starts_with('<') {
            col_move + &row_move
        } else {
            row_move + &col_move
        }) + "A"
    }

    pub fn gen_moves_str(mut self, s: String) -> Vec<String> {
        s.chars().map(move |c| self.gen_moves(c)).collect()
    }
}
