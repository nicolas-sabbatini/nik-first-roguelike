use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default, Reflect)]
pub struct Grid {
    pub tiles: HashMap<IVec2, Entity>,
}
