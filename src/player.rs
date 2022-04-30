use crate::assets::{AsciiSheet, TILE_SIZE};
use crate::physics::{Accelerations, Speed};
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Player;

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(30);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Transform::default()
            },
            ..SpriteSheetBundle::default()
        })
        .insert(Player)
        .insert(Accelerations(Vec3::default()))
        .insert(Speed(Vec3::default()))
        .insert(Name::new("Player"));
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Accelerations)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_player, mut accelerations) = player_query.single_mut();

    const BASE_ACCEL: f32 = 5.0;
    //**accelerations = accelerations.mul(BASE_ACCEL).mul(time.delta_seconds());

    let mut accelerated = false;
    if keyboard.pressed(KeyCode::W) {
        accelerations.y += BASE_ACCEL * time.delta_seconds() * TILE_SIZE;
        accelerated = true;
    }

    if keyboard.pressed(KeyCode::S) {
        accelerations.y -= BASE_ACCEL * time.delta_seconds() * TILE_SIZE;
        accelerated = true;
    }

    if keyboard.pressed(KeyCode::D) {
        accelerations.x += BASE_ACCEL * time.delta_seconds() * TILE_SIZE;
        accelerated = true;
    }

    if keyboard.pressed(KeyCode::A) {
        accelerations.x -= BASE_ACCEL * time.delta_seconds() * TILE_SIZE;
        accelerated = true;
    }

    if !accelerated {
        *accelerations = Accelerations(Vec3::default());
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.after(spawn_player));
    }
}
