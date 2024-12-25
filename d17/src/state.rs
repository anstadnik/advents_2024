pub type N = i64;

#[derive(Clone, Debug)]
pub struct State {
    pub a: N,
    pub b: N,
    pub c: N,
    pub pc: usize,
}

impl State {
    #[allow(dead_code)]
    pub fn new(a: N, b: N, c: N, pc: usize) -> Self {
        Self { a, b, c, pc }
    }
}
