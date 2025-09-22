use bevy::prelude::*;
use bitvec::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::asset::RenderAssetUsages;
use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Maze {
    width: usize,
    height: usize,
    grid_bytes: Vec<usize>,
}

#[derive(Resource)]
pub struct MazeData {
    pub width: usize,
    pub height: usize,
    pub grid: BitVec,
}

fn parse_maze(json: &str) -> (usize, usize, BitVec) {
    // Deserialize JSON string
    let maze: Maze = serde_json::from_str(json).unwrap();

    // Reconstruct BitVec from grid_bytes
    let mut bv = BitVec::<usize, Lsb0>::from_vec(maze.grid_bytes.clone());

    // Ensure it matches width*height exactly
    bv.truncate(maze.width * maze.height);

    (maze.width, maze.height, bv)
}

fn bitvec_to_image(width: usize, height: usize, bv: &BitVec) -> Image {
    let mut pixels = Vec::with_capacity(width * height * 4);

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;

            // Decide color: wall = black, path = white
            let (r, g, b) = if bv[idx] { (255, 255, 255) } else { (0, 0, 0) };

            pixels.extend_from_slice(&[r, g, b, 255]); // RGBA
        }
    }

    let size = Extent3d {
        width: width as u32,
        height: height as u32,
        depth_or_array_layers: 1,
    };

    let image = Image::new_fill(size, TextureDimension::D2, &pixels, TextureFormat::Rgba8UnormSrgb, RenderAssetUsages::all());

    image
}

fn spawn_maze(commands: &mut Commands, images: &mut ResMut<Assets<Image>>, image: Image) {
    let handle = images.add(image);

    commands.spawn((
        Sprite::from_image(handle),
        Transform::from_scale(Vec3::splat(20.0)), // scale to make it visible
        GlobalTransform::default(),
    ));
}

pub fn load_maze(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Example: receive JSON from server
    let json_str = std::fs::read_to_string("maze.json").expect("Failed to read maze.json");
    let json: &str = &json_str;
    // Parse
    let (w, h, bv) = parse_maze(json);
    commands.insert_resource(MazeData {
        width: w,
        height: h,
        grid: bv.clone(),
    });

    // Generate texture
    let image = bitvec_to_image(w, h, &bv);

    // Spawn maze
    spawn_maze(&mut commands, &mut images, image);
}