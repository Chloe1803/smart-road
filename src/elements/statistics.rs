use crate::constants::*;

pub struct Statitics {
    pub max_number_of_vehicules : usize,
    pub max_velocity : f64,
    pub min_velocity : f64, 
    pub max_time : usize,
    pub min_time : usize,
    pub number_of_colision : usize
}

impl Statitics {
    pub fn new()->Self{
        Statitics{
            max_number_of_vehicules :0,
            max_velocity : 0.00,
            min_velocity : 1000000.00,
            max_time : 0,
            min_time : 10000000,
            number_of_colision : 0
        }
    }

    pub fn get_stats(&self)-> String {
        let mut stats = String::new();

        stats = format!("Statistics :
        Max number of vehicles that passed the intersection : {}
        Max velocity of all vehicles : {} px/seconds
        Min velocity of all vehicles : {} px/seconds
        Max time that the vehicles took to pass the intersection : {} seconds
        Min time that the vehicles took to pass the intersection : {} seconds
        Close calls : {}", self.max_number_of_vehicules, self.max_velocity, self.min_velocity, self.max_time, self.min_time, self.number_of_colision);

        return stats
    }
}