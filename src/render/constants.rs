use bevy::prelude::Vec2;

pub const SPRITESHEET_PATH: &str = "spritesheet.png";
pub const SPRITESHEET_PADDING: Option<Vec2> = None;
pub const SPRITESHEET_OFFSET: Option<Vec2> = None;
pub const SPRITESHEET_ROWS: usize = 16;
pub const SPRITESHEET_COLUMNS: usize = 10;
pub const SPRITE_SIZE: f32 = 8.0;

pub const SNAP_TO_GRID_DELTA: f32 = 0.05;
pub const LERP_SPEED_PX_SECONDS: f32 = 16.0;
