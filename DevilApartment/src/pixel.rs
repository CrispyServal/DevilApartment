mod air;
mod common_dy;
mod sand;
mod stone;
mod water;

use air::Air;
use common_dy::*;
use dyn_clonable::*;

use crate::world_buffer::WorldBuffer;
use crate::UVec2;

#[clonable]
pub trait Pixel: Send + Sync + Clone {
    fn get_id(&self) -> u8;
    /// 空的像素不阻挡其他像素、粒子运动
    fn is_empty(&self) -> bool;
    /// 液体可以允许固体进入
    fn is_liquid(&self) -> bool;
    fn is_solid(&self) -> bool;
    fn try_move_self(
        &mut self,
        world_buffer: &WorldBuffer,
        self_x: usize,
        self_y: usize,
    ) -> Option<UVec2>;
}

/// 会掉落的像素，
pub trait FallingPixel: Send + Sync {
    /// 获取当前帧应该下降多少距离
    fn get_dy(&self) -> usize;
    /// 递增自己的速度（速度即dy）
    fn add_dy(&mut self);
    fn reset_dy(&mut self);
}

pub fn default_pixel() -> Box<dyn Pixel> {
    Box::new(Air)
}

pub fn new_from_id(id: u8) -> Box<dyn Pixel> {
    match id {
        1 => Box::new(sand::Sand::default()),
        2 => Box::new(stone::Stone),
        3 => Box::new(water::Water::default()),
        _ => default_pixel(),
    }
}
