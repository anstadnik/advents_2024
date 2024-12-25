use crate::state::{State, N};
use crate::Instruction;
use anyhow::{anyhow, Result};
use std::str::FromStr;
use winnow::combinator::empty;
use winnow::{ascii::dec_int, combinator::separated, seq, PResult, Parser};

#[derive(Clone, Debug)]
pub struct Program {
    pub state: State,
    pub code: Vec<N>,
}

impl Program {
    pub fn execute(&mut self) {
        loop {
            let pc = self.state.pc;
            if pc >= self.code.len() {
                break;
            }
            let instr = self.code[pc];
            let operand = self.code[pc + 1];
            Instruction::from(instr).execute(operand, &mut self.state);
        }
    }
}

fn parse_program(s: &mut &str) -> PResult<Program> {
    seq! { Program {
        state: seq! { State {
            _: "Register A: ",
            a: dec_int,
            _: "\nRegister B: ",
            b: dec_int,
            _: "\nRegister C: ",
            c: dec_int,
            pc: empty.value(0),
            output: empty.value(Vec::new())
        }},
        _: "\n\nProgram: ",
        code: separated(1.., dec_int::<_, N, _>, ',')
    }}
    .parse_next(s)
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(mut s: &str) -> Result<Self> {
        parse_program(&mut s).map_err(|e| anyhow!("{e}"))
    }
}
