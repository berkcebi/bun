#[derive(Debug, Clone, Copy)]
pub enum Sprite {
    Player = 0,
    Goblin = 1,
    TargetIndicator = 2,
    Wall = 3,
}

impl Sprite {
    pub const SIZE: f32 = 16.0;
    pub const SHEET_PATH: &'static str = "sprite-sheet.png";
    pub const SHEET_COLUMNS: usize = 3;
    pub const SHEET_ROWS: usize = 2;

    pub fn index(&self) -> usize {
        *self as usize
    }
}
