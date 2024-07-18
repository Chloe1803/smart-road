use crate::elements::car::*;
use crate::elements::smart_intersection_system::*;
use crate::elements::direction::*;
use crate::constants::*;
pub struct Simulation {
    pub cars: Vec<Car>,                 // cars that didn't pass the crossroad
    pub passed_cars: Vec<Car>,          // cars out of the crossroad
    pub smart_intersection_system: SmartIntersectionSystem,    // implements the light strategy
    frame_count: u32,                   // frame count for auto spawning cars
    car_spawn_rate: f64,                // number of random cars spawned by second
}

impl Simulation {
    pub fn new() -> Simulation {
        return Simulation{
            cars: vec![],
            passed_cars: vec![],
            smart_intersection_system: SmartIntersectionSystem::new(),
            frame_count: 0,
            car_spawn_rate: 0.0
        };
    }

    pub fn run(&mut self) {
        self.frame_count += 1;
        // spawn some cars
        if self.car_spawn_rate != 0.0 && self.frame_count as f64 / FPS as f64 >= 1.0 / self.car_spawn_rate {
            self.frame_count = 0;
            self.add_car(Direction::random());
        }

        // run the smart intersection manager
      

        // move all passed cars. remove the ones out of the screen
        for car in &mut self.passed_cars {
            car.run();
        }
        self.passed_cars.retain(|c| !c.is_out());    


        //PLUS UTILE -> A REMPLACER ? 
        // stop cars at the light 
        // for car in &mut self.cars {
        //     car.running = true;
        //     car.stop_at_light(&self.light_manager.lights);
        // }

        // stop cars too close to each others
        self.stop_cars_too_close();

        // cars who are allowed to run can now move
        for car in &mut self.cars {
            car.run();
            // if car.check_passed_light() {
                // this car just passed the light, we can reduce the corresponding queue
            //     self.light_manager.remove_from_queue(car.from);
            // }
            if car.is_passed() {
                // car is now far enough from the crossroad, we can put it in the passed_car vector
                self.passed_cars.push(car.clone());
            }
        }
        self.cars.retain(|c| !c.is_passed());
    }

    // try to spawn a car from this direction. won't spawn if there is already a car on the edege
    pub fn add_car(&mut self, from: Direction) {
        let new_car = Car::new(from);
        for car in &self.cars {
            if new_car.distance(car) < SECURITY_LIMIT {
                return;
            }
        }
        //self.light_manager.add_in_queue(from);
        self.cars.push(new_car);
    }

    // checks all cars and stops those that are too close to each others
    // we decide to stop a car if its next position is within an other car's security range,
    // and if the distance between those cars was actually reduced after moving.
    // NOTE : not the best way to do that
    pub fn stop_cars_too_close(&mut self) {
        for i in 0..self.cars.len() {
            if !self.cars[i].running {
                continue;
            }
            let current_pos = self.cars[i].pos;
            let next_pos = self.cars[i].get_run_pos();

            let mut should_stop = false;

            for j in 0..self.cars.len() {
                if i == j {
                    continue;
                }
                let next_distance = Car::distance_from_pos(next_pos, self.cars[j].pos);
                if next_distance < SECURITY_LIMIT && 
                    next_distance < Car::distance_from_pos(current_pos, self.cars[j].pos){
                    should_stop = true;
                    break;
                }
            }
            self.cars[i].running = !should_stop;
        }
    }
}









