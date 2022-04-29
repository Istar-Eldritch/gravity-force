use bevy::prelude::*;
use std::ops::Deref;

pub const TILE_SIZE: f32 = 0.3;

pub struct AsciiSheet(Handle<TextureAtlas>);

impl Deref for AsciiSheet {
    type Target = Handle<TextureAtlas>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(image, Vec2::splat(9.0), 16, 16, Vec2::splat(2.0));
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiSheet(atlas_handle));
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii);
    }
}
