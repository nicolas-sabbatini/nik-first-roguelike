use self::systems::{load_assets, render_pieces, render_tiles, update_piece_position};
use bevy::prelude::*;

mod constants;
pub mod resources;
mod systems;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_system(render_tiles)
            .add_system(render_pieces)
            .add_system(update_piece_position);
    }
}
