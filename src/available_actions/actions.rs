use crate::grid::{components::Position, resources::Grid};
use bevy::prelude::*;

pub trait Action: Send + Sync + Reflect + 'static {
    fn execute(&self, world: &mut World) -> bool;
}

#[derive(Debug, Reflect)]
pub struct WalkAction {
    pub actor: Entity,
    pub target_position: IVec2,
}
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        println!("Executing walk action: {self:?}");
        let Some(board) = world.get_resource::<Grid>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.target_position) {
            return false;
        };
        let Some(mut position) = world.get_mut::<Position>(self.actor) else {
            return false;
        };
        position.0 = self.target_position;
        true
    }
}
