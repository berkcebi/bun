#[derive(Clone, Copy)]
pub enum Sprite {
    Player = 0,
}

impl Sprite {
    pub const SIZE: f32 = 16.0;
    pub const SHEET_PATH: &'static str = "sprite-sheet.png";
    pub const SHEET_COLUMNS: usize = 3;
    pub const SHEET_ROWS: usize = 1;

    pub fn index(&self) -> u32 {
        *self as u32
    }
}
