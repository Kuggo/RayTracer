
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::f32::consts::PI;

pub use crate::voxels::{World, Color, Ray};
use crate::linalg::*;



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
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        sdl_ctx.mouse().show_cursor(false);
        canvas.set_scale(pixel_size as f32, pixel_size as f32).unwrap();

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

    pub fn get_screen_center_pix(&self) -> (i32, i32) {
        let x = self.width_pix as i32 * self.pixel_size as i32 / 2;
        let y = self.height_pix as i32 * self.pixel_size as i32 / 2;
        (x, y)
    }

    pub fn center_pix(&self, x: i32, y: i32) -> Point {
        let x_c = x + (self.width_pix as i32 / 2);
        let y_c = (self.height_pix as i32 / 2) - y;
        Point::new(x_c, y_c)
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        -(self.width_pix as i32) / 2 <= x && x < (self.width_pix / 2) as i32 &&
         -(self.height_pix as i32) / 2 <= y && y < (self.height_pix / 2) as i32
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.canvas.set_draw_color(color.sdl_format());
        let p = self.center_pix(x, y);
        self.canvas.draw_point(p).unwrap();
    }
}


pub struct Camera {
    pub screen: Screen,
    pixels_per_unit: u32,
    width_units: f32,
    height_units: f32,
    world: Box<World>,
    position: Vec3,
    lookat_direction: Vec3,
    up_vector: Vec3,
    fov: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new(screen: Screen, world: Box<World>, position: Vec3, direction: Vec3, up_vector: Vec3, fov: f32, pixels_per_unit: u32) -> Self {
        let pixels_per_unit = pixels_per_unit;
        let width_units = screen.width_pix as f32 / pixels_per_unit as f32;
        let height_units = screen.height_pix as f32 / pixels_per_unit as f32;
        let front_direction = direction.normalize();
        let up_vector = up_vector.normalize();
        let mut camera = Camera {
            screen,
            pixels_per_unit,
            width_units,
            height_units,
            world,
            position,
            lookat_direction: front_direction,
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
        let fov = (self.fov + zoom).clamp(30.0 * PI/180.0, 160.0 * PI/180.0);
        self.set_fov(fov);
    }

    pub fn get_window(&self) -> &Window {
        self.screen.canvas.window()
    }

    pub fn get_direction(&self) -> Vec3 {
        self.lookat_direction
    }

    pub fn rotate_yaw(&mut self, angle: f32) {
        let yaw = angle / self.pixels_per_unit as f32;
        self.lookat_direction = self.lookat_direction.rotate_xz(-yaw);
        self.up_vector = self.up_vector.rotate_xz(-yaw);
    }

    pub fn rotate_pitch(&mut self, angle: f32) {
        let pitch = angle / self.pixels_per_unit as f32;
        let right_vec = self.lookat_direction.cross(&self.up_vector);
        self.lookat_direction = self.lookat_direction.rotate_around(&right_vec, pitch);
        self.up_vector = self.up_vector.rotate_around(&right_vec, pitch);
    }

    pub fn rotate_roll(&mut self, angle: f32) {
        self.up_vector = self.up_vector.rotate_around(&self.lookat_direction, angle);
    }

    pub fn move_rel_to_facing(&mut self, direction: Vec3) {
        let up_component = Vec3::new(0.0, direction.y, 0.0);
        let hori_component = direction.sub(&up_component);

        let sign = (((self.lookat_direction.x >= 0.0) as i32) * 2) - 1;
        let reference = Vec3::new(self.lookat_direction.x, 0.0, self.lookat_direction.z);
        let angle = sign as f32 * Z_AXIS.angle(&reference);

        let hori_dir = hori_component.rotate_xz(angle);
        let mov_dir = hori_dir.add(&up_component);

        self.position = self.position.add(&mov_dir);

        self.world.update_chunks_in_area(self.position);
    }

    pub fn world_to_camera(&self, p: Vec3) -> Vec3 {
        p.sub(&self.position)
    }

    pub fn draw_frame(&mut self) {
        let up_vec = self.up_vector.scale(self.height_units / self.screen.height_pix as f32);
        let right_vec = self.lookat_direction.cross(&self.up_vector).scale(self.width_units / self.screen.width_pix as f32);
        let front_vec = self.lookat_direction.scale(self.focal_length);

        for x in (-(self.screen.width_pix as i32) / 2)..(self.screen.width_pix as i32 / 2) {
            for y in (-(self.screen.height_pix as i32) / 2)..(self.screen.height_pix as i32 / 2) {
                let v = front_vec.add(&right_vec.scale(x as f32)).add(&up_vec.scale(y as f32));
                let ray = Ray::new(self.position, v);
                let color = ray.trace(&self.world, 0);
                self.screen.draw_pixel(x, y, color);
            }
        }
        self.screen.show();
    }
}


