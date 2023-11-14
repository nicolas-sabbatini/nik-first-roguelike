use self::{resources::Grid, systems::initialize_grid};
use crate::flow_control::GameState;
use bevy::prelude::*;

pub mod components;
pub mod resources;
mod systems;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>();

        app.add_systems(OnEnter(GameState::Play), initialize_grid);
    }
}
