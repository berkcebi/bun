use crate::sprite::Sprite;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub sprite: Sprite,
    pub is_obstructed: bool,
}

impl Tile {
    pub const SIZE: f32 = Sprite::SIZE;

    const WALL: Self = Self {
        sprite: Sprite::Wall,
        is_obstructed: true,
    };
}

pub struct Zone {
    columns: usize,
    rows: usize,
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl Zone {
    pub fn new(columns: usize, rows: usize) -> Self {
        let mut tiles = vec![vec![None; rows]; columns];
        for x in 0..columns {
            for y in 0..rows {
                if x == 0 || x == columns - 1 || y == 0 || y == rows - 1 {
                    tiles[x][y] = Some(Tile::WALL);
                }
            }
        }

        tiles[10][6] = Some(Tile::WALL);
        tiles[10][7] = Some(Tile::WALL);

        Self {
            columns,
            rows,
            tiles,
        }
    }

    pub fn tile_position(&self, x: usize, y: usize) -> Vec2 {
        self.origin() + Vec2::new(x as f32 * Tile::SIZE, y as f32 * Tile::SIZE)
    }

    fn width(&self) -> f32 {
        self.columns as f32 * Tile::SIZE
    }

    fn height(&self) -> f32 {
        self.rows as f32 * Tile::SIZE
    }

    fn origin(&self) -> Vec2 {
        Vec2::new(
            (self.width() - Tile::SIZE) / -2.0,
            (self.height() - Tile::SIZE) / -2.0,
        )
    }
}
