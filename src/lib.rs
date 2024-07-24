extern crate sdl2; 


use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub use std::time::Duration;

mod elements;
mod constants;
mod draw;

use elements::simulation::*;
use elements::direction::*;
use constants::*;
use draw::*;

pub fn start_simulation(){
    //génère la fenêtre
    let (mut canvas, mut event_pump) = initiate();
    let mut simulation = Simulation::new();

    let mut pause: bool = false;

     // main loop
     'running: loop {

        // gère les inputs utilisateurs
        let input = handle_inputs(&mut event_pump, &mut simulation);
        if input == 2 {
            break 'running;
        }
        if input == 1 {
            pause = !pause;
        }

        if pause {
            continue 'running;
        }

        simulation.run();
        draw_background(&mut canvas);
        draw_simulation(&mut canvas, &simulation);
        canvas.present();

        // NOTE : this is bad. we should remain time needed to calculate the frame from this sleep duration
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
     }
}



// initiates canvas and eventpump
fn initiate() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("smart_road", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    return (canvas, event_pump);
}


//Sert à généré les inputs utilisateurs 
// apply events from user input to the simulation
// returns 2 if program should stop and 1 if program should pause
fn handle_inputs(event_pump: &mut EventPump, simulation: &mut Simulation) -> u8 {
    for event in event_pump.poll_iter() {
        let mut direction_opt: Option<Cardinal> = None;
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return 2;
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                return 1;
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                direction_opt = Some(Cardinal::W);
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                direction_opt = Some(Cardinal::E);
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                direction_opt = Some(Cardinal::N);
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                direction_opt = Some(Cardinal::S);
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                direction_opt = Some(Cardinal::random());
            },
            _ => {}
        }

        if direction_opt != None {
            simulation.add_car(direction_opt.unwrap());
        }
    }

    return 0;
}


