use crate::{constants::*, Cardinal, Direction};
use crate::elements::area::*;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct Car {
    pub id: usize,
    pub from: Cardinal,       
    pub to: Cardinal,
    pub direction : Direction,  
    pub speed : i32,        
    pub pos: (i32, i32),        // current car position
    pub turning_point : (i32, i32), //position of when to turn
    pub running: bool,          // car is allowed to move this frame
    pub turned: bool,           // car doesn't need to turn anymore
    pub passed: bool,
    pub was_in_intersection : bool,
    pub color: u8,              // car color according to Cardinals from and to
    pub timestamp: Option<Instant>,
    pub distance : i32
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

                pos_y = - CAR_LENGTH / 2;
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
                pos_y = HEIGHT + CAR_LENGTH / 2;
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
                pos_x= - CAR_LENGTH / 2;
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
                pos_x= WIDTH + CAR_LENGTH/2;
                turn_y = pos_y;
            }
        }

        pos = (pos_x, pos_y);
        turning_point = (turn_x, turn_y);

        // calculate color and store it so that we don't do that every frame 
        let color: u8;
        //speed
        let speed:i32;
        match direction {
            Direction::Right => {speed = SPEED_MEDIUM; color= 1}
            Direction::Left => {speed = SPEED_FAST; color = 2 }
            Direction::Straight => {speed = SPEED_SLOW; color = 3}
        }

        return Car{
            id,
            from,
            to: cardinal_to,
            direction,
            speed,
            pos,
            turning_point,
            running: true,
            was_in_intersection : false,
            passed: false,
            turned: Cardinal::opposite(from, cardinal_to), // turned is set to true if Cardinals are opposite
            color,
            timestamp: Some(Instant::now()),
            distance :0
        };
    }

    // returns distance between those cars. 
    pub fn distance(&self, car: &Car) -> i32 {
        Car::distance_from_pos(self.pos, car.pos)
    }

    // calculates distance from car positions. formula is wrong, but simplified for faster calculus
    pub fn distance_from_pos(v1: (i32,i32), v2: (i32, i32)) -> i32 {
        std::cmp::max((v1.0 - v2.0).abs(), (v1.1 - v2.1).abs()) - CAR_LENGTH
    }

    // move the car
    pub fn run(&mut self) {
        if !self.running  {
            return;
        }
        self.distance += self.speed;
        if self.turned {
            // car passed its turn point. it should go toward its "to" Cardinal
            match self.to {
                Cardinal::N => {self.pos.1 -= self.speed;}
                Cardinal::S => {self.pos.1 += self.speed;}
                Cardinal::W => {self.pos.0 -= self.speed;}
                Cardinal::E => {self.pos.0 += self.speed;}
            }
        } else {
            // car should move away from its "from" Cardinal
            match self.from {
                Cardinal::N => {self.pos.1 += self.speed;}
                Cardinal::S => {self.pos.1 -= self.speed;}
                Cardinal::W => {self.pos.0 += self.speed;}
                Cardinal::E => {self.pos.0 -= self.speed;}
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
            Cardinal::N => self.pos.1 <= -CAR_LENGTH / 2,
            Cardinal::S => self.pos.1 >= HEIGHT + CAR_LENGTH / 2,
            Cardinal::W => self.pos.0 <= -CAR_LENGTH / 2,
            Cardinal::E => self.pos.0 >= WIDTH + CAR_LENGTH / 2
        }
    }



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


 }