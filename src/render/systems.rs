use super::{constants::*, resources::GraphicsAssets};
use crate::{
    asset_loading::AssetList,
    grid::components::{Position, Tile},
    pieces::components::{Piece, PieceKind},
};
use bevy::prelude::*;

const TILE_Z_INDEX: f32 = 0.0;
const PIECE_Z_INDEX: f32 = 1.0;

fn grid_to_graphics(position: &Position, z_index: f32) -> Vec3 {
    Vec3::new(
        position.0.x as f32 * SPRITE_SIZE,
        position.0.y as f32 * SPRITE_SIZE,
        z_index,
    )
}

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
        Vec2::splat(SPRITE_SIZE),
        SPRITESHEET_ROWS,
        SPRITESHEET_COLUMNS,
        SPRITESHEET_PADDING,
        SPRITESHEET_OFFSET,
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
        sprite.custom_size = Some(Vec2::splat(SPRITE_SIZE));
        let transform = Transform::from_translation(grid_to_graphics(position, TILE_Z_INDEX));
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform,
            ..Default::default()
        });
    }
}

pub fn render_pieces(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece)>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position, piece) in &query {
        let mut sprite = TextureAtlasSprite::new(match &piece.kind {
            PieceKind::Player => 4,
            _ => 112,
        });
        sprite.custom_size = Some(Vec2::splat(SPRITE_SIZE));
        let transform = Transform::from_translation(grid_to_graphics(position, PIECE_Z_INDEX));
        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform,
            ..Default::default()
        });
    }
}
