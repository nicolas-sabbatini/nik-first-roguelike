use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);
