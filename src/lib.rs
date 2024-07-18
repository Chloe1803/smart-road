extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::{Point, Rect};
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub use std::time::Duration;

mod elements;
mod constants;

use elements::simulation::*;
use elements::direction::*;
use elements::car::Car;
use constants::*;

pub fn start_simulation(){
    //génère la fenêtre
    let (mut canvas, mut event_pump) = initiate();
    // let mut simulation = Simulation::new();

    let mut pause: bool = false;

     // main loop
     'running: loop {

        // gère les inputs utilisateurs
        let input = handle_inputs(&mut event_pump, /*&mut simulation*/);
        if input == 2 {
            break 'running;
        }
        if input == 1 {
            pause = !pause;
        }

        if pause {
            continue 'running;
        }

        // simulation.run();
        draw_background(&mut canvas);
        // draw_simulation(&mut canvas, &simulation);
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
fn handle_inputs(event_pump: &mut EventPump, /*simulation: &mut Simulation*/) -> u8 {
    for event in event_pump.poll_iter() {
        let direction_opt: Option<Direction> = None;
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return 2;
            },
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                return 1;
            },
            // Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
            //     direction_opt = Some(Direction::W);
            // },
            // Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
            //     direction_opt = Some(Direction::E);
            // },
            // Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            //     direction_opt = Some(Direction::N);
            // },
            // Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            //     direction_opt = Some(Direction::S);
            // },
            // Event::KeyDown { keycode: Some(Keycode::KpPlus), .. } => {
            //     simulation.increase_spawn_rate();
            // },
            // Event::KeyDown { keycode: Some(Keycode::KpMinus), .. } => {
            //     simulation.decrease_spawn_rate();
            // },
            // Event::KeyDown { keycode: Some(Keycode::R), .. } => {
            //     direction_opt = Some(Direction::random());
            // },
            _ => {}
        }

        // if direction_opt != None {
        //     simulation.add_car(direction_opt.unwrap());
        // }
    }

    return 0;
}

// draw simulation background
fn draw_background(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(169,169,169));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0,0,0));

    canvas.fill_rect(Rect::new(0,0,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH) as u32 /2)).unwrap();
    canvas.fill_rect(Rect::new(0,(HEIGHT + ROAD_WIDTH) as i32/2,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH) as u32/2)).unwrap();
    canvas.fill_rect(Rect::new((WIDTH + ROAD_WIDTH) as i32/2,0,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH)as u32/2)).unwrap();
    canvas.fill_rect(Rect::new((WIDTH + ROAD_WIDTH) as i32/2,(HEIGHT + ROAD_WIDTH) as i32/2,(WIDTH - ROAD_WIDTH)as u32/2 , (HEIGHT - ROAD_WIDTH)as u32/2)).unwrap();

    canvas.set_draw_color(Color::RGB(255,255,255));

    //trait des axes centraux
    canvas.draw_line(Point::new(WIDTH as i32 / 2,0), Point::new(WIDTH as i32 / 2,(HEIGHT as i32 /2  - ROAD_WIDTH as i32/2))).unwrap();
    canvas.draw_line(Point::new(WIDTH as i32 / 2, (HEIGHT as i32/2+ ROAD_WIDTH as i32/2)), Point::new(WIDTH as i32 / 2,HEIGHT as i32)).unwrap();
    let _ = draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2,(HEIGHT as i32 /2  - ROAD_WIDTH as i32/2)), Point::new(WIDTH as i32 / 2, (HEIGHT as i32/2+ ROAD_WIDTH as i32/2)), CENTRAL_DASH_LENGTH);

    canvas.draw_line(Point::new(0, HEIGHT as i32 / 2), Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2)).unwrap();
    canvas.draw_line(Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2), Point::new(WIDTH as i32,HEIGHT as i32 / 2)).unwrap();
    let _ = draw_dashed_line(canvas, Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2) , Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2), CENTRAL_DASH_LENGTH);

    //traits des voies
    //traits verticaux
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /6, 0), Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /6 ,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /3, 0), Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /3 ,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /6, 0), Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /6 ,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /3, 0), Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /3 ,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2), SECONDARY_DASH_LENGTH);

    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /6, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /6 ,HEIGHT as i32 ), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /3, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2 - ROAD_WIDTH as i32 /3 ,HEIGHT as i32 ), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /6, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /6 ,HEIGHT as i32 ), SECONDARY_DASH_LENGTH);
    let _ =draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /3, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2 + ROAD_WIDTH as i32 /3 ,HEIGHT as i32 ), SECONDARY_DASH_LENGTH);

    //traits horizontaux
    let _ = draw_dashed_line(canvas,  Point::new(0, HEIGHT as i32 / 2 - ROAD_WIDTH as i32 /6), Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2 - ROAD_WIDTH as i32/6) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(0, HEIGHT as i32 / 2 - ROAD_WIDTH as i32 /3), Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2 - ROAD_WIDTH as i32/3) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(0, HEIGHT as i32 / 2 + ROAD_WIDTH as i32 /3), Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2 + ROAD_WIDTH as i32/3) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(0, HEIGHT as i32 / 2 + ROAD_WIDTH as i32 /6), Point::new(WIDTH as i32/2 - ROAD_WIDTH as i32/2,HEIGHT as i32 / 2 + ROAD_WIDTH as i32/6) , SECONDARY_DASH_LENGTH);

    let _ = draw_dashed_line(canvas,  Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2 - ROAD_WIDTH as i32 /6), Point::new(WIDTH as i32,HEIGHT as i32 / 2 - ROAD_WIDTH as i32/6) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2 - ROAD_WIDTH as i32 /3), Point::new(WIDTH as i32,HEIGHT as i32 / 2 - ROAD_WIDTH as i32/3) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2 + ROAD_WIDTH as i32 /3), Point::new(WIDTH as i32,HEIGHT as i32 / 2 + ROAD_WIDTH as i32/3) , SECONDARY_DASH_LENGTH);
    let _ = draw_dashed_line(canvas,  Point::new(WIDTH as i32/2 + ROAD_WIDTH as i32/2, HEIGHT as i32 / 2 + ROAD_WIDTH as i32 /6), Point::new(WIDTH as i32,HEIGHT as i32 / 2 + ROAD_WIDTH as i32/6) , SECONDARY_DASH_LENGTH);




}

// draw all elements of the simulation to the canvas
fn draw_simulation(canvas: &mut Canvas<Window>, simulation: &Simulation) {
    // draw cars
    for car in &simulation.cars {
        draw_car(car, canvas);
    }

    for car in &simulation.passed_cars {
        draw_car(car, canvas);
    }

    // draw lights
    for i in 0..4 {
        let x;
        let y;
        let light_size = 30;
        // if simulation.light_manager.lights[i] {
        //     canvas.set_draw_color(Color::RGB(0,255,0));
        // } else {
        //     canvas.set_draw_color(Color::RGB(255,0,0));
        // }
        match i {
            0 => {x = (WIDTH - ROAD_WIDTH) / 2 - light_size - 5; y = (HEIGHT - ROAD_WIDTH) / 2 - light_size - 5;}
            1 => {x = (WIDTH + ROAD_WIDTH) / 2 + 5; y = (HEIGHT + ROAD_WIDTH) / 2 + 5;}
            2 => {x = (WIDTH - ROAD_WIDTH) / 2 - light_size - 5; y = (HEIGHT + ROAD_WIDTH) / 2 + 5;}
            3 => {x = (WIDTH + ROAD_WIDTH) / 2 + 5; y = (HEIGHT - ROAD_WIDTH) / 2 - light_size - 5;}
            _ => {x = 0; y = 0;}
        }
        canvas.fill_rect(Rect::new(x, y, light_size as u32, light_size as u32)).unwrap();
    }
}


// draw a car on the canvas
fn draw_car(car: &Car, canvas: &mut Canvas<Window>) {
    match car.color {
        0 => {canvas.set_draw_color(Color::RGB(255,0,0));}
        1 => {canvas.set_draw_color(Color::RGB(0,255,0));}
        2 => {canvas.set_draw_color(Color::RGB(0,0,255));}
        _ => {}
    }
    canvas.fill_rect(Rect::new((car.pos.0 - CAR_SIZE / 2) as i32, (car.pos.1 - CAR_SIZE / 2) as i32, CAR_SIZE as u32, CAR_SIZE as u32)).unwrap();
}

fn draw_dashed_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, start: Point, end: Point, dash_length: i32) -> Result<(), String> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = ((dx * dx + dy * dy) as f64).sqrt() as i32;
    let num_dashes = distance / dash_length;
    
    for i in 0..num_dashes {
        let t_start = i as f64 / num_dashes as f64;
        let t_end = (i as f64 + 0.5) / num_dashes as f64;
        
        let x1 = (start.x as f64 * (1.0 - t_start) + end.x as f64 * t_start) as i32;
        let y1 = (start.y as f64 * (1.0 - t_start) + end.y as f64 * t_start) as i32;
        
        let x2 = (start.x as f64 * (1.0 - t_end) + end.x as f64 * t_end) as i32;
        let y2 = (start.y as f64 * (1.0 - t_end) + end.y as f64 * t_end) as i32;
        
        canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2))?;
    }
    
    Ok(())
}
