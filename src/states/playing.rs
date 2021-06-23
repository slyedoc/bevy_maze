use bevy::prelude::*;

use crate::AppState;


pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(SystemSet::on_update(AppState::Playing).with_system(exit.system()));
    }
}


fn exit(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state.set(AppState::Menu).unwrap();
    }
}