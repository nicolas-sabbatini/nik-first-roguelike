use crate::flow_control::GameState;

use self::systems::{load_assets, render_pieces, render_tiles, update_piece_position};
use bevy::prelude::*;

mod constants;
pub mod resources;
mod systems;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets).add_systems(
            Update,
            (render_tiles, render_pieces, update_piece_position).run_if(in_state(GameState::Play)),
        );
    }
}
