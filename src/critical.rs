pub const CRITICAL_MULTIPLIER: u16 = 2;
const CRITICAL_PERCENT: f32 = 0.1;

pub struct Critical {
    pub percent: f32,
}

impl Default for Critical {
    fn default() -> Self {
        Self {
            percent: CRITICAL_PERCENT,
        }
    }
}
