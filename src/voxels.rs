
pub use sdl2::pixels::Color;



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


// Linear algebra and Rays

pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.x / l, self.y / l, self.z / l)
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
