use bevy::prelude::*;

use crate::cleanup::*;
use crate::AppState;
use crate::maze::MazeSize;

mod config {
    use super::*;

    pub const COLOR: Color = Color::YELLOW;

    pub const WORLD_SCALE: f32 = 600.0;
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerMaterials>()
        .add_system_set(
            SystemSet::on_enter(AppState::Playing)
                .with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Playing)
                .with_system(cleanup_system::<Player>.system()),
        );
    }
}

struct Player;

/// Adds cameras to our game
pub fn spawn_player(mut commands: Commands, materials: Res<PlayerMaterials>, size: Res<MazeSize>) {

    let cell_size: f32 = config::WORLD_SCALE / size.x as f32;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(cell_size, cell_size)),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            material: materials.player_material.clone(),
            ..Default::default()
        })
        .insert(Player);
}

pub struct PlayerMaterials {
    player_material: Handle<ColorMaterial>,
}

impl FromWorld for PlayerMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");
        PlayerMaterials {
            player_material: materials.add(config::COLOR.into()),
        }
    }
}
