use bevy::prelude::*;



mod config {
    use crate::maze;
    use super::*;

    //Sizes
    pub const SIZE : (f32, f32) = (maze::config::SIZE as f32 * 0.9, maze::config::SIZE as f32 * 0.9);
    pub const COLOR : Color = Color::YELLOW;

}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<PlayerMaterials>()
            .add_startup_system(spawn_player.system());

    }
}

struct Player;

/// Adds cameras to our game
pub fn spawn_player(mut commands: Commands, materials: Res<PlayerMaterials>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(config::SIZE.0, config::SIZE.1)),
            transform: Transform::from_xyz( 0.0, 0.0, 2.0),
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