use super::{
    components::{Player, PlayerBundle},
    PlayerInputReadyEvent,
};
use crate::{
    available_actions::{actions::WalkAction, components::Actor, resources::ActorQueue},
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
        actor: Actor(None),
    });
}

pub fn queue_player_movement_action(
    keys: Res<Input<KeyCode>>,
    mut actor_queue: ResMut<ActorQueue>,
    mut player_query: Query<(&Position, &mut Actor, Entity), With<Player>>,
    mut event_writer: EventWriter<PlayerInputReadyEvent>,
) {
    let Ok((position, mut actor, entity)) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        let target_position = position.0 + dir;
        let movement_action = WalkAction {
            actor: entity,
            target_position,
        };
        println!("Queuing movement action: {movement_action:?}");
        actor.0 = Some(Box::new(movement_action));
        actor_queue.0.push_back(entity);
        event_writer.send(PlayerInputReadyEvent);
    }
}

// pub fn move_player(
//     keys: ResMut<Input<KeyCode>>,
//     mut player_query: Query<&mut Position, With<Player>>,
// ) {
//     let Ok(mut position) = player_query.get_single_mut() else { return };
//     for (key, dir) in DIR_KEY_MAPPING {
//         if !keys.just_pressed(key) {
//             continue;
//         }
//         position.0 += dir;
//     }
// }
