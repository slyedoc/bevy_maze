#[allow(dead_code)]
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use fps::FPSPlugin;
use maze::MazePlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

mod maze;
mod fps;
mod camera;
mod player;
mod menu;


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
 enum AppState {
     Loading,
     Menu,
     //Playing,
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

        .add_state(AppState::Menu)


        // // A state's "enter" schedule is run once when the state is entered
        // .state_enter(AppState::Loading, SystemStage::parallel()
        //     .with_system(setup)
        //     .with_system(load_textures)
        // )

        //config
        .add_startup_system(camera::spawn_cameras.system())
        .add_system(camera::pan_orbit_camera.system())
        .add_plugin(MazePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FPSPlugin)
        .add_plugin(MenuPlugin)
        .run();
}

// #[allow(dead_code)]
// // Labels for order of operations for systems
// #[derive(SystemLabel, Clone, Hash, Copy, PartialEq, Eq, Debug)]
// enum CommonLabels {
//     Input,
//     Action,
// }


// #[allow(dead_code)]
// enum GameState {
//     Loading,
//     Menu,
//     Playing,
// }