use bevy::prelude::*;

use maze_generator::prelude::*;
use maze_generator::recursive_backtracking::RbGenerator;

use crate::maze::config::*;

#[allow(dead_code)]
mod config {
    use super::*;

    // Colors
    pub const WALL_COLOR: Color = Color::BLACK;
    pub const NORMAL_COLOR: Color = Color::AZURE;
    pub const START_COLOR: Color = Color::GREEN;
    pub const END_COLOR: Color = Color::RED;
    pub const GRID_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

    // Options
    pub const SHOW_GRID: bool = false;

    // Sizes
    pub const MAZE_SIZE: (u8, u8) = (10u8, 10u8);
    pub const CELL_SIZE: f32 = 50.0;
    pub const MINOR_LINE_THICKNESS: f32 = 1.0;
    pub const MAJOR_LINE_THICKNESS: f32 = 4.0;

    pub const MAZE_BOARD_SIZE: (f32, f32) = ( MAZE_SIZE.0 as f32 * CELL_SIZE, MAZE_SIZE.1 as f32 * CELL_SIZE);
    pub const MAZE_BOARD_LEFT_EDGE: f32 = - 0.5 * MAZE_BOARD_SIZE.0 as f32 ;
    pub const MAZE_BOARD_BOT_EDGE: f32 = -0.5 * MAZE_BOARD_SIZE.1 as f32;
}

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // ASSETS
            .init_resource::<MazeGridShow>()
            .init_resource::<MazeSize>()
            .init_resource::<MazeMaterials>()
            // SETUP
            //.add_startup_system_to_stage(StartupStage::PreStartup, spawn_cells.system())
            .add_startup_system(spawn_grid.system())
            .add_startup_system(spawn_maze.system());
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
    x: u8,
    y: u8,
}

impl Default for MazeSize {
    fn default() -> Self {
        MazeSize {
            x: config::MAZE_SIZE.0,
            y: config::MAZE_SIZE.1,
        }
    }
}

pub struct MazeGridShow{
    show: bool,
}

