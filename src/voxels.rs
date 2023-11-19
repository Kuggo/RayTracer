
use crate::linalg::*;

pub type Color = sdl2::pixels::Color;

// Materials
type MaterialID = u8;

// air is the default material
static AIR: Material = Material {color: Color { r: 0, g: 0, b: 0, a: 0 }};

static MATERIALS: [Material; 256] = [AIR; 256];

#[derive(Copy)]
#[derive(Clone)]
struct Material {
    color: Color,
    //refractiviness: f32,
    //metallicness: f32,
}



// Voxel data structures and algorithms

pub struct Octree {
    size: u8,
    material: MaterialID,
    children: [Option<Box<Octree>>; 8],
}

pub struct Chunk {
    tree: Octree,
}

pub struct World {
    chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> World {
        World {chunks: Vec::new()}
    }
}


pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn trace() -> Color {
        // TODO
        Color { r: 0, g: 0, b: 0, a: 0 }
    }
}