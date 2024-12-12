use nalgebra::{try_convert, Scalar, Vector2};
pub type Pos = Vector2<usize>;
pub type IPos = Vector2<isize>;
pub type VVc = Vec<Vec<char>>;

pub trait IndexPoint<T: Scalar> {
    fn get_p(&self, p: Vector2<T>) -> Option<char>;
    fn get_mut_p(&mut self, p: Vector2<T>) -> Option<&mut char>;
}

impl IndexPoint<usize> for VVc {
    fn get_p(&self, p: Pos) -> Option<char> {
        self.get(p.y)?.get(p.x).copied()
    }

    fn get_mut_p(&mut self, p: Pos) -> Option<&mut char> {
        self.get_mut(p.y)?.get_mut(p.x)
    }
}

impl IndexPoint<isize> for VVc {
    fn get_p(&self, p: IPos) -> Option<char> {
        self.get_p(try_convert::<_, Pos>(p)?)
    }

    fn get_mut_p(&mut self, p: IPos) -> Option<&mut char> {
        self.get_mut_p(try_convert::<_, Pos>(p)?)
    }
}
