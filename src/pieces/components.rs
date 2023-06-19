use bevy::prelude::*;

#[non_exhaustive]
pub enum PieceKind {
    Player,
}

#[derive(Component)]
pub struct Piece {
    pub kind: PieceKind,
}
