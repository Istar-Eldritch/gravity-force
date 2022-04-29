use crate::assets::{AsciiSheet, TILE_SIZE};
use bevy::prelude::*;

#[derive(Component)]
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
        .insert(Name::new("Player"));
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_player, mut transform) = player_query.single_mut();
    const base_speed: f32 = 5.0;
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += base_speed * time.delta_seconds() * TILE_SIZE;
    }

    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= base_speed * time.delta_seconds() * TILE_SIZE;
    }

    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += base_speed * time.delta_seconds() * TILE_SIZE;
    }

    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= base_speed * time.delta_seconds() * TILE_SIZE;
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.after(spawn_player));
    }
}
