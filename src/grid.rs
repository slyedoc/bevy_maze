use bevy::prelude::*;

use crate::{AppState, maze::MazeSize};

use crate::cleanup::*;


pub mod config {
    use super::*;

    pub const GRID_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GridState {
    Ready,
    Reset
}


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // ASSETS
            .init_resource::<GridMaterials>()
            // SETUP
            .add_system_set(
                SystemSet::on_enter(AppState::Menu)
                    .with_system(spawn_grid.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Menu)
                    .with_system(cleanup_system::<GridLine>.system())
            )
            // Local State, used for reseting
            .add_state(GridState::Ready)
            .add_system_set(
                SystemSet::on_enter(GridState::Reset)
                    .with_system(cleanup_system::<GridLine>.system())
                    .with_system(spawn_grid.system())
                    .with_system(reset_state.system())

            );

        // ACTION HANDLING
        // .add_system_set(
        //     SystemSet::new()
        //         .after(CommonLabels::Action)
        //         .with_system(actions::color_selected.system())
        //         .with_system(actions::update_cell_numbers.system())
        //         .with_system(actions::style_numbers.system()),
        // );
    }
}

fn reset_state(mut state: ResMut<State<GridState>>) {
    state.set(GridState::Ready).unwrap();
}


pub struct GridMaterials {
    grid_material: Handle<ColorMaterial>,
}
impl FromWorld for GridMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");
        GridMaterials {
            grid_material: materials.add(config::GRID_COLOR.into()),
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

struct GridLine;

pub fn spawn_grid(mut commands: Commands, materials: Res<GridMaterials>, size: Res<MazeSize>) {

    for row in 0..=size.x {
        commands.spawn_bundle(new_gridline(
            Orientation::Horizontal,
            row,
            materials.grid_material.clone(),
            row == 0 || row == size.x,
            (size.x, size.y)
        )).insert(GridLine);
    }

    for column in 0..=size.y {
        commands.spawn_bundle(new_gridline(
            Orientation::Vertical,
            column,
            materials.grid_material.clone(),
            column == 0 || column == size.y,
            (size.x, size.y)
        )).insert(GridLine);
    }
}

fn new_gridline(
    orientation: Orientation,
    i: u8,
    grid_handle: Handle<ColorMaterial>,
    edge: bool,
    size: (u8, u8)
) -> SpriteBundle {

    let max = std::cmp::max(size.0, size.1);
    let cell_size =  600.0 / max as f32;


    // Sizes
    

    // These are generated for positioning in the center of the world
    let grid_world_size: (f32, f32) = (size.0 as f32 * cell_size, size.1 as f32 * cell_size);
    let grid_world_left_edge: f32 = -0.5 * grid_world_size.0 as f32;
    let grid_world_bot_edge: f32 = -0.5 * grid_world_size.1 as f32;

    // The grid lines that define the boxes need to be thicker
    let thickness = if edge {
        cell_size * 0.1
    } else {
        cell_size * 0.01
    };

    // Each objects' position is defined by its center

    let offset = i as f32 * cell_size;

    let (x, y, sprite_size) = match orientation {
        Orientation::Horizontal => (
            grid_world_left_edge + 0.5 * grid_world_size.1 as f32,
            grid_world_bot_edge + offset,
            Vec2::new(grid_world_size.1 as f32 + thickness, thickness),
        ),
        Orientation::Vertical => (
            grid_world_left_edge + offset,
            grid_world_bot_edge + 0.5 * grid_world_size.0 as f32,
            Vec2::new(thickness, grid_world_size.0 as f32 + thickness),
        ),
    };

    SpriteBundle {
        sprite: Sprite::new(sprite_size),
        // We want these grid lines to cover any cell that it might overlap with
        transform: Transform::from_xyz(x, y, 1.0),
        material: grid_handle,
        ..Default::default()
    }
}
