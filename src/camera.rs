use bevy::prelude::*;

/// Marker component for game camera
pub struct MainCamera;
/// Marker component for UI camera
pub struct UiCamera;

/// Adds cameras to our game
pub fn spawn_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
}