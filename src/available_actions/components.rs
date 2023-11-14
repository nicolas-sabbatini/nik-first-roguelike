use super::actions::Action;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);
