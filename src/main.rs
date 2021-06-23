#[allow(dead_code)]
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use camera::CameraPlugin;
use fps::FPSPlugin;
use grid::GridPlugin;
use maze::{MazePlugin, MazeSize};
use states::*;
use player::PlayerPlugin;

mod camera;
mod cleanup;
mod fps;
mod grid;
mod maze;
mod player;
mod states;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum AppState {
    Menu,
    Playing,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 }) // Crashes if after default
        .insert_resource(ClearColor(Color::GRAY))
        .insert_resource(WindowDescriptor {
            title: "Bevy Maze".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)

        // Setup
        .add_state(AppState::Menu)
        .add_plugin(CameraPlugin)
        .insert_resource(MazeSize { x: 11u8, y: 10u8 })
        .add_plugin(GridPlugin) // runs during playing
        .add_plugin(MazePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FPSPlugin)

        // Load state logic
        .add_plugin(MenuPlugin)
        .add_plugin(PlayingPlugin)
        .run();
}

// #[allow(dead_code)]
// // Labels for order of operations for systems
// #[derive(SystemLabel, Clone, Hash, Copy, PartialEq, Eq, Debug)]
// enum CommonLabels {
//     Input,
//     Action,
// }
