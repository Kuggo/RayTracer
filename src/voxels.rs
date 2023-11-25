
use std::mem;
use rand::Rng;

use crate::linalg::*;



#[derive(Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as f32 / 255.0, 
            g: g as f32 / 255.0, 
            b: b as f32 / 255.0, 
            a: a as f32 / 255.0
        }
    }

    pub fn sdl_format(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGBA(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8, 
            (self.b * 255.0) as u8, 
            (self.a * 255.0) as u8
        )
    }

    pub fn from_sdl(color: sdl2::pixels::Color) -> Color {
        Color {
            r: color.r as f32 / 255.0, 
            g: color.g as f32 / 255.0, 
            b: color.b as f32 / 255.0, 
            a: color.a as f32 / 255.0
        }
    }

    pub fn weight_mix(&self, other: Color) -> Color {
        let c = self.a + other.a;
        let a1 = self.a / c;
        let a2 = other.a / c;
        Color {
            r: self.r * a1 + other.r * a2,
            g: self.g * a1 + other.g * a2,
            b: self.b * a1 + other.b * a2,
            a: c,
        }
    }
}



// Materials
pub type MaterialID = u8;

static MATERIALS: [Material; 256] = Materials::init_pallete();


#[derive(Clone, Copy)]
pub struct Material {
    color: Color,
    //reflectiveness: f32,
    //refractiviness: f32,
    //metallicness: f32,
}

impl Material {
    pub fn from_id(id: MaterialID) -> Material {
        MATERIALS[id as usize]
    }
}


#[derive(Clone, Copy, Debug)]
pub enum Materials {
    Air,
    Stone,
    Dirt,
    Grass,
    Water,
    Sand,
    Wood,
}

impl Materials {
    pub fn get_properties(self) -> Material {
        MATERIALS[self as usize]
    }

    pub const fn init_pallete() -> [Material; 256] {
        let air: Material = Material {color: Color {r: 0.0, g: 0.0, b: 0.0, a: 0.0}};
        let mut materials = [air; 256];
        materials[Materials::Air as usize] = Material {color: Color {r: 0.0, g: 0.0, b: 0.0, a: 0.0}};
        materials[Materials::Stone as usize] = Material {color: Color {r: 0.5725, g: 0.5569, b: 0.5216, a: 1.0}};
        materials[Materials::Dirt as usize] = Material {color: Color {r: 0.5451, g: 0.2706, b: 0.0745, a: 1.0}};
        materials[Materials::Grass as usize] = Material {color: Color {r: 0.0, g: 1.0, b: 0.0, a: 1.0}};
        materials[Materials::Water as usize] = Material {color: Color {r: 0.0, g: 0.0, b: 1.0, a: 1.0}};
        materials[Materials::Sand as usize] = Material {color: Color {r: 1.0, g: 1.0, b: 0.0, a: 1.0}};
        materials[Materials::Wood as usize] = Material {color: Color {r: 0.8471, g: 0.7098, b: 0.5373, a: 1.0}};
        
        materials
    }
}



// Voxel data structures and algorithms
/* 
pub struct Octree {
    size: u16,
    material: MaterialID,
    children: [Option<Box<Octree>>; 8],
}
*/


const CHUNK_SIZE: usize = 8;
const RENDER_DISTANCE: usize = 8;

const CHUNK_MASK: usize = RENDER_DISTANCE - 1;
const VOXEL_MASK: usize = CHUNK_SIZE - 1;

#[derive(Clone, Copy)]
pub struct Chunk {
    //tree: Octree,
    coords: Pos,
    voxels: [MaterialID; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE],
}

impl Chunk {
    pub fn new(coords: Pos) -> Box<Chunk> {
        Box::new(Chunk {coords, voxels: [0; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE]})
    }

    pub fn random_gen(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..CHUNK_SIZE {
            let x = rng.gen_range(0..CHUNK_SIZE);
            let y = rng.gen_range(0..CHUNK_SIZE);
            let z = rng.gen_range(0..CHUNK_SIZE);
            let index = ((z * CHUNK_SIZE) + y) * CHUNK_SIZE + x;
            self.voxels[index] = Materials::Stone as MaterialID;
        }
    }

    fn get_voxel_index(coords: Vec3) -> usize {
        let x: usize = coords.x as usize & VOXEL_MASK;
        let y: usize = coords.y as usize & VOXEL_MASK;
        let z: usize = coords.z as usize & VOXEL_MASK;
        let index = ((z * CHUNK_SIZE) + y) * CHUNK_SIZE + x;
        index
    }

    pub fn get_voxel(&self, coords: Vec3) -> MaterialID {
        let index = Chunk::get_voxel_index(coords);
        self.voxels[index]
    }

    pub fn set_voxel(&mut self, coords: Vec3, material: MaterialID) {
        let index = Chunk::get_voxel_index(coords);
        self.voxels[index] = material;
    }

}



pub struct World {
    chunks: [Option<Box<Chunk>>; RENDER_DISTANCE * RENDER_DISTANCE * RENDER_DISTANCE],
}

impl World {
    pub fn new() -> Box<World> {
        Box::new(World {chunks: std::array::from_fn(|_| None)})
    }

    pub fn random_gen(&mut self) {
        for x in 0..RENDER_DISTANCE {
            for y in 0..RENDER_DISTANCE {
                for z in 0..RENDER_DISTANCE {
                    let mut chunk = Chunk::new(Pos::new(x as i32, y as i32, z as i32));
                    chunk.random_gen();
                    self.load_chunk(chunk);
                }
            }
        }
    }

    fn chunk_index(coords: Pos) -> usize {
        let x: usize = coords.x as usize & CHUNK_MASK;
        let y: usize = coords.y as usize & CHUNK_MASK;
        let z: usize = coords.z as usize & CHUNK_MASK;
        let index = (z * RENDER_DISTANCE + y) * RENDER_DISTANCE + x;
        index
    }

    pub fn load_chunk(&mut self, chunk: Box<Chunk>) -> Option<Box<Chunk>> {
        let index = World::chunk_index(chunk.coords);

        let old_chunk = mem::replace(&mut self.chunks[index], Some(chunk));

        old_chunk
    }

    pub fn unload_chunk(&mut self, coords: Pos) -> Option<Box<Chunk>> {
        let index = World::chunk_index(coords);

        let old = mem::replace(&mut self.chunks[index], None);

        old
    }
}



pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn trace(&self, world: &World, bounces_left: u8) -> Color {
        let mut pos = self.origin;
        while pos.manhattan(&self.origin) < (RENDER_DISTANCE * CHUNK_SIZE) as f32 {
            let chunk = &world.chunks[World::chunk_index(pos.pos())];
            if let Some(chunk) = chunk {
                let material = Material::from_id(chunk.get_voxel(pos));
                
                if material.color.a == 0.0 {
                    pos = pos.add(&self.direction);
                    continue;
                }
                
                if material.color.a == 1.0 || bounces_left <= 0 {
                    return material.color;
                }
                
                let ray = Ray::new(pos, self.direction);
                let color = ray.trace(world, bounces_left - 1);
                
                let final_color = material.color.weight_mix(color);
                return final_color;
            }
            else {
                break;
            }
        }

        Materials::Air.get_properties().color
    }
}