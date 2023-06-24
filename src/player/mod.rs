use self::systems::{move_player, spawn_player};
use crate::flow_control::GameState;
use bevy::prelude::*;

mod components;
mod systems;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Play)))
            .add_system(move_player);
    }
}
