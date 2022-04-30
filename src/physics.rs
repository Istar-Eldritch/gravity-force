use crate::assets::TILE_SIZE;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use core::ops::{Add, Mul};
use glam::Vec3;
use std::ops::{Deref, DerefMut};

// This should be an array of all the forces that affect the entity
#[derive(Component, Inspectable)]
pub struct Accelerations(pub Vec3);

impl Deref for Accelerations {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Accelerations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Component, Inspectable)]
pub struct Speed(pub Vec3);

impl Deref for Speed {
    type Target = Vec3;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Speed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Computes speed from acceleration
pub fn forces(mut forces_query: Query<(&Accelerations, &mut Speed)>) {
    for (accelerations, mut speed) in forces_query.iter_mut() {
        *speed = Speed((*speed).add(**accelerations));
    }
}

fn movement(mut moving_query: Query<(&Speed, &mut Transform)>, time: Res<Time>) {
    for (speed, mut transform) in moving_query.iter_mut() {
        transform.translation = transform
            .translation
            .add(speed.mul(time.delta_seconds()).mul(TILE_SIZE));
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(forces).add_system(movement);
    }
}
