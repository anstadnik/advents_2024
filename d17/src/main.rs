mod instruction;
mod program;
mod state;

use anyhow::Result;
use indicatif::ParallelProgressIterator;
use instruction::Instruction;
use program::Program;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use state::N;
use std::fs::read_to_string;

fn task1(program: &mut Program) -> String {
    program.execute();
    program
        .state
        .output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn task2_bruteforce(program: &Program) -> Option<N> {
    let _hmmm = 265_601_188_299_675_usize;
    let max = 1_000_000_000_000_000_usize;
    assert!(max < N::MAX as usize);
    (1..max)
        .into_par_iter()
        .progress()
        .find_first(|a| {
            let mut program = program.clone();
            program.state.a = *a as _;
            program.execute();
            program.state.output == program.code
        })
        .map(|a| a as N)
}

fn main() -> Result<()> {
    let mut program: Program = read_to_string("input.txt")?.parse()?;

    println!("Answer 1: {}", task1(&mut program));
    println!("Answer 2: {}", task2_bruteforce(&program).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::State;

    #[ignore]
    #[test]
    fn test_main() -> Result<()> {
        main()
    }

    #[test]
    fn test_register_c_contains_9() {
        let mut program = Program {
            state: State::new(0, 0, 9, 0),
            code: vec![2, 6],
        };
        program.execute();
        assert_eq!(program.state.b, 1);
    }

    #[test]
    fn test_register_a_contains_10() {
        let mut program = Program {
            state: State::new(10, 0, 0, 0),
            code: vec![5, 0, 5, 1, 5, 4],
        };
        let output = task1(&mut program);
        assert_eq!(output, "0,1,2");
    }

    #[test]
    fn test_register_a_contains_2024() {
        let mut program = Program {
            state: State::new(2024, 0, 0, 0),
            code: vec![0, 1, 5, 4, 3, 0],
        };
        let output = task1(&mut program);
        assert_eq!(output, "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(program.state.a, 0);
    }

    #[test]
    fn test_register_b_contains_29() {
        let mut program = Program {
            state: State::new(0, 29, 0, 0),
            code: vec![1, 7],
        };
        program.execute();
        assert_eq!(program.state.b, 26);
    }

    #[test]
    fn test_register_b_and_c() {
        let mut program = Program {
            state: State::new(0, 2024, 43690, 0),
            code: vec![4, 0],
        };
        program.execute();
        assert_eq!(program.state.b, 44354);
    }

    #[test]
    fn test_task1() {
        let input = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

        let mut program: Program = input.parse().unwrap();
        assert_eq!(task1(&mut program), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_task2() {
        let input = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

        let program: Program = input.parse().unwrap();
        assert_eq!(task2_bruteforce(&program).unwrap(), 117440);
    }
}
