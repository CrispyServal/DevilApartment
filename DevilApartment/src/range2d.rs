#[derive(Debug)]
pub struct Range2d {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl Range2d {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
        Self { min_x, max_x, min_y, max_y}
    }
}
