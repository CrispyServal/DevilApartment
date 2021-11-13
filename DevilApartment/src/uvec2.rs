#[derive(Copy, Clone, Debug)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl UVec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn same(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn move_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn move_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
}
