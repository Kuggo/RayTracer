

const FP_TOLERANCE: f32 = 0.0001;

// Directions
pub const X_AXIS: Vec3 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
pub const Y_AXIS: Vec3 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
pub const Z_AXIS: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};



// Linear algebra and Rays
#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }
    
    pub fn pos(&self) -> Pos {
        Pos::new(self.x as i32, self.y as i32, self.z as i32)
    }

    pub fn from_polar(len: f32, pitch: f32, yaw: f32) -> Vec3 {
        Vec3::new(
            len * pitch.cos() * yaw.cos(),
            len * pitch.sin(),
            len * pitch.cos() * yaw.sin())
    }
    
    pub fn null(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

    pub fn fp_equals(&self, other: &Vec3) -> bool {
        (self.x - other.x).abs() < FP_TOLERANCE &&
        (self.y - other.y).abs() < FP_TOLERANCE &&
        (self.z - other.z).abs() < FP_TOLERANCE

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

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn manhattan(&self, other: &Vec3) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
    
    pub fn normalize(&self) -> Vec3 {
        let l = self.length();
        if l == 0.0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        Vec3::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn polar(&self) -> (f32, f32, f32) {
        let len = self.length();
        if len == 0.0 {
            return (0.0, 0.0, 0.0);
        }
        let pitch = (self.y / len).asin();
        let yaw = self.y.atan2(self.x);
        (len, pitch, yaw)
    }

    pub fn colinear(&self, other: &Vec3) -> bool {
        let l1 = self.length();
        let l2 = other.length();
        if l1 == 0.0 || l2 == 0.0 {
            return true;
        }
        let n1 = self.scale(1.0 / l1);
        let n2 = other.scale(1.0 / l2);
        n1.fp_equals(&n2)
    }

    pub fn angle(&self, other: &Vec3) -> f32 {
        if self.null() || other.null() {
            return 0.0;
        }
        (self.dot(&other) / (self.length() * other.length())).acos() 
    }

    pub fn project_onto(&self, other: &Vec3) -> Vec3 {
        let l = other.length();
        if l == 0.0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        other.scale(self.dot(other) / l)
    }

    pub fn get_ortho(&self, other: &Vec3) -> Vec3 {
        self.sub(&self.project_onto(&other))
    }

    pub fn rotate_around(&self, other: &Vec3, angle: f32) -> Vec3 {
        let paralel = self.project_onto(other);
        let ortho = self.get_ortho(other);
        let axis = other.cross(&ortho);
        ortho.scale(angle.cos()).add(&axis.scale(angle.sin())).add(&paralel)
    }

    pub fn rotate_to_plane(self, point: Vec3) -> Vec3{
        if self.null() {
            return Vec3::new(0.0, 0.0, 0.0);
        }
            
        let (_, pitch, yaw) = self.polar();
        point.rotate_yz(-pitch).rotate_xz(-yaw)
    } 
    
    pub fn rotate_yz(&self, angle: f32) -> Vec3 {
        Vec3::new(
            self.x,
            self.y * angle.cos() - self.z * angle.sin(),
            self.y * angle.sin() + self.z * angle.cos()
        )
    }

    pub fn rotate_xz(&self, angle: f32) -> Vec3 {
        Vec3::new(
            self.x * angle.cos() + self.z * angle.sin(),
            self.y,
            -self.x * angle.sin() + self.z * angle.cos()
        )
    }

    pub fn rotate_xy(&self, angle: f32) -> Vec3 {
        Vec3::new(
            self.x * angle.cos() - self.y * angle.sin(),
            self.x * angle.sin() + self.y * angle.cos(),
            self.z
        )
    }
}


#[derive(Debug, Copy, PartialEq, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32, z: i32) -> Pos {
        Pos {x: x, y: y, z: z}
    }

    pub fn vec3(&self) -> Vec3 {
        Vec3 {x: self.x as f32, y: self.y as f32, z: self.z as f32}
        
    }

}
