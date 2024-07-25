use std::rc::Rc;
use std::cell::RefCell;

use crate::elements::car::*;
use crate::elements::smart_intersection_system::*;
use crate::elements::direction::*;
use crate::constants::*;

use super::statistics::Statitics;
pub struct Simulation{
    pub cars: Vec<Rc<RefCell<Car>>>,                 // cars that didn't pass the crossroad
    pub nb_cars : usize,
    pub smart_intersection_system: SmartIntersectionSystem,    // implements the light strategy
    frame_count: u32,                   // frame count for auto spawning cars
    pub stats : Statitics
}

impl Simulation {
    pub fn new() -> Simulation{
        return Simulation{
            nb_cars : 0,
            cars: vec![],
            smart_intersection_system: SmartIntersectionSystem::new(),
            frame_count: 0,
            stats : Statitics::new()
        };
    }

    pub fn run(&mut self) {
        self.frame_count += 1;

        self.smart_intersection_system.manage_intersection();

        for car in &self.cars {
            
            car.borrow_mut().run();
   
            if car.borrow().is_in_intersection(&self.smart_intersection_system.area){
                if !car.borrow().was_in_intersection {
                    car.borrow_mut().was_in_intersection = true;

                    if car.borrow().direction != Direction::Right{
                        self.smart_intersection_system.car_entering(car.clone())  
                    }
                        
                }

            }else{
                if car.borrow().was_in_intersection && !car.borrow().passed {
                    car.borrow_mut().passed = true;

                    if car.borrow().direction != Direction::Right{
                        self.smart_intersection_system.car_exiting(car.clone())
                    }
                }
            }
            

            if (car.borrow().passed && car.borrow().is_out())  {
                let exiting_time = car.borrow().timestamp.unwrap().elapsed().as_secs() as usize;

                if exiting_time < self.stats.min_time {
                    self.stats.min_time = exiting_time
                }

                if exiting_time > self.stats.max_time {
                    self.stats.max_time = exiting_time
                }

                let velocity = car.borrow().distance as f64 /exiting_time as f64;
                if velocity < self.stats.min_velocity {
                    self.stats.min_velocity = velocity
                }

                if velocity > self.stats.max_velocity {
                    self.stats.max_velocity = velocity
                }

            }
        }
        
       
        self.cars.retain(|car| !(car.borrow().passed && car.borrow().is_out()));
        
        self.check_security_limit()
    }

    // try to spawn a car from this direction. won't spawn if there is already a car on the edege
    pub fn add_car(&mut self, from: Cardinal) {
        let new_car = Rc::new(RefCell::new(Car::new(from, self.nb_cars)));
        self.nb_cars +=1;
        self.stats.max_number_of_vehicules +=1;
        for car in &self.cars {
            if new_car.borrow().distance(&car.borrow()) < SECURITY_LIMIT {
                return;
            }
        }

        self.cars.push(new_car);
    }

   

    // checks all cars and stops those that are too close to each others
    // we decide to stop a car if its next position is within an other car's security range,
    // and if the distance between those cars was actually reduced after moving.
    // NOTE : not the best way to do that
    pub fn check_security_limit(&self) {
        for i in 0..self.cars.len() {
            let mut car = self.cars[i].borrow_mut();
    
            if car.is_in_intersection(&self.smart_intersection_system.area) {
                continue;
            }
    
            let current_pos = car.pos;
            let next_pos = car.get_run_pos();
    
            let mut should_stop = false;
    
            for j in 0..i {
                if i == j  || car.direction != self.cars[j].borrow().direction || car.from != self.cars[j].borrow().from{
                    continue;
                }
                
                let other_car = self.cars[j].borrow();
                let next_distance = Car::distance_from_pos(next_pos, other_car.pos);
                let current_distance = Car::distance_from_pos(current_pos, other_car.pos);
            
                if next_distance < SECURITY_LIMIT*2 && next_distance < current_distance*2 {
                    should_stop = true;
                    break;
                }
            }
    
            car.running = !should_stop;
        }
    }
    
}









