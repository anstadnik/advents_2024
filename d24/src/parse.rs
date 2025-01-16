use fxhash::FxHashMap as HashMap;
use anyhow::{anyhow, Result};
//use std::collections::HashMap;
use winnow::ascii::{alphanumeric1, dec_int};
use winnow::combinator::{empty, fail, separated};
use winnow::{dispatch, seq, PResult, Parser};

#[derive(Debug, Clone, Copy)]
pub enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    pub fn apply(&self, a: bool, b: bool) -> bool {
        match self {
            Op::And => a & b,
            Op::Xor => a ^ b,
            Op::Or => a | b,
        }
    }
}

type State_<'a> = Vec<(&'a str, bool)>;
fn parse_init<'a>(s: &mut &'a str) -> PResult<State_<'a>> {
    separated(
        1..,
        seq!(alphanumeric1, _: ": ", dec_int.map(|v: i8| v == 1)),
        '\n',
    )
    .parse_next(s)
}

type Ops_<'a> = Vec<(&'a str, Op, &'a str, &'a str)>;
fn parse_ops<'a>(s: &mut &'a str) -> PResult<Ops_<'a>> {
    separated(
        1..,
        seq!(
            alphanumeric1,
            _: " ",
            dispatch! {alphanumeric1;
                "AND" => empty.value(Op::And),
                "XOR" => empty.value(Op::Xor),
                "OR" => empty.value(Op::Or),
                _ => fail
            },
            _: " ", 
            alphanumeric1,
            _: " -> ", alphanumeric1),
        '\n',
    )
    .parse_next(s)
}

fn parse_input_<'a>(s: &mut &'a str) -> PResult<(State_<'a>, Ops_<'a>)> {
    seq!(parse_init, _: "\n\n", parse_ops).parse_next(s)
}

pub type State<'a> = HashMap<&'a str, bool>;
pub type Ops<'a> = Vec<(&'a str, Op, &'a str, &'a str)>;

pub fn parse_intput(s: &str) -> Result<(State, Ops)> {
    let (init, ops) = parse_input_.parse(s).map_err(|e| anyhow!("{e}"))?;
    Ok((init.into_iter().collect(), ops))
}
