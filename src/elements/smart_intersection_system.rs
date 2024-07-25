use crate::{Cardinal, Direction};
use crate::{elements::car::*, HEIGHT, ROAD_WIDTH, WIDTH};
use crate::elements::area::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct SmartIntersectionSystem{
    pub area : Rect,
    pub cars_waiting: Vec<(Rc<RefCell<Car>>, Vec<(usize, usize)>)>,
    pub cars_crossing: Vec<(Rc<RefCell<Car>>, Vec<(usize, usize)>)>,
    pub locked_areas : Vec<Vec<LockedArea>>
}

impl  SmartIntersectionSystem  {
    
    pub fn new() -> Self {
        let a = Point {
            x: (WIDTH / 2 - ROAD_WIDTH / 2),
            y: (HEIGHT / 2 + ROAD_WIDTH / 2),
        };

        let b = Point {
            x: (WIDTH / 2 + ROAD_WIDTH / 2),
            y: (HEIGHT / 2 + ROAD_WIDTH / 2),
        };

        let c = Point {
            x: (WIDTH / 2 + ROAD_WIDTH / 2),
            y: (HEIGHT / 2 - ROAD_WIDTH / 2),
        };

        let d = Point {
            x: (WIDTH / 2 - ROAD_WIDTH / 2),
            y: (HEIGHT / 2 - ROAD_WIDTH / 2),
        };

        let area = Rect { a, b, c, d };

        let locked_areas = get_locked_area(area.clone());

        SmartIntersectionSystem {
            area,
            cars_waiting: Vec::new(),
            cars_crossing: Vec::new(),
            locked_areas, 
        }
    }

    pub fn car_entering(&mut self, car: Rc<RefCell<Car>>) {
        let routes = get_route(car.borrow().from, car.borrow().direction);

        for route in &routes {
            self.locked_areas[route.0][route.1].add_to_queue(car.borrow().id);
        }

        car.borrow_mut().running = false;

        self.cars_waiting.push((car, routes));
    }

    pub fn manage_intersection(&mut self){
        
        self.update_areas();
        self.update_cars();
      
    }

    pub fn update_areas(&mut self){

        for row in &mut self.locked_areas {
            for locked_area in row {
                locked_area.update_queue( &self.cars_crossing);
            }
        }
    }

    pub fn update_cars(&mut self){
        let mut cars_to_move = Vec::new();
        
        // Itére sur toutes les voitures en attente

        for (i, (car, routes)) in self.cars_waiting.iter().enumerate() {

            
            // Vérifie si la voiture peut traverser toutes les zones verrouillées
            if routes.iter().all(|&(x, y)| {
                if let Some((current_car_id, _)) = self.locked_areas[x][y].current_ticket {
                    current_car_id == car.borrow().id      
                } else {
                    false
                }
            }) {
                cars_to_move.push(i);
            }
        }

        for &i in cars_to_move.iter().rev() {
            let (car, routes) = self.cars_waiting.remove(i);
        
            car.borrow_mut().running = true;
            self.cars_crossing.push((car, routes));
        }
    }
    
    pub fn car_exiting(&mut self, car: Rc<RefCell<Car>>){
        self.cars_crossing.retain(|(c, _)| c.borrow().id != car.borrow().id);
    }

}


fn get_route(from: Cardinal, direction : Direction)-> Vec<(usize, usize)>{
    match direction {
        Direction::Straight => {
            match from {
                Cardinal::N => return vec![(3, 0), (2, 0), (1, 0), (0,0)],
                Cardinal::S => return vec![(0, 3),(1,3),(2,3), (3,3)],
                Cardinal::W => return vec![(0,0), (0,1), (0,2), (0,3)],
                Cardinal::E => return vec![(3,3), (3,2), (3,1), (3,0)]
            }
        },
        Direction::Left => {
            match from {
                Cardinal::N => return vec![(3,1), (2,1), (1,1), (1,2), (1,3)],
                Cardinal::S => return  vec![(0,2), (1,2), (2,2), (2,1), (2,0)],
                Cardinal::W => return vec![(1,0), (1,1), (1,2), (2,2), (3,2)],
                Cardinal::E => return vec![(2,3), (2,2), (2,1), (1,1), (0,1)],
            }
        }
        Direction::Right => return vec![]
    }
}







