#[allow(dead_code)]
use bevy::prelude::*;
use fps::FPSPlugin;
use maze::MazePlugin;

mod maze;
mod fps;
mod camera;

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
        .insert_resource(WindowDescriptor {
            title: "Bevy Maze".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)

        //config
        .add_startup_system(camera::spawn_cameras.system())
        .add_plugin(MazePlugin)
        .add_plugin(FPSPlugin)
        .run();
}

#[allow(dead_code)]
// Labels for order of operations for systems
#[derive(SystemLabel, Clone, Hash, Copy, PartialEq, Eq, Debug)]
enum CommonLabels {
    Input,
    Action,
}
