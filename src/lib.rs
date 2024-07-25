extern crate sdl2; 

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

use std::collections::HashMap;

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
    let texture_creator = canvas.texture_creator();
    let sprites = Sprites::new(&texture_creator);
    let mut simulation = Simulation::new();

    let mut pause: bool = false;
    let mut show_stats : bool = false;

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

        if input == 3 {
            show_stats = true;
        }

        if pause {
            continue 'running;
        }

        if show_stats {
            draw_stats(&mut &mut canvas, simulation.stats.get_stats());
            canvas.present();
            continue 'running;
        }

        simulation.run();
        draw_background(&mut canvas);
        draw_simulation(&mut canvas, &simulation, &sprites);
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
            Event::Quit {..} => {
                return 2;
            },
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return 3;
            }
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


pub struct Sprites<'a> {
    pub cars: HashMap<u32, Vec<Texture<'a>>>
}

impl<'a> Sprites<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Sprites<'a> {
        // pub fn new(canvas: Canvas<Window>) -> Sprites<'a> {
        // let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let mut cars = HashMap::<u32, Vec<Texture<'a>>>::new();

        let mut i = 1;
        for id in ["a","b","c"] {
            let mut textures = vec![];
            for j in 1..5 {
                let file_name = format!("assets/img/car_{}_{}.png", id, j);
                textures.push(texture_creator.load_texture(file_name).unwrap());
            }
            cars.insert(i, textures);
            i+=1;
        }

        return Sprites{cars};

        // let texture = texture_creator.load_texture("./assets/car2-left.png").unwrap();
        // canvas.copy(&texture, None, Rect::new(50,50,42,23)).unwrap();
    }

    pub fn get_car_sprite(&self, id: u32, dir: Cardinal) -> &Texture {
        let dir_id = match dir {
            Cardinal::W => 0,
            Cardinal::N => 1,
            Cardinal::E => 2,
            Cardinal::S => 3
        };
        // canvas.copy(&self.cars.get(&id).unwrap()[dir_id], None, rect);
        return &self.cars.get(&id).unwrap()[dir_id];
    }
}

