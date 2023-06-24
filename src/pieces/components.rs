use bevy::prelude::*;

#[non_exhaustive]
#[derive(Default, Reflect)]
pub enum PieceKind {
    #[default]
    Player,
}

#[derive(Component, Default, Reflect)]
pub struct Piece {
    pub kind: PieceKind,
}
