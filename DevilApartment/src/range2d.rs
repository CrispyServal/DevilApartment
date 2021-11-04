#[derive(Debug, Copy, Clone)]
pub struct Range2d {
    pub min_x: usize,
    pub max_x: usize,
    pub min_y: usize,
    pub max_y: usize,
}

impl Default for Range2d {
    fn default() -> Self {
        Self {
            min_x: usize::MAX,
            max_x: 0,
            min_y: usize::MAX,
            max_y: 0,
        }
    }
}

impl Range2d {
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}
