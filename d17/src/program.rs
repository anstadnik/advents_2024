use crate::state::{State, N};
use crate::Instruction;
use anyhow::{anyhow, Result};
use indicatif::ParallelProgressIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::iter::from_fn;
use std::str::FromStr;
use winnow::combinator::empty;
use winnow::{ascii::dec_int, combinator::separated, seq, PResult, Parser};

#[derive(Clone, Debug)]
pub struct Program {
    pub state: State,
    pub code: Vec<N>,
}

impl Program {
    pub fn execute(&mut self) -> impl Iterator<Item = N> + '_ {
        from_fn(|| {
            let pc = self.state.pc;
            if pc >= self.code.len() {
                return None;
            }
            let instr = self.code[pc];
            let operand = self.code[pc + 1];
            Some(Instruction::from(instr).execute(operand, &mut self.state))
        })
        .flatten()
    }

    pub fn find_a(&self) -> Option<N> {
        let max = 1_000_000_000_000_000_usize;
        let code = self.code.clone();
        assert!(max < N::MAX as usize);

        (1..max)
            .into_par_iter()
            .progress()
            .find_first(|a| {
                let mut program = self.clone();
                program.state.a = *a as _;
                program.execute().eq(code.iter().copied())
            })
            .map(|a| a as N)
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
