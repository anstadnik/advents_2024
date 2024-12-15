use glam::{IVec2, UVec2};
pub type Pos = UVec2;
pub type IPos = IVec2;
pub type VVc = Vec<Vec<char>>;

pub trait IndexPoint<T> {
    fn get_p(&self, p: T) -> Option<char>;
    fn get_mut_p(&mut self, p: T) -> Option<&mut char>;
}

impl IndexPoint<UVec2> for VVc {
    fn get_p(&self, p: Pos) -> Option<char> {
        self.get(p.y as usize)?.get(p.x as usize).copied()
    }

    fn get_mut_p(&mut self, p: Pos) -> Option<&mut char> {
        self.get_mut(p.y as usize)?.get_mut(p.x as usize)
    }
}

impl IndexPoint<IVec2> for VVc {
    fn get_p(&self, p: IPos) -> Option<char> {
        self.get_p(p.as_uvec2())
    }

    fn get_mut_p(&mut self, p: IPos) -> Option<&mut char> {
        self.get_mut_p(p.as_uvec2())
    }
}
