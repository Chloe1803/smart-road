
use crate::constants::*;
use crate::elements::simulation::*;
use crate::elements::car::*;
use crate::elements::direction::*;
use crate::Sprites;


extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;

use sdl2::video::Window;


// draw simulation background
pub fn draw_background(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(169,169,169));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(0,0,0));

    canvas.fill_rect(Rect::new(0,0,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH) as u32 /2)).unwrap();
    canvas.fill_rect(Rect::new(0,(HEIGHT + ROAD_WIDTH) as i32/2,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH) as u32/2)).unwrap();
    canvas.fill_rect(Rect::new((WIDTH + ROAD_WIDTH) as i32/2,0,(WIDTH - ROAD_WIDTH) as u32/2 , (HEIGHT - ROAD_WIDTH)as u32/2)).unwrap();
    canvas.fill_rect(Rect::new((WIDTH + ROAD_WIDTH) as i32/2,(HEIGHT + ROAD_WIDTH) as i32/2,(WIDTH - ROAD_WIDTH)as u32/2 , (HEIGHT - ROAD_WIDTH)as u32/2)).unwrap();

    canvas.set_draw_color(Color::RGB(255,255,255));

    //trait des axes centraux
    canvas.draw_line(Point::new(WIDTH as i32 / 2,0), Point::new(WIDTH as i32 / 2,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2)).unwrap();
    canvas.draw_line(Point::new(WIDTH as i32 / 2, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2,HEIGHT as i32)).unwrap();
    let _ = draw_dashed_line(canvas, Point::new(WIDTH as i32 / 2,HEIGHT as i32 /2  - ROAD_WIDTH as i32/2), Point::new(WIDTH as i32 / 2, HEIGHT as i32/2+ ROAD_WIDTH as i32/2), CENTRAL_DASH_LENGTH);

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
pub fn draw_simulation(canvas: &mut Canvas<Window>, simulation: &Simulation, sprites :&Sprites) {
    // draw cars
    for car in &simulation.cars {
        draw_car(&car.borrow(), canvas, sprites);
    }
}


// draw a car on the canvas
pub fn draw_car(car: &Car, canvas: &mut Canvas<Window>, sprites :&Sprites) {



    let (width, height, dir) = get_orientation(car);

    let rect = Rect::new((car.pos.0 - width / 2) as i32, (car.pos.1 - height / 2) as i32, width as u32, height as u32);

    canvas.copy(&sprites.get_car_sprite(car.color as u32, dir), None, rect).unwrap();
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

pub fn draw_stats(canvas: &mut Canvas<Window>, stats: String) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = "assets/font/Roboto-Black.ttf";
    let font = ttf_context.load_font(font_path, 20).unwrap();

    // Clear the canvas
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.clear();

    // Split the stats string into lines
    let lines: Vec<&str> = stats.split('\n').collect();
    let mut y_offset = 100;

    for line in lines {
        // Render each line to a surface, and then create a texture from it
        let surface = font.render(line)
            .blended(Color::RGBA(255, 255, 255, 255))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        // Copy the texture to the canvas
        canvas.copy(&texture, None, Some(Rect::new(100, y_offset, surface.width(), surface.height()))).unwrap();

        // Increment y_offset for the next line
        y_offset += surface.height() as i32 + 5; // Adjust the spacing between lines if necessary
    }

    // Present the canvas
    canvas.present();
}

fn get_orientation(car: &Car)-> (i32, i32, Cardinal) {
    let width : i32;
    let height : i32;
    let dir : Cardinal;

    match car.direction {
        Direction::Straight => {
            match car.from {
                Cardinal::S => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::N}
                Cardinal::N => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::S},
                Cardinal::W => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::E} 
                Cardinal::E => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::W}
            }
        }
        Direction::Right => {
            match car.turned {
                false => {
                    match car.from {
                        Cardinal::S => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::N},
                        Cardinal::N => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::S},
                        Cardinal::W => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::E} 
                        Cardinal::E => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::W}
                    }
                } 
                true => {
                    match car.from {
                        Cardinal::S => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::E},
                        Cardinal::N => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::W},
                        Cardinal::W => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::S}  
                        Cardinal::E => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::N}
                    }
                }
            }
        }

        Direction::Left => {
            match car.turned {
                false => {
                    match car.from {
                        Cardinal::S => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::N},
                        Cardinal::N => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::S},
                        Cardinal::W => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::E}
                        Cardinal::E => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::W}
                    }
                } 
                true => {
                    match car.from {
                        Cardinal::S  => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::W},
                        Cardinal::N => {width = CAR_LENGTH-10; height = CAR_WIDTH-10; dir = Cardinal::E},
                        Cardinal::W => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::N},
                        Cardinal::E => {width = CAR_WIDTH-10; height = CAR_LENGTH-10; dir = Cardinal::S}
                    }
                }
            }
        }
                
        }   
    return (width, height, dir)
}

    
