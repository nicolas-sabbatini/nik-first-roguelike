use bevy::prelude::*;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub tag: Tile,
    pub name: Name,
    pub position: Position,
    // #[bundle]
    // pub sprite: SpriteBundle,
}
