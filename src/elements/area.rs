use crate::elements::car::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x : i32,
    pub y : i32
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub a: Point,
    pub b: Point,
    pub c : Point,
    pub d : Point
}

#[derive(Clone, Debug)]
pub struct LockedArea {
    pub area: Rect,
    pub tickets: Vec<usize>, // Stocke l'ID de la voiture plutôt que la référence
    pub current_ticket: Option<(usize, bool)>,
}

impl LockedArea {
    fn new(area: Rect) -> Self {
        LockedArea {
            area,
            tickets: vec![],
            current_ticket: None,
        }
    }

    pub fn update_queue(&mut self,  cars_crossing: &Vec<(Rc<RefCell<Car>>, usize, Vec<(usize, usize)>)>) {

        //si la zone n'est pas réservé donc pas de mise a jour à faire
        if self.tickets.is_empty() && self.current_ticket.is_none() {
            return;
        }

        //si il y a bien un current_ticket
        if let Some((current_car_id, has_passed)) = self.current_ticket {
            
            //récupération de la voiture qui est en train de traverser si elle est dans la liste des crossing_cars
            if let Some((current_car, _, _)) = cars_crossing.iter().find(|(car, _, _)| car.borrow().id == current_car_id) {
                let current_car = current_car.borrow();

                //si la voiture est pas dans la zone et est passé
                if !current_car.is_in_intersection(&self.area) && has_passed {
                    
                    //alors on la retire du current ticket
                    self.current_ticket = None;
                //sinon si la voiture est dans la zone
                } else if current_car.is_in_intersection(&self.area) {
                    //mettre la valeur à jour de has passed pour dire que la voiture est bien passé
                    
                    self.current_ticket = Some((current_car_id, true));
                
                }else if !current_car.is_in_intersection(&self.area) && !has_passed{
                    //si la voiture n'est pas dans la liste des voitures en train de traverser, vérifie que c'est bien le ticket le plus ancien qui est current
                    if let Some(&min_id)= self.tickets.iter().min() {
                        if min_id < current_car_id {
                            self.current_ticket = Some((min_id, false));
                            self.tickets.push(current_car_id);
                            self.tickets.retain(|&x| x != min_id);
                        }
                    }
                }
            }else{
                self.current_ticket = None;
            }


        }

        //s'il n'y pas de ticket en cours, prendre le premier dans la queue
        if self.current_ticket.is_none() {
            self.tickets.sort();
            if let Some(car_id) = self.tickets.pop() {
                self.current_ticket = Some((car_id, false));
            }
        }
    }

    pub fn add_to_queue(&mut self, car_id: usize) {
        self.tickets.push(car_id, );
    }
}

pub fn get_locked_area(rect: Rect) -> Vec<Vec<LockedArea>> {
    const GRID_SIZE: usize = 6;

    let mut areas = vec![vec![]; GRID_SIZE - 2];
    
    // Calculate the width and height of each sub-area
    let width = rect.b.x - rect.a.x;
    let height = rect.a.y - rect.c.y;
    
    let sub_width = width / GRID_SIZE as i32;
    let sub_height = height / GRID_SIZE as i32;
    
    for i in 1..GRID_SIZE - 1 {
        let mut row = vec![];
        for j in 1..GRID_SIZE - 1 {
            let a = Point {
                x: rect.a.x + j as i32 * sub_width,
                y: rect.a.y - i as i32 * sub_height,
            };

            let b = Point {
                x: a.x + sub_width,
                y: a.y,
            };

            let c = Point {
                x: a.x + sub_width,
                y: a.y - sub_height,
            };

            let d = Point {
                x: a.x,
                y: a.y - sub_height,
            };

            let sub_rect = Rect { a, b, c, d };
            let locked_area = LockedArea::new(sub_rect);

            row.push(locked_area);
        }
        areas[i - 1] = row;
    }

    areas
}


impl Rect {
    // Vérifie si deux rectangles se chevauchent
    pub fn overlaps(rect1: &Rect, rect2: &Rect) -> bool {
    
        if rect1.b.x < rect2.a.x || rect2.b.x < rect1.a.x {
            return false;
        }

        if rect1.d.y > rect2.a.y || rect2.d.y > rect1.a.y {
            return false;
        }
        true
    }

}