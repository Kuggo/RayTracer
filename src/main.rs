
use std::ops::Index;

use std::time::Instant;

use sdl2::{keyboard::Scancode};
use sdl2::event::Event;

pub mod linalg;
pub use linalg::{Vec3};
pub mod voxels;
pub use voxels::{Color};
pub mod camera;
pub use camera::{Camera, Screen};


// Aliases
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

type Keys = [bool; 256];
impl Index<Key> for Keys {
    type Output = bool;

    fn index(&self, key: Key) -> &Self::Output {
        match key {
            _ => &self[key as usize],
        }
    }
}


// Data clumps
struct Settings {
    mouse_sensitivity: f32,
    scroll_sensitivity: f32,
    zoom_sensitivity: f32,
}



enum Key {
    W,
    A,
    S,
    D,
    
    /*Up,
    Down,
    Left,
    Right, */

    Space,
    Shift,
    Ctrl,
    Esc,
    
    MouseLeft,
    MouseRight,
    MouseMiddle,
}

impl Key {
    fn from_scancode(scancode: Scancode) -> Option<Key> {
        match scancode {
            Scancode::W => Some(Key::W),
            Scancode::A => Some(Key::A),
            Scancode::S => Some(Key::S),
            Scancode::D => Some(Key::D),
            Scancode::Space => Some(Key::Space),
            Scancode::LShift => Some(Key::Shift),
            Scancode::LCtrl => Some(Key::Ctrl),
            Scancode::Escape => Some(Key::Esc),
            _ => None,
        }
    }

    fn from_mouse(mouse_btn: sdl2::mouse::MouseButton) -> Option<Key> {
        match mouse_btn {
            sdl2::mouse::MouseButton::Left => Some(Key::MouseLeft),
            sdl2::mouse::MouseButton::Right => Some(Key::MouseRight),
            sdl2::mouse::MouseButton::Middle => Some(Key::MouseMiddle),
            _ => None,
        }
    }
}


//Globals





// functions

fn generate_world() -> voxels::World {
    let mut world = voxels::World::new();

    world
}


fn tick() {

}
 

fn user_inputs(sdl_ctx: &mut sdl2::Sdl, cfg: &Settings, camera: &mut Camera, key_states: &mut Keys) -> bool {
    let center_x = camera.screen.width_pix as i32 / 2;
    let center_y = camera.screen.height_pix as i32 / 2;
    
    let mut events = sdl_ctx.event_pump().unwrap();
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} => {
                return true;
            },

            Event::KeyDown { scancode, .. } => {
                let key = match Key::from_scancode(scancode.unwrap()) {
                    Some(key) => key,
                    None => continue,
                };
                match key {
                    Key::Esc => return true,
                    _ => {},
                }

                key_states[key as usize] = true;
            },

            Event::KeyUp { scancode, .. } => {
                let key = match Key::from_scancode(scancode.unwrap()) {
                    Some(key) => key,
                    None => continue,
                };
                
                key_states[key as usize] = false;
            },

            Event::MouseMotion { xrel, yrel, .. } => {
                let yaw = xrel as f32 * cfg.mouse_sensitivity;
                camera.rotate_yaw(yaw);
                
                let pitch = -yrel as f32 * cfg.mouse_sensitivity;
                camera.rotate_pitch(pitch);

                // setting the mouse to the center
                sdl_ctx.mouse().warp_mouse_in_window(camera.get_window(), center_x, center_y);
            },

            Event::MouseWheel { y, .. } => {
                if key_states[Key::Ctrl] {
                    let zoom = (y as f32) * cfg.zoom_sensitivity;
                    camera.zoom(zoom);
                } 
                else {
                    let roll = (y as f32) * cfg.scroll_sensitivity;
                    camera.rotate_roll(roll);
                }
            },

            Event::MouseButtonDown {mouse_btn, .. } => {
                //clicks tells you how many clicks it was. Ex: 1 for single click, 2 for double click, etc.
                if let Some(key) = Key::from_mouse(mouse_btn) {
                    key_states[key as usize] = true;
                }
                // atm nothing is done to know the position of where mouse was clicked, because its always in the center
            },

            Event::MouseButtonUp {mouse_btn, .. } => {
                if let Some(key) = Key::from_mouse(mouse_btn) {
                    key_states[key as usize] = false;
                }
            },

            _ => {}
        }
    }
    
    let mov_x = key_states[Key::D] as i32 as f32 - key_states[Key::A] as i32 as f32;
    let mov_y = key_states[Key::Space] as i32 as f32 - key_states[Key::Shift] as i32 as f32;
    let mov_z = key_states[Key::S] as i32 as f32 - key_states[Key::W] as i32 as f32;
    let mov = Vec3::new(mov_x, mov_y, mov_z).normalize();

    camera.move_forward(mov);
    
    false
}

fn main() -> Result<(), String> {
    const screen_width_pix: u32 = 800;
    const screen_height_pix: u32 = 600;
    const pixels_per_unit: u32 = 100;
    const pixel_size: u8 = 1;
    let fps: f32 = 30.0;

    let camera_pos = Vec3::new(0.0, 0.0, 0.0);
    let camera_dir = Vec3::new(0.0, 0.0, 1.0);
    let camera_up = Vec3::new(0.0, 1.0, 0.0);
    let fov: f32 = 90.0;

    let mouse_sensitivity: f32 = 0.1;
    let scroll_sensitivity: f32 = 0.1;
    let zoom_sensitivity: f32 = 0.1;

    let mut world = generate_world();
    
    let mut sdl_ctx: sdl2::Sdl = sdl2::init()?;
    let screen = Screen::new(&mut sdl_ctx, screen_width_pix, screen_height_pix, pixel_size, "RayTracer").unwrap();
    let mut camera = Camera::new(screen, world, camera_pos, camera_dir, camera_up, fov, pixels_per_unit);
    
    let mut key_states: Keys = [false; 256];

    let config = Settings {
        mouse_sensitivity,
        scroll_sensitivity,
        zoom_sensitivity,
    };
    
    
    let target_dt = (SEC_NANOS / fps) as u64;
    const SEC_NANOS : f32 = 1_000_000_000.0;
    
    let mut dt = target_dt;
    
    loop {
        let last_time = Instant::now();

        // game logic
        tick();

        // rendering
        camera.draw_frame();

        // user input
        let stop = user_inputs(&mut sdl_ctx, &config, &mut camera, &mut key_states);
        if stop {break;}

        // timing
        let current_time = Instant::now();
        dt = current_time.duration_since(last_time).as_nanos() as u64;

        let sleep_time = target_dt - dt;
        if sleep_time > 0 {
            spin_sleep::sleep(std::time::Duration::from_nanos(sleep_time));
        }
        println!("FPS: {:.2}", SEC_NANOS / dt.max(target_dt) as f32);

    }
    Ok(())
}

