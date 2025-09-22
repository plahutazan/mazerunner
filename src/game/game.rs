use bevy::prelude::*;
use crate::game::maze_reader::*;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Startup, load_maze)
            .add_systems(Update, print_maze_info);
    }
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
}

fn print_maze_info(maze: Res<MazeData>) {
    println!("Maze size: {}x{}", maze.width, maze.height);
    // Example: check if top-left cell is a wall
    if maze.grid[0] {
        println!("Top-left cell is a path");
    } else {
        println!("Top-left cell is a wall");
    }
}