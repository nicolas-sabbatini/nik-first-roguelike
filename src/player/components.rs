use crate::{
    available_actions::components::Actor, grid::components::Position, pieces::components::Piece,
};
use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub name: Name,
    pub position: Position,
    pub piece: Piece,
    pub actor: Actor,
}
