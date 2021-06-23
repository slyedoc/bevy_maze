use bevy::prelude::*;

use maze_generator::prelude::{Coordinates, Direction, FieldType, Generator};
use maze_generator::recursive_backtracking::RbGenerator;

use crate::AppState;
use crate::cleanup::cleanup_system;

pub mod config {
    use super::*;

    // Colors
    pub const WALL_COLOR: Color = Color::BLACK;
    pub const NORMAL_COLOR: Color = Color::AZURE;
    pub const START_COLOR: Color = Color::GREEN;
    pub const END_COLOR: Color = Color::RED;

    // Sizes
    pub const WORLD_SCALE: f32 = 600.0;

}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // ASSETS
            .init_resource::<MazeMaterials>()
            // SETUP
            .add_system_set(
                SystemSet::on_enter(AppState::Playing)
                .with_system(spawn_maze.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Playing)
                    .with_system(cleanup_system::<Cell>.system())
                    .with_system(cleanup_system::<Wall>.system())
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

pub struct MazeSize {
    pub x: u8,
    pub y: u8,
}

pub struct MazeMaterials {
    wall_material: Handle<ColorMaterial>,
    normal_material: Handle<ColorMaterial>,
    start_material: Handle<ColorMaterial>,
    end_material: Handle<ColorMaterial>,
}

impl FromWorld for MazeMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");
        MazeMaterials {
            wall_material: materials.add(config::WALL_COLOR.into()),
            normal_material: materials.add(config::NORMAL_COLOR.into()),
            start_material: materials.add(config::START_COLOR.into()),
            end_material: materials.add(config::END_COLOR.into()),
        }
    }
}

enum Cell {
    Normal,
    Start,
    End,
}

struct Wall;

fn spawn_maze(mut commands: Commands, materials: Res<MazeMaterials>, size: Res<MazeSize>) {
    let seed = Some([3; 32]);
    let mut generator = RbGenerator::new(seed);
    let mut maze = generator.generate(size.x as i32, size.y as i32);

    maze.start = Coordinates { x: 0, y: 0 };
    maze.goal = Coordinates {
        x: size.x as i32 - 1,
        y: size.y as i32 - 1,
    };

    // Sizes
    let cell_size: f32 = config::WORLD_SCALE / size.x as f32;


    // These are generated for positioning in the center of the world
    let maze_world_size: (f32, f32) = (
        size.x as f32 * cell_size,
        size.y as f32 * cell_size,
    );
    let maze_world_left_edge: f32 = -0.5 * maze_world_size.0 as f32;
    let maze_world_bot_edge: f32 = -0.5 * maze_world_size.1 as f32;


    for x in 0..size.x {
        for y in 0..size.y {
            let coord = Coordinates {
                x: x as i32,
                y: y as i32,
            };
            let field = maze.get_field(&coord).unwrap();
            let sprite = Sprite::new(Vec2::splat(cell_size));
            let trans = Transform::from_xyz(
                maze_world_left_edge + (cell_size * x as f32) + cell_size * 0.5,
                maze_world_bot_edge + (cell_size * y as f32) + cell_size * 0.5,
                0f32,
            );

            let (material, cell) = match field.field_type {
                FieldType::Start => (materials.start_material.clone(), Cell::Start),
                FieldType::Goal => (materials.end_material.clone(), Cell::End),
                FieldType::Normal => (materials.normal_material.clone(), Cell::Normal),
            };

            let cell = commands
                .spawn_bundle(SpriteBundle {
                    sprite: sprite,
                    // We want these grid lines to cover any cell that it might overlap with
                    material: material,
                    transform: trans,
                    ..Default::default()
                })
                .insert(cell)
                .id();

            // Draw walls for the cell
            if !field.has_passage(&Direction::North) {
                spawn_wall(
                    &mut commands,
                    cell,
                    materials.wall_material.clone(),
                    &Direction::North,
                    cell_size
                )
            }
            if !field.has_passage(&Direction::South) {
                spawn_wall(
                    &mut commands,
                    cell,
                    materials.wall_material.clone(),
                    &Direction::South,
                    cell_size
                )
            }
            if !field.has_passage(&Direction::East) {
                spawn_wall(
                    &mut commands,
                    cell,
                    materials.wall_material.clone(),
                    &Direction::East,
                    cell_size
                )
            }
            if !field.has_passage(&Direction::West) {
                spawn_wall(
                    &mut commands,
                    cell,
                    materials.wall_material.clone(),
                    &Direction::West,
                    cell_size
                )
            }
        }
    }
}

// Draws a wall on a cell given a direction
fn spawn_wall(
    commands: &mut Commands,
    parent: Entity,
    material: Handle<ColorMaterial>,
    direction: &Direction,
    cell_size: f32,
) {

    let MINOR_LINE_THICKNESS: f32 = cell_size * 0.01;
    let MAJOR_LINE_THICKNESS: f32 = cell_size * 0.1;

    // TODO: direction feels reversed but it looks right, something is getting flipped or my understanding is off, look into this
    let (x, y, size) = match direction {
        Direction::North => (
            0f32,
            cell_size * -0.5,
            Vec2::new(cell_size + MAJOR_LINE_THICKNESS * 0.5, MAJOR_LINE_THICKNESS),
        ),
        Direction::South => (
            0f32,
            cell_size * 0.5,
            Vec2::new(cell_size + MAJOR_LINE_THICKNESS * 0.5, MAJOR_LINE_THICKNESS),
        ),
        Direction::East => (
            cell_size * 0.5,
            0f32,
            Vec2::new(MAJOR_LINE_THICKNESS, cell_size + MAJOR_LINE_THICKNESS * 0.5),
        ),
        Direction::West => (
            cell_size * -0.5,
            0f32,
            Vec2::new(MAJOR_LINE_THICKNESS, cell_size + MAJOR_LINE_THICKNESS * 0.5),
        ),
    };

    let wall = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            // We want these grid lines to cover any cell that it might overlap with
            material: material,
            transform: Transform::from_xyz(x, y, 1.0),
            ..Default::default()
        })
        .insert(Wall)
        .id();

    // add the child to the parent
    commands.entity(parent).push_children(&[wall]);
}
