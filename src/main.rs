use bevy::prelude::*;

mod assets;
mod debug;
mod physics;
mod player;

use assets::AssetsPlugin;
use debug::DebugPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

fn spawn_camera(mut commands: Commands) {
    let size = 10.0;
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.top = size;
    cam.orthographic_projection.bottom = -1.0 * size;
    cam.orthographic_projection.right = size * ASPECT_RATIO;
    cam.orthographic_projection.left = -1.0 * size * ASPECT_RATIO;
    cam.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;

    commands.spawn_bundle(cam);
}

fn main() {
    let height = 900.0;
    let window = WindowDescriptor {
        title: "Test Game".into(),
        height,
        width: height * ASPECT_RATIO,
        present_mode: bevy::window::PresentMode::Fifo,
        ..WindowDescriptor::default()
    };
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(window)
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(DebugPlugin)
        .add_startup_system(spawn_camera)
        .run();
}
