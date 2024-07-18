use super::direction::Direction;
use crate::constants::*;


#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Car {
    pub from: Direction,        // direction the car is coming from
    pub to: Direction,          // direction the car is going to
    pub pos: (i32, i32),        // current car position
    pub running: bool,          // car is allowed to move this frame
    pub turned: bool,           // car doesn't need to turn anymore
    pub passed_light: bool,     // car is after its corresponding light
    pub color: u8,              // car color according to directions from and to
}

impl Car {
    pub fn new(from: Direction) -> Car {
        let pos: (i32,i32);

        // calculate initial position
        match from {
            Direction::N => {pos = (WIDTH/2 - ROAD_WIDTH/4, - CAR_SIZE / 2);}
            Direction::S => {pos = (WIDTH/2 + ROAD_WIDTH/4, HEIGHT + CAR_SIZE / 2);}
            Direction::W => {pos = (- CAR_SIZE / 2, HEIGHT/2 + ROAD_WIDTH / 4 );}
            Direction::E => {pos = (WIDTH + CAR_SIZE/2, HEIGHT/2 - ROAD_WIDTH / 4);}
        }

        // get a direction that is different from the "from" direction
        let mut rand_to = Direction::random();
        while rand_to == from {
            rand_to = Direction::random();
        }

        // calculate color and store it so that we don't do that every frame 
        let color;
        if Direction::opposite(from, rand_to) {
            color = 0;
        } else if Direction::right(from, rand_to) {
            color = 1;
        } else {
            color = 2;
        }

        return Car{
            from,
            to: rand_to,
            pos,
            running: true,
            passed_light: false,
            turned: Direction::opposite(from, rand_to), // turned is set to true if directions are opposite
            color};
    }

    // returns distance between those cars. 
    pub fn distance(&self, car: &Car) -> i32 {
        Car::distance_from_pos(self.pos, car.pos)
    }

    // calculates distance from car positions. formula is wrong, but simplified for faster calculus
    pub fn distance_from_pos(v1: (i32,i32), v2: (i32, i32)) -> i32 {
        std::cmp::max((v1.0 - v2.0).abs(), (v1.1 - v2.1).abs()) - CAR_SIZE
    }

    // move the car
    pub fn run(&mut self) {
        if !self.running  {
            return;
        }

        if self.turned {
            // car passed its turn point. it should go toward its "to" direction
            match self.to {
                Direction::N => {self.pos.1 -= CAR_SPEED;}
                Direction::S => {self.pos.1 += CAR_SPEED;}
                Direction::W => {self.pos.0 -= CAR_SPEED;}
                Direction::E => {self.pos.0 += CAR_SPEED;}
            }
        } else {
            // car should move away from its "from" direction
            match self.from {
                Direction::N => {self.pos.1 += CAR_SPEED;}
                Direction::S => {self.pos.1 -= CAR_SPEED;}
                Direction::W => {self.pos.0 += CAR_SPEED;}
                Direction::E => {self.pos.0 -= CAR_SPEED;}
            }
        }
        // check if car passed its turn point
        self.check_turned();
    }

    // updates self.turned at true if car passed its turn point
    fn check_turned(&mut self) {
        if self.turned {
            return;
        }

        if self.from == Direction::N || self.from == Direction::S {
            let y_threshold: i32 = if self.to==Direction::E {HEIGHT / 2 + ROAD_WIDTH / 4} else {HEIGHT / 2 - ROAD_WIDTH / 4};
            if (self.from == Direction::N && self.pos.1 >= y_threshold) || (self.from == Direction::S && self.pos.1 <= y_threshold) {
                self.pos.1 = y_threshold;
                self.turned = true;
            }
            return;
        }

        let x_threshold: i32 = if self.to==Direction::N {WIDTH / 2 + ROAD_WIDTH / 4} else {WIDTH / 2 - ROAD_WIDTH / 4};
        if (self.from == Direction::W && self.pos.0 >= x_threshold) || (self.from == Direction::E && self.pos.0 <= x_threshold) {
            self.pos.0 = x_threshold;
            self.turned = true;
        }
    }

    // true if car should be removed from simulation
    pub fn is_out(&self) -> bool {
        match self.to {
            Direction::N => self.pos.1 <= -CAR_SIZE / 2,
            Direction::S => self.pos.1 >= HEIGHT + CAR_SIZE / 2,
            Direction::W => self.pos.0 <= -CAR_SIZE / 2,
            Direction::E => self.pos.0 >= WIDTH + CAR_SIZE / 2
        }
    }

    // true if car passed the intersection and doesn't need to be considered anymore
    pub fn is_passed(&self) -> bool {
        match self.to {
            Direction::N => self.pos.1 <= HEIGHT/2 - 2*ROAD_WIDTH,
            Direction::S => self.pos.1 >= HEIGHT/2 + 2*ROAD_WIDTH,
            Direction::W => self.pos.0 <= WIDTH/2 - 2*ROAD_WIDTH,
            Direction::E => self.pos.0 >= WIDTH/2 + 2*ROAD_WIDTH
        }
    }

    // updates self.running if the car should stop at the light (considering position and light state)
    // pub fn stop_at_light(&mut self, lights: &[bool;4]) {
    //     self.running = !match self.from {
    //         Direction::N => !lights[0] && (self.pos.1 - (HEIGHT - ROAD_WIDTH - CAR_SIZE) / 2).abs() < CAR_SPEED,
    //         Direction::S => !lights[1] && (self.pos.1 - (HEIGHT + ROAD_WIDTH + CAR_SIZE) / 2).abs() < CAR_SPEED,
    //         Direction::W => !lights[2] && (self.pos.0 - (WIDTH - ROAD_WIDTH - CAR_SIZE) / 2).abs() < CAR_SPEED,
    //         Direction::E => !lights[3] && (self.pos.0 - (WIDTH + ROAD_WIDTH + CAR_SIZE) / 2).abs() < CAR_SPEED
    //     }
    // }

    // calculates the car position after moving
    pub fn get_run_pos(&self) -> (i32,i32) {
        let mut run_car = self.clone();
        run_car.run();
        return run_car.pos;
    }

    //updates field passed_light. return true if the field was updated
    // pub fn check_passed_light(&mut self) -> bool {
    //     if self.passed_light {
    //         return false;
    //     }
    //     self.passed_light = match self.from {
    //         Direction::N => self.pos.1 > (HEIGHT - ROAD_WIDTH - CAR_SIZE) / 2 +  CAR_SPEED,
    //         Direction::S => self.pos.1 < (HEIGHT + ROAD_WIDTH + CAR_SIZE) / 2 - CAR_SPEED,
    //         Direction::W => self.pos.0 > (WIDTH - ROAD_WIDTH - CAR_SIZE) / 2 +  CAR_SPEED,
    //         Direction::E => self.pos.0 < (WIDTH + ROAD_WIDTH + CAR_SIZE) / 2 - CAR_SPEED
    //     };
    //     return self.passed_light;
    // }
}