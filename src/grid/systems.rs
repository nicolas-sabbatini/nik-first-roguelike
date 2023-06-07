use bevy::prelude::*;
use std::collections::HashMap;

use super::{
    components::{Position, Tile, TileBundle},
    resources::Grid,
};

const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 10;

pub fn initialize_grid(mut commands: Commands, mut grid: ResMut<Grid>) {
    grid.tiles = HashMap::new();
    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            let position = IVec2::new(x, y);
            let tile = commands.spawn(TileBundle {
                name: Name::new(format!("Tile ({}, {})", x, y)),
                position: Position(position),
                tag: Tile,
            });
            grid.tiles.insert(position, tile.id());
        }
    }
}