impl Default for MazeGridShow {
    fn default() -> Self {
        MazeGridShow {
            show: SHOW_GRID,
        }
    }
}
pub struct MazeMaterials {
    wall_material: Handle<ColorMaterial>,
    normal_material: Handle<ColorMaterial>,
    start_material: Handle<ColorMaterial>,
    end_material: Handle<ColorMaterial>,
    grid_material: Handle<ColorMaterial>,
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
            grid_material: materials.add(config::GRID_COLOR.into()),
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

pub fn spawn_grid(mut commands: Commands, materials: Res<MazeMaterials>, size: Res<MazeSize>, grid_show: Res<MazeGridShow>) {

    if grid_show.show {
    for row in 0..=size.x {
        commands.spawn_bundle(new_gridline(
            Orientation::Horizontal,
            row,
            materials.grid_material.clone(),
            row == 0 || row == size.x,
        ));
    }

    for column in 0..=size.y {
        commands.spawn_bundle(new_gridline(
            Orientation::Vertical,
            column,
            materials.grid_material.clone(),
            column == 0 || column == size.y,
        ));
    }
    }
}

fn new_gridline(
    orientation: Orientation,
    i: u8,
    grid_handle: Handle<ColorMaterial>,
    edge: bool,
) -> SpriteBundle {
    use config::*;

    // The grid lines that define the boxes need to be thicker
    let thickness = if edge {
        MAJOR_LINE_THICKNESS
    } else {
        MINOR_LINE_THICKNESS
    };

    let size = match orientation {
        Orientation::Horizontal => Vec2::new(MAZE_BOARD_SIZE.0 as f32 + thickness, thickness),
        Orientation::Vertical => Vec2::new(thickness, MAZE_BOARD_SIZE.1 as f32 + thickness),
    };

    // Each objects' position is defined by its center
    let offset = i as f32 * CELL_SIZE;

    let (x, y) = match orientation {
        Orientation::Horizontal => ( MAZE_BOARD_LEFT_EDGE + 0.5 * MAZE_BOARD_SIZE.0 as f32, MAZE_BOARD_BOT_EDGE + offset),
        Orientation::Vertical => (MAZE_BOARD_LEFT_EDGE + offset, MAZE_BOARD_BOT_EDGE + 0.5 * MAZE_BOARD_SIZE.1 as f32),
    };

    SpriteBundle {
        sprite: Sprite::new(size),
        // We want these grid lines to cover any cell that it might overlap with
        transform: Transform::from_xyz(x, y, 1.0),
        material: grid_handle,
        ..Default::default()
    }
}

enum Cell {
    Normal,
    Wall,
    Start,
    End
}

fn spawn_maze(mut commands: Commands, materials: Res<MazeMaterials>, size: Res<MazeSize>) {

    let mut generator = RbGenerator::new(Some([42; 32]));
    let maze = generator.generate(size.x as i32, size.y as i32);

    for x in 0..size.x {
        for y in 0..size.y {
            let coord = Coordinates { x: x as i32, y: y as i32};
            let field = maze.get_field( &coord).unwrap();
            let sprite = Sprite::new(Vec2::splat(config::CELL_SIZE));
            let trans = Transform::from_xyz(
                MAZE_BOARD_LEFT_EDGE + (CELL_SIZE * x as f32) + CELL_SIZE * 0.5 ,
                MAZE_BOARD_BOT_EDGE + (CELL_SIZE * y as f32) + CELL_SIZE * 0.5,
                0f32);

            let (material, cell) = match field.field_type {
                FieldType::Start => {
                    ( materials.start_material.clone(), Cell::Start)
                }
                FieldType::Goal => {
                    (materials.end_material.clone(), Cell::End)
                },
                FieldType::Normal => {
                    (materials.normal_material.clone(), Cell::Normal)
                }
            };

            let cell = commands.spawn_bundle(SpriteBundle {
                sprite: sprite,
                // We want these grid lines to cover any cell that it might overlap with
                material: material,
                transform: trans,
                ..Default::default()
               })
               .insert(cell)
               .id();

            // Draw walls for the cell
            if !field.has_passage(&maze_generator::prelude::Direction::North) {
                spawn_wall(&mut commands, cell, materials.wall_material.clone(), &maze_generator::prelude::Direction::North)
            }
            if !field.has_passage(&maze_generator::prelude::Direction::South) {
                spawn_wall(&mut commands, cell, materials.wall_material.clone(), &maze_generator::prelude::Direction::South)
            }
            if !field.has_passage(&maze_generator::prelude::Direction::East) {
                spawn_wall(&mut commands, cell, materials.wall_material.clone(), &maze_generator::prelude::Direction::East)
            }
            if !field.has_passage(&maze_generator::prelude::Direction::West) {
                spawn_wall(&mut commands, cell, materials.wall_material.clone(), &maze_generator::prelude::Direction::West)
            }

        }
    }
}

fn spawn_wall(commands: &mut Commands, parent: Entity,  material: Handle<ColorMaterial>, direction: &maze_generator::prelude::Direction) {

    // Draw the walls,
    // TODO: direction feels reversed, should refactor this
    let (x, y, size) = match direction {
        maze_generator::prelude::Direction::North => {
            (0f32, CELL_SIZE * -0.5, Vec2::new(CELL_SIZE + MAJOR_LINE_THICKNESS * 0.5, MAJOR_LINE_THICKNESS ))
        },
        maze_generator::prelude::Direction::South => {
            ( 0f32, CELL_SIZE * 0.5, Vec2::new( CELL_SIZE + MAJOR_LINE_THICKNESS * 0.5, MAJOR_LINE_THICKNESS,))
        },
        maze_generator::prelude::Direction::East => {
            ( CELL_SIZE * 0.5, 0f32, Vec2::new(MAJOR_LINE_THICKNESS, CELL_SIZE + MAJOR_LINE_THICKNESS * 0.5))
        },
        maze_generator::prelude::Direction::West => {
            ( CELL_SIZE * -0.5, 0f32, Vec2::new(MAJOR_LINE_THICKNESS, CELL_SIZE+ MAJOR_LINE_THICKNESS * 0.5))
        },
    };

    let wall = commands.spawn_bundle(SpriteBundle {
        sprite: Sprite::new(size),
        // We want these grid lines to cover any cell that it might overlap with
        material: material,
        transform: Transform::from_xyz( x, y, 1.0),
        ..Default::default()
       }).insert(Cell::Wall)
       .id();

       // add the child to the parent
    commands.entity(parent).push_children(&[wall]);
}

