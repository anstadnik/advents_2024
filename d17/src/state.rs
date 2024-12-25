pub type N = i64;

#[derive(Clone, Debug)]
pub struct State {
    pub a: N,
    pub b: N,
    pub c: N,
    pub pc: usize,
    pub output: Vec<N>,
}

impl State {
    pub fn new(a: N, b: N, c: N, pc: usize) -> Self {
        Self {
            a,
            b,
            c,
            pc,
            output: Vec::new(),
        }
    }
}
