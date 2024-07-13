
use std::mem;
use rand::Rng;

use crate::linalg::*;



#[derive(Clone, Copy, Debug)]
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


#[derive(Clone, Copy, Debug)]
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
const RENDER_DISTANCE: usize = 2;

const CHUNK_MASK: usize = RENDER_DISTANCE - 1;
const VOXEL_MASK: usize = CHUNK_SIZE - 1;

#[derive(Clone, Copy, Debug)]
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


#[derive(Debug)]
pub struct World {
    chunks: [Option<Box<Chunk>>; RENDER_DISTANCE * RENDER_DISTANCE * RENDER_DISTANCE],
    coord_index: Pos,
}

impl World {
    pub fn new() -> Box<World> {
        Box::new(World {
            chunks: std::array::from_fn(|_| None),
            coord_index: Pos::new(0, 0, 0),
        })
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

    fn fetch_unloaded_chunk(&self, coords: Pos) -> Box<Chunk> {
        let was_generated = false;

        if was_generated {
            // TODO load from file
            let c = Chunk::new(coords);
            c
        }
        else {
            let mut c = Chunk::new(coords);
            c.random_gen();
            c
        }
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

    pub fn voxel_at(&self, pos: Pos) -> Option<MaterialID> {
        let chunk_coords = Pos::new(pos.x / CHUNK_SIZE as i32, pos.y / CHUNK_SIZE as i32, pos.z / CHUNK_SIZE as i32);
        let voxel_offset = Pos::new(pos.x % CHUNK_SIZE as i32, pos.y % CHUNK_SIZE as i32, pos.z % CHUNK_SIZE as i32);
        let chunk = &self.chunks[World::chunk_index(chunk_coords)];
        if let Some(chunk) = chunk {
            Some(chunk.get_voxel(voxel_offset.vec3()))
        }
        else {
            None
        }
    }

    pub fn update_chunks_in_area(&mut self, pos: Vec3) {
        const RD: i32 = RENDER_DISTANCE as i32;
        
        let pos = pos.pos();
        let index = pos.div(CHUNK_SIZE as i32);
        if index == self.coord_index { 
            return; // early return as no changes to loaded chunks
        }
        let offset = index.sub(&self.coord_index);

        let mut unloaded_chunks: Vec<Box<Chunk>> = Vec::new();

        if offset.x >= RD || offset.y >= RD || offset.z >= RD {
            unloaded_chunks.append(&mut self.unload_all_chunks());
            self.load_all_chunks(pos);
        }
        
        let corner = self.coord_index.sub(&Pos::new(RD/2, RD/2, RD/2));
        
        let range_x = if offset.x >= 0 { 0..offset.x } 
                                else { (RD + offset.x)..RD };
        for x in range_x {
            for y in 0..RENDER_DISTANCE {
                for z in 0..RENDER_DISTANCE {
                    let coords = Pos::new(x, y as i32, z as i32).sub(&corner);
                    let new = self.fetch_unloaded_chunk(coords);
                    let old = self.load_chunk(new);
                    if let Some(old) = old {
                        unloaded_chunks.push(old);
                    }
                }
            }
        }

        let range_y = if offset.y >= 0 { 0..offset.y } 
                                else { (RD + offset.y)..RD };
        for y in range_y {
            let range_x = if offset.x >= 0 { offset.x..RD } // feels like backwards but its correct
                                    else { 0..(RD + offset.x) };
            
            for x in range_x {
                for z in 0..RENDER_DISTANCE {
                    let coords = Pos::new(x, y, z as i32).sub(&corner);
                    let c = self.unload_chunk(coords);
                    if let Some(c) = c {
                        unloaded_chunks.push(c);
                    }
                }
            }
        }

        let range_z = if offset.z >= 0 { 0..offset.z } 
                                else { (RD + offset.z)..RD };
        for z in range_z {
            let range_x = if offset.x >= 0 { offset.x..RD }
                                    else { 0..(RD + offset.x) };
            
            for x in range_x {
                let range_y = if offset.y >= 0 { offset.y..RD }
                                        else { 0..(RD + offset.y) };
                for y in range_y {
                    let coords = Pos::new(x, y, z).sub(&corner);
                    let c = self.unload_chunk(coords);
                    if let Some(c) = c {
                        unloaded_chunks.push(c);
                    }
                }
            }
        }

        for c in unloaded_chunks {
            println!("{:?}\n", c.coords);
        }
        
        self.coord_index = index;    
    }

    fn unload_all_chunks(&mut self) -> Vec<Box<Chunk>> {
        let mut unloaded_chunks = Vec::new();
        for x in 0..RENDER_DISTANCE {
            for y in 0..RENDER_DISTANCE {
                for z in 0..RENDER_DISTANCE {
                    let coords = Pos::new(x as i32, y as i32, z as i32);
                    let c = self.unload_chunk(coords);
                    if let Some(c) = c {
                        unloaded_chunks.push(c);
                    }
                }
            }
        }
        unloaded_chunks
    }

    fn load_all_chunks(&mut self, pos: Pos) {
        let index = pos.div(CHUNK_SIZE as i32);
        self.coord_index = index;
        for x in 0..RENDER_DISTANCE {
            for y in 0..RENDER_DISTANCE {
                for z in 0..RENDER_DISTANCE {
                    let coords = Pos::new(x as i32, y as i32, z as i32);
                    let chunk = Chunk::new(coords);
                    self.load_chunk(chunk);
                }
            }
        }
    }

}



pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin, direction: direction.normalize()}
    }

    pub fn trace(&self, world: &World, bounces_left: u8) -> Color {
        let mut pos = self.origin;
        while pos.manhattan(&self.origin) < (RENDER_DISTANCE * CHUNK_SIZE) as f32 {
            let voxel = world.voxel_at(pos.pos());
            if let Some(voxel) = voxel {
                let color = Material::from_id(voxel).color;
                
                if color.a != 0.0 {
                    if color.a == 1.0 || bounces_left <= 0 {
                        return color;
                    }
                    
                    let ray = Ray::new(self.origin, self.direction);
                    let color = ray.trace(world, bounces_left - 1);
                    
                    return color.weight_mix(color);
                }
            }
            else { break; }

            pos = pos.add(&self.direction);
        }

        Materials::Air.get_properties().color
    }
    
    /*
    pub fn trace(&self, world: &World, bounces_left: u8) -> Color {
        let step_size_frac = Vec3::new(
            (1.0 / self.direction.x).abs(), 
            (1.0 / self.direction.y).abs(), 
            (1.0 / self.direction.z).abs()
        );
        
        let step_sign = Pos::new(
            self.direction.x.signum() as i32, 
            self.direction.y.signum() as i32, 
            self.direction.z.signum() as i32
        );

        let mut vox = self.origin.pos();
        let mut vox_frac: Vec3 = Vec3::new(
            self.direction.x.signum() * (self.origin.x - (vox.x + (self.direction.x < 0.0) as i32) as f32) * step_size_frac.x,
            self.direction.x.signum() * (self.origin.y - (vox.y + (self.direction.y < 0.0) as i32) as f32) * step_size_frac.y,
            self.direction.z.signum() * (self.origin.z - (vox.z + (self.direction.z < 0.0) as i32) as f32) * step_size_frac.z
        );
        
        let mut dist: f32 = 0.0;

        while dist < (RENDER_DISTANCE * CHUNK_SIZE) as f32 {
            let voxel = world.voxel_at(vox);
            if let Some(voxel) = voxel {
                let color = Material::from_id(voxel).color;
                
                if color.a != 0.0 {
                    if color.a == 1.0 || bounces_left <= 0 {
                        return color;
                    }
                    
                    let ray = Ray::new(vox_frac, self.direction);
                    let color = ray.trace(world, bounces_left - 1);
                    
                    return color.weight_mix(color);
                }
            }
            else { break; }

            if vox_frac.x < vox_frac.y {
                if vox_frac.x < vox_frac.z {
                    vox.x += step_sign.x;
                    dist = vox_frac.x;
                    vox_frac.x += step_size_frac.x;
                } 
                else {
                    vox.z += step_sign.z;
                    dist = vox_frac.z;
                    vox_frac.z += step_size_frac.z;
                }
            } 
            else {
                if vox_frac.y < vox_frac.z {
                    vox.y += step_sign.y;
                    dist = vox_frac.y;
                    vox_frac.y += step_size_frac.y;
                } 
                else {
                    vox.z += step_sign.z;
                    dist = vox_frac.z;
                    vox_frac.z += step_size_frac.z;
                }
            }
        }
        Materials::Air.get_properties().color
    }
    */
}