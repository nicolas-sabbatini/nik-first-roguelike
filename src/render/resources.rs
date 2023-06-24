use bevy::prelude::*;

#[derive(Resource, Reflect)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}
