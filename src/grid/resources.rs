use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Default, Resource)]
pub struct Grid {
    pub tiles: HashMap<IVec2, Entity>,
}
