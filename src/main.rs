extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard;
use std::time::Duration;

mod engine;
mod physics;
use engine::engine::Engine;

pub fn main() {

    // SDL SETUP:
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("Cell Simulator", 700, 500)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // ENGINE SETUP
    let mut engine = Engine::new(&mut canvas);
    engine.rand_spawn_init(10);
    
    // MAIN LOOP
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        engine.canvas.set_draw_color(Color::WHITE);
        engine.canvas.clear();
        engine.update_cells();
        engine.draw_cells();

        engine.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
