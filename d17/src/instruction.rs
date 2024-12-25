use crate::state::{State, N};

pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

fn combo(operand: N, state: &mut State) -> N {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        op => panic!("Invalid operand: {}", op),
    }
}

impl From<N> for Instruction {
    fn from(i: N) -> Self {
        match i {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction: {}", i),
        }
    }
}

impl Instruction {
    pub fn execute(&self, operand: N, state: &mut State) -> Option<N> {
        let mut value = None;

        match self {
            Instruction::Adv => state.a /= (2 as N).pow(combo(operand, state) as _),
            Instruction::Bxl => state.b ^= operand,
            Instruction::Bst => state.b = combo(operand, state) % 8,
            Instruction::Jnz if state.a != 0 => {
                state.pc = operand as usize;
                return value;
            }
            Instruction::Jnz => {}
            Instruction::Bxc => state.b ^= state.c,
            Instruction::Out => value = Some(combo(operand, state) % 8),
            Instruction::Bdv => state.b = state.a / (2 as N).pow(combo(operand, state) as _),
            Instruction::Cdv => state.c = state.a / (2 as N).pow(combo(operand, state) as _),
        }
        state.pc += 2;

        value
    }
}
