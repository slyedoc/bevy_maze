#[allow(dead_code)]
use bevy::prelude::*;
use maze::MazePlugin;

mod maze;
#[allow(dead_code)]
enum GameState {
    Loading,
    Menu,
    Playing,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 }) // Crashes if after default
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Bevy Maze".to_string(),
            ..Default::default()
        })
        //config
        .add_startup_system(spawn_cameras.system())
        .add_plugin(MazePlugin)
        .run();
}

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

#[allow(dead_code)]
// Labels for order of operations for systems
#[derive(SystemLabel, Clone, Hash, Copy, PartialEq, Eq, Debug)]
enum CommonLabels {
    Input,
    Action,
}
