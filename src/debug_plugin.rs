// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::available_actions::actions::WalkAction;
use crate::grid::components::{Position, Tile};
use crate::pieces::components::{Piece, PieceKind};
use crate::player::components::Player;
use crate::render::resources::GraphicsAssets;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
        // app.add_plugin(LogDiagnosticsPlugin::default());
        // app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.register_type::<GraphicsAssets>()
            .register_type::<Piece>()
            .register_type::<PieceKind>()
            .register_type::<Player>()
            .register_type::<Position>()
            .register_type::<Tile>()
            .register_type::<WalkAction>();
    }
}
