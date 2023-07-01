use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Position(pub IVec2);

#[derive(Component, Reflect)]
pub struct Tile;

#[derive(Bundle)]
pub struct TileBundle {
    pub tag: Tile,
    pub name: Name,
    pub position: Position,
}
