use bevy::prelude::*;

use crate::{
    asset_loading::AssetList,
    grid::components::{Position, Tile},
};

use super::resources::GraphicsAssets;

const SPRITESHEET_PATH: &str = "spritesheet.png";
pub const TILE_SIZE: f32 = 8.0;
pub const TILE_PADDING: Option<Vec2> = None;
pub const TILE_OFFSET: Option<Vec2> = None;
pub const TILES_ROW: usize = 16;
pub const TILES_COL: usize = 10;

pub fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<AssetList>,
) {
    let texture_handle = asset_server.load(SPRITESHEET_PATH);
    asset_list.0.push(texture_handle.clone_untyped());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(TILE_SIZE),
        TILES_ROW,
        TILES_COL,
        TILE_PADDING,
        TILE_OFFSET,
    );
    let sprite_texture = texture_atlasses.add(texture_atlas);
    commands.insert_resource(GraphicsAssets { sprite_texture });
}

pub fn render_tiles(
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in &query {
        let mut sprite = TextureAtlasSprite::new(17);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        let transform = Transform::from_translation(Vec3::new(
            position.0.x as f32 * TILE_SIZE,
            position.0.y as f32 * TILE_SIZE,
            0.,
        ));
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform,
            ..Default::default()
        });
    }
}
