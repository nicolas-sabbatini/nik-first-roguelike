use self::systems::{queue_player_movement_action, spawn_player};
use crate::flow_control::{GameState, PlayState};
use bevy::prelude::*;

pub mod components;
mod systems;

#[derive(Event)]
pub struct PlayerInputReadyEvent;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerInputReadyEvent>()
            .add_systems(OnEnter(GameState::Play), spawn_player)
            .add_systems(
                Update,
                (queue_player_movement_action).run_if(in_state(PlayState::PlayerTurn)),
            );
    }
}
