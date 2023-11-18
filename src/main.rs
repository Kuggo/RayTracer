
use std::ops::Index;
use std::time::SystemTime;

use sdl2::keyboard::Scancode;
use sdl2::{rect::Rect, keyboard::Keycode};
use sdl2::event::Event;

pub mod voxels;
pub use voxels::{Vec3, Color};



// Aliases
type Canvas = sdl2::render::Canvas<sdl2::video::Window>;



// Data clumps
struct Settings {
    screen_width_pix: u32,
    screen_height_pix: u32,
    pixels_per_unit: u32,
    pixel_size: u8,
    fps: f32,

    camera_pos: Vec3,
    camera_dir: Vec3,
    camera_up: Vec3,
    fov: f32,

    mouse_sensitivity: f32,
    scroll_sensitivity: f32,
    zoom_sensitivity: f32,
    
}


struct Context {
    cfg: Settings,
    sdl_ctx: sdl2::Sdl,
    canvas: Canvas,
    keys: [bool; 256],
    //world: voxels::World,
}
impl Index<Key> for [bool; 256] {
    type Output = bool;

    fn index(&self, key: Key) -> &Self::Output {
        match key {
            _ => &self[key as usize],
        }
    }
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
    /*MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseScrollUp,
    MouseScrollDown,*/
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
}


//Globals





// functions

fn generate_world() -> voxels::World {
    let mut world = voxels::World::new();

    world
}


fn tick() {

}


fn draw_frame(ctx: &mut Context) {
    let clear_color = Color::RGB(0, 0, 0);

    ctx.canvas.set_draw_color(clear_color);
    ctx.canvas.clear();

    ctx.canvas.present();
}
 

fn user_inputs(ctx: &mut Context) -> bool {
    let center: (i32, i32) = ((ctx.cfg.screen_width_pix/2).try_into().unwrap(), (ctx.cfg.screen_height_pix/2).try_into().unwrap());
    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut roll = 0.0;

    
    let mut events = ctx.sdl_ctx.event_pump().unwrap();
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

                ctx.keys[key as usize] = true;
            },

            Event::KeyUp { scancode, .. } => {
                let key = match Key::from_scancode(scancode.unwrap()) {
                    Some(key) => key,
                    None => continue,
                };
                
                ctx.keys[key as usize] = false;
            },

            Event::MouseMotion { xrel, yrel, .. } => {
                yaw += (xrel as f32 / ctx.cfg.pixels_per_unit as f32) * ctx.cfg.mouse_sensitivity;
                pitch += (-yrel as f32 / ctx.cfg.pixels_per_unit as f32) * ctx.cfg.mouse_sensitivity;
                
                // setting the mouse to the center
                ctx.sdl_ctx.mouse().warp_mouse_in_window(ctx.canvas.window(), center.0, center.1);
            },

            _ => {}
        }
    }
    false
}


fn tick_loop(ctx: &mut Context) {
    let target_dt = (1000.0 / ctx.cfg.fps) as i64;
    let start_time = SystemTime::now();
    let mut last_time = start_time.elapsed().unwrap().as_millis();
    let mut dt = target_dt;
    
    loop {
        // game logic
        tick();

        // rendering
        draw_frame(ctx);

        // user input
        let stop = user_inputs(ctx);
        if stop {break;}

        // timing
        let current_time = start_time.elapsed().unwrap().as_millis();
        dt = (current_time - last_time) as i64;
        last_time = current_time;

        let sleep_time = target_dt - dt;
        if sleep_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(sleep_time as u64));
        }
    }
}


fn main() -> Result<(), String> {

    let config: Settings = Settings {
        screen_width_pix: 800,
        screen_height_pix: 600,
        pixels_per_unit: 100,
        pixel_size: 1,
        fps: 30.0,

        camera_pos: Vec3::new(0.0, 0.0, 0.0),
        camera_dir: Vec3::new(0.0, 0.0, 1.0),
        camera_up: Vec3::new(0.0, 1.0, 0.0),
        fov: 90.0,

        mouse_sensitivity: 0.1,
        scroll_sensitivity: 0.1,
        zoom_sensitivity: 0.1,
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("RayTracer", config.screen_width_pix, config.screen_height_pix)
        .build()
        .unwrap();
    
    let canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut key_states: [bool; 256] = [false; 256];
    
    let mut ctx = Context {
        cfg: config,
        sdl_ctx: sdl_context,
        canvas: canvas,
        keys: key_states,
        //world: generate_world(),
    };

    tick_loop(&mut ctx);

    Ok(())
}

