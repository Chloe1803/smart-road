use crate::{constants::*, Cardinal, Direction};
use crate::elements::area::*;

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Car {
    pub id: usize,
    pub from: Cardinal,       
    pub to: Cardinal,
    pub direction : Direction,          
    pub pos: (i32, i32),        // current car position
    pub turning_point : (i32, i32), //position of when to turn
    pub running: bool,          // car is allowed to move this frame
    pub turned: bool,           // car doesn't need to turn anymore
    pub passed: bool,
    pub was_in_intersection : bool,
    pub color: u8,              // car color according to Cardinals from and to
}

impl Car {
    pub fn new(from: Cardinal, id: usize) -> Car {
        let pos: (i32,i32);
        let pos_x : i32;
        let pos_y : i32;

        let turning_point: (i32,i32);
        let turn_x : i32;
        let turn_y : i32; 

        let direction = Direction::random();
        let cardinal_to : Cardinal;

        // calculate initial position
        match from {
            Cardinal::N => {
                match direction {
                    Direction::Right => {
                        pos_x = WIDTH/2 - (ROAD_WIDTH/12)*5;
                        cardinal_to = Cardinal::W;          
                        turn_y= HEIGHT/2 - (ROAD_WIDTH/12)*5;
                    },
                    Direction::Straight => {
                        pos_x = WIDTH/2 - (ROAD_WIDTH/12)*3;
                        cardinal_to = Cardinal::S;
                        turn_y = HEIGHT/2 -(ROAD_WIDTH/12)*3;
                    },
                    Direction::Left => {
                        pos_x= WIDTH/2 - ROAD_WIDTH/12;
                        cardinal_to = Cardinal::E;
                        turn_y = HEIGHT/2 + ROAD_WIDTH / 12;
                    }
                }

                pos_y = - CAR_SIZE / 2;
                turn_x = pos_x;              
            }
            Cardinal::S => {
                match direction {
                    Direction::Right => {
                        pos_x= WIDTH/2 + (ROAD_WIDTH/12)*5;
                        cardinal_to = Cardinal::E;
                        turn_y = HEIGHT/2 + (ROAD_WIDTH/12)*5;
                    }
                    Direction::Straight => {
                        pos_x = WIDTH/2 + (ROAD_WIDTH/12)*3;
                        cardinal_to = Cardinal::N;
                        turn_y = HEIGHT/2 + (ROAD_WIDTH/12)*3;
                    }
                    Direction::Left => {
                        pos_x = WIDTH/2 + ROAD_WIDTH/12;
                        cardinal_to= Cardinal::W;
                        turn_y = HEIGHT/2 - ROAD_WIDTH / 12;
                    }
                }
                pos_y = HEIGHT + CAR_SIZE / 2;
                turn_x = pos_x;            
                
            }
            Cardinal::W => {
                match direction {
                    Direction::Right => {
                        pos_y= HEIGHT/2 + (ROAD_WIDTH/12)*5;
                        cardinal_to = Cardinal::S;
                        turn_x = WIDTH/2 - (ROAD_WIDTH/12)*5;
                    }
                    Direction::Straight => {
                        pos_y= HEIGHT/2 + (ROAD_WIDTH/12)*3;
                        cardinal_to = Cardinal::E;
                        turn_x = WIDTH/2 - (ROAD_WIDTH/12)*3;
                    }
                    Direction::Left =>  {
                        pos_y= HEIGHT/2 + ROAD_WIDTH / 12;
                        cardinal_to = Cardinal::N;
                        turn_x = WIDTH/2 + ROAD_WIDTH/12;
                    }
                }
                pos_x= - CAR_SIZE / 2;
                turn_y = pos_y;
            }
            Cardinal::E => {
                match direction{
                    Direction::Right => {
                        pos_y = HEIGHT/2 - (ROAD_WIDTH/12)*5;
                        cardinal_to = Cardinal::N;
                        turn_x = WIDTH/2 + (ROAD_WIDTH/12)*5;
                    }
                    Direction::Straight => {
                        pos_y= HEIGHT/2 -(ROAD_WIDTH/12)*3;
                        cardinal_to = Cardinal::W;
                        turn_x = WIDTH/2 + (ROAD_WIDTH/12)*3;
                    }
                    Direction::Left =>  {
                        pos_y= HEIGHT/2 - ROAD_WIDTH / 12;
                        cardinal_to = Cardinal::S;
                        turn_x = WIDTH/2 - ROAD_WIDTH/12;
                    }
                }
                pos_x= WIDTH + CAR_SIZE/2;
                turn_y = pos_y;
            }
        }

        pos = (pos_x, pos_y);
        turning_point = (turn_x, turn_y);

        // calculate color and store it so that we don't do that every frame 
        let color;
        if Cardinal::opposite(from, cardinal_to) {
            color = 0;
        } else if Cardinal::right(from, cardinal_to) {
            color = 1;
        } else {
            color = 2;
        }

        return Car{
            id,
            from,
            to: cardinal_to,
            direction,
            pos,
            turning_point,
            running: true,
            was_in_intersection : false,
            passed: false,
            turned: Cardinal::opposite(from, cardinal_to), // turned is set to true if Cardinals are opposite
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
            // car passed its turn point. it should go toward its "to" Cardinal
            match self.to {
                Cardinal::N => {self.pos.1 -= CAR_SPEED;}
                Cardinal::S => {self.pos.1 += CAR_SPEED;}
                Cardinal::W => {self.pos.0 -= CAR_SPEED;}
                Cardinal::E => {self.pos.0 += CAR_SPEED;}
            }
        } else {
            // car should move away from its "from" Cardinal
            match self.from {
                Cardinal::N => {self.pos.1 += CAR_SPEED;}
                Cardinal::S => {self.pos.1 -= CAR_SPEED;}
                Cardinal::W => {self.pos.0 += CAR_SPEED;}
                Cardinal::E => {self.pos.0 -= CAR_SPEED;}
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

        if self.from == Cardinal::N || self.from == Cardinal::S {
            let y_threshold: i32 = self.turning_point.1;
            if (self.from == Cardinal::N && self.pos.1 >= y_threshold) || (self.from == Cardinal::S && self.pos.1 <= y_threshold) {
                self.pos.1 = y_threshold;
                self.turned = true;
            }
            return;
        }

        let x_threshold: i32 = self.turning_point.0;
        if (self.from == Cardinal::W && self.pos.0 >= x_threshold) || (self.from == Cardinal::E && self.pos.0 <= x_threshold) {
            self.pos.0 = x_threshold;
            self.turned = true;
        }
    }

    // true if car should be removed from simulation
    pub fn is_out(&self) -> bool {
        match self.to {
            Cardinal::N => self.pos.1 <= -CAR_SIZE / 2,
            Cardinal::S => self.pos.1 >= HEIGHT + CAR_SIZE / 2,
            Cardinal::W => self.pos.0 <= -CAR_SIZE / 2,
            Cardinal::E => self.pos.0 >= WIDTH + CAR_SIZE / 2
        }
    }

    // true if car passed the intersection and doesn't need to be considered anymore
    // pub fn is_passed(&self) -> bool {
    //     match self.to {
    //         Cardinal::N => self.pos.1 <= HEIGHT/2 - 2*ROAD_WIDTH,
    //         Cardinal::S => self.pos.1 >= HEIGHT/2 + 2*ROAD_WIDTH,
    //         Cardinal::W => self.pos.0 <= WIDTH/2 - 2*ROAD_WIDTH,
    //         Cardinal::E => self.pos.0 >= WIDTH/2 + 2*ROAD_WIDTH
    //     }
    // }

    pub fn get_rect(&self) -> Rect {
        Rect {
            a: Point { x: self.pos.0 - CAR_WIDTH/2 , y: self.pos.1 + CAR_LENGTH/2  },
            b: Point { x: self.pos.0 + CAR_WIDTH/2 , y: self.pos.1 + CAR_LENGTH/2 },
            c: Point { x: self.pos.0 + CAR_WIDTH/2 , y: self.pos.1 - CAR_LENGTH/2 },
            d: Point { x: self.pos.0 - CAR_WIDTH/2 , y: self.pos.1 - CAR_LENGTH/2 },
        }
    }

    pub fn is_in_intersection(&self, rect: &Rect) -> bool {
        let car_rect = self.get_rect();
        Rect::overlaps(&car_rect, rect)
    }


    // calculates the car position after moving
    pub fn get_run_pos(&self) -> (i32,i32) {
        let mut run_car = self.clone();
        run_car.run();
        return run_car.pos;
    }

//     pub fn is_in_intersection(&self)->bool{

//     }
 }