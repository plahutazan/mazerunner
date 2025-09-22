// maze_gen.rs
use bitvec::prelude::*;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use image::{RgbImage, Rgb};


#[derive(Serialize, Deserialize)]
struct Maze {
    width: usize,
    height: usize,
    // Use Vec<u8> to save bits in JSON (BitVec → Vec<u8>)
    grid_bytes: Vec<usize>,
}

impl Maze {
    fn from_bitvec(width: usize, height: usize, grid: &BitVec) -> Self {
        Self {
            width,
            height,
            grid_bytes: grid.clone().into_vec(), // Convert BitVec to Vec<u8> for JSON
        }
    }
}

#[derive(Clone, Copy)]
struct Cell { visited: bool }
impl Cell { fn new() -> Self { Self { visited: false } } }

fn create_grid(width: usize, height: usize) -> Vec<Vec<Cell>> {
    vec![vec![Cell::new(); width]; height]
}

fn generate_maze(grid: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    grid[y][x].visited = true;

    let mut directions = vec!["up","down","left","right"];
    directions.shuffle(&mut rand::rng());

    for dir in directions {
        match dir {
            "up" => if y > 1 && !grid[y-2][x].visited { grid[y-1][x].visited = true; generate_maze(grid,x,y-2); }
            "down" => if y < grid.len()-2 && !grid[y+2][x].visited { grid[y+1][x].visited = true; generate_maze(grid,x,y+2); }
            "left" => if x > 1 && !grid[y][x-2].visited { grid[y][x-1].visited = true; generate_maze(grid,x-2,y); }
            "right"=> if x < grid[0].len()-2 && !grid[y][x+2].visited { grid[y][x+1].visited = true; generate_maze(grid,x+2,y); }
            _ => {}
        }
    }
}

fn grid_to_bitvec(grid: &Vec<Vec<Cell>>) -> BitVec {
    let height = grid.len();
    let width = grid[0].len();
    let mut bv = bitvec![0; width*height];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x].visited { bv.set(y*width + x, true); }
        }
    }
    bv
}

#[allow(dead_code)]
fn draw_maze(grid: &Vec<Vec<Cell>>, cell_size: u32, filename: &str) {
    let width = grid[0].len() as u32;
    let height = grid.len() as u32;
    let mut img = RgbImage::new(width * cell_size, height * cell_size);

    // Colors
    let black = Rgb([0, 0, 0]);       // walls
    let white = Rgb([255, 255, 255]); // paths
    let green = Rgb([0, 255, 0]);     // finish

    // Finish cell = bottom-right (inside the maze, not on border)
    let finish = (width as usize - 2, height as usize - 2);

    for y in 0..height {
        for x in 0..width {
            let mut color = if grid[y as usize][x as usize].visited {
                white
            } else {
                black
            };

            // Paint finish cell green
            if (x as usize, y as usize) == finish {
                color = green;
            }

            // Paint as block
            for dy in 0..cell_size {
                for dx in 0..cell_size {
                    img.put_pixel(x * cell_size + dx, y * cell_size + dy, color);
                }
            }
        }
    }

    img.save(filename).unwrap();
}

pub fn main() {
    let mut width = 8; // must be odd
    if width % 2 == 0 { width += 1; } // ensure odd
    let height = width.clone(); // square maze

    let mut grid = create_grid(width, height);
    generate_maze(&mut grid, 1, 1);

    let bv = grid_to_bitvec(&grid);
    let maze = Maze::from_bitvec(width, height, &bv);

    // Save as JSON
    let json = serde_json::to_string(&maze).unwrap();
    let mut file = File::create("maze.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    // Save Image
    //let cell_size = 25;
    //draw_maze(&grid, cell_size, "maze.png");

    println!("Maze created: {}x{}", width, height);
}