use crate::flow_control::GameState;
use bevy::prelude::*;

use self::{resources::Grid, systems::initialize_grid};

mod components;
mod resources;
mod systems;

pub struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>();

        app.add_system(initialize_grid.in_schedule(OnEnter(GameState::Play)));
    }
}
