#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AspectRatio {
    R16_9,
    R16_10,
    R4_3,
    R21_9,
}

impl AspectRatio {
    pub fn value(self) -> f32 {
        match self {
            Self::R16_9 => 16.0 / 9.0,
            Self::R16_10 => 16.0 / 10.0,
            Self::R4_3 => 4.0 / 3.0,
            Self::R21_9 => 21.0 / 9.0,
        }
    }
}