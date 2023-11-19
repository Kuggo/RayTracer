
pub use crate::voxels::{World, Color};
use crate::linalg::*;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;




pub struct Screen {
    pub width_pix: u32,
    pub height_pix: u32,
    pixel_size: u8,
    canvas: WindowCanvas,
}

impl Screen {
    pub fn new(sdl_ctx: &mut sdl2::Sdl, width_pix: u32, height_pix: u32, pixel_size: u8, title: &str) -> Result<Self, String> {
        let video_subsystem = sdl_ctx.video()?;
        let window = video_subsystem.window(title, width_pix as u32 * pixel_size as u32, height_pix as u32 * pixel_size as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        sdl_ctx.mouse().show_cursor(false);

        Ok(Screen {
            width_pix,
            height_pix,
            pixel_size,
            canvas,
        })
    }

    pub fn show(&mut self) {
        self.canvas.present();
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        -(self.width_pix as i32) / 2 <= x && x < (self.width_pix / 2) as i32 &&
         -(self.height_pix as i32) / 2 <= y && y < (self.height_pix / 2) as i32
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        let rect = Rect::new(
            (self.width_pix / 2) as i32 + (x * self.pixel_size as i32), 
            (self.height_pix / 2) as i32 - (y * self.pixel_size as i32), 
            self.pixel_size as u32, 
            self.pixel_size as u32);
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }
}


pub struct Camera {
    pub screen: Screen,
    pixels_per_unit: u32,
    width_units: f32,
    height_units: f32,
    world: World,
    position: Vec3,
    front_direction: Vec3,
    up_vector: Vec3,
    fov: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(screen: Screen, world: World, position: Vec3, direction: Vec3, up_vector: Vec3, fov: f32, pixels_per_unit: u32) -> Self {
        let pixels_per_unit = pixels_per_unit;
        let width_units = screen.width_pix as f32 / pixels_per_unit as f32;
        let height_units = screen.height_pix as f32 / pixels_per_unit as f32;
        let front_direction = (direction.sub(&position)).normalize();
        let up_vector = up_vector.normalize();
        let mut camera = Camera {
            screen,
            pixels_per_unit,
            width_units,
            height_units,
            world,
            position,
            front_direction,
            up_vector,
            fov: 0.0,
            focal_length: 0.0,
        };
        camera.set_fov(fov);
        camera
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.focal_length = (self.width_units / 2.0) / (fov / 2.0).tan();
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.set_fov(self.fov + zoom);
    }

    pub fn get_window(&self) -> &Window {
        self.screen.canvas.window()
    }

    pub fn get_direction(&self) -> Vec3 {
        self.front_direction
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        let yaw = angle / self.pixels_per_unit as f32;
        self.front_direction = self.front_direction.rotate_xz(-yaw);
        self.up_vector = self.up_vector.rotate_xz(-angle);
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        let pitch = angle / self.pixels_per_unit as f32;
        let right_vec = self.front_direction.cross(&self.up_vector);
        self.front_direction = self.front_direction.rotate_around(&right_vec, pitch);
        self.up_vector = self.up_vector.rotate_around(&right_vec, pitch);
    }

    pub fn rotate_roll(&mut self, angle: f32) {
        self.up_vector = self.up_vector.rotate_around(&self.front_direction, angle);
    }

    pub fn move_forward(&mut self, direction: Vec3) {
        let mov_reference = Vec3::new(self.position.x, 0.0, self.position.z).normalize();
        let angle = mov_reference.angle(&Vec3::new(0.0, 0.0, 1.0));
        let mov_dir = direction.rotate_around(&Vec3::new(0.0, 1.0, 0.0), angle);
        self.position = self.position.add(&mov_dir);
    }

    pub fn world_to_camera(&self, p: Vec3) -> Vec3 {
        p.sub(&self.position)
    }

    pub fn draw_frame(&mut self) {
        /*let up_vec = self.up_vector.scale(self.height_units / self.screen.height_pix as f32);
        let right_vec = self.front_direction.cross(&self.up_vector).scale(self.width_units / self.screen.width_pix as f32);
        let front_vec = self.front_direction.scale(self.focal_length);

        for x in 0..self.screen.width_pix {
            for y in 0..self.screen.height_pix {
                let x_c = x - (self.screen.width_pix / 2);
                let y_c = (self.screen.height_pix / 2) - y;
                let v = front_vec + right_vec * x_c as f32 + up_vec * y_c as f32;
                let ray = Ray::new(self.position, v);
                let color = ray.trace(&self.world);
                self.screen.draw_pixel(x, y, color);
            }
        }
        */
        self.screen.draw_pixel(0, 0, Color::RGB(255, 0, 0));
        self.screen.show();
    }
}


