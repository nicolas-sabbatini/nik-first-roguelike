use bevy::prelude::*;

use self::systems::{load_assets, render_tiles};

mod resources;
mod systems;

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets).add_system(render_tiles);
    }
}
