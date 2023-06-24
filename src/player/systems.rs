use super::components::{Player, PlayerBundle};
use crate::{
    constants::DIR_KEY_MAPPING,
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

pub fn move_player(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let Ok(mut position) = player_query.get_single_mut() else { return };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        position.0 += dir;
    }
}
