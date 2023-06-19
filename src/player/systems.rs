use super::components::{Player, PlayerBundle};
use crate::{
    grid::components::Position,
    pieces::components::{Piece, PieceKind},
};
use bevy::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        tag: Player,
        name: Name::new("Player"),
        position: Position(IVec2::new(0, 0)),
        piece: Piece {
            kind: PieceKind::Player,
        },
    });
}
