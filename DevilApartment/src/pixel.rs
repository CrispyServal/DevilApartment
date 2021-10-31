#[derive(Copy, Clone)]
pub struct Pixel {
    pub id: u8,
    pub is_fall: bool,
    pub speed: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            id: 0,
            is_fall: false,
            speed: 8
        }
    }
}

// xx xx xx xx
// ?? [??, 速度] 属性 id

// id: 8bit

// 属性，假设有8种
// ？ ？ ？ ？ ？ ？ ？ 掉落否

// 速度（假设液体掉落先加速，然后达到满速，因此可以记几个状态，查表得dy）
// 暂且假设16帧达到满速，用4bit存
const PIXEL_AIR: u8 = 0;
const PIXEL_SAND: u8 = 1;
const PIXEL_STONE: u8 = 2;

const DY_LUT_LEN: u8 = 16;
const DY_LUT: [usize; (DY_LUT_LEN as usize)] = [1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8];

impl Pixel {
    pub fn from_id(id: u8) -> Self {
        match id {
            1 => Pixel {
                id,
                is_fall: true,
                speed: 0,
            },
            2 => Pixel {
                id,
                is_fall: false,
                speed: 0,
            },
            _ => Pixel::default(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.id == 0
    }

    pub fn is_fall(&self) -> bool{
        self.is_fall
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }

    pub fn get_dy(&self) -> usize {
        DY_LUT[self.get_speed() as usize]
    }

    pub fn add_speed(&self) -> Pixel {
        let speed = (self.get_speed() + 1).min(DY_LUT_LEN - 1);
        Pixel {
            id: self.id,
            is_fall: self.is_fall,
            speed,
        }
    }
}
