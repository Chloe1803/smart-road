use rand::*;

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone, Hash)]
pub enum Cardinal {
    N,
    S,
    W,
    E
}

impl Cardinal {
    // true if Cardinals are opposite
    pub fn opposite(a: Cardinal, b: Cardinal) -> bool {
        a == Cardinal::N && b == Cardinal::S
        || a == Cardinal::S && b == Cardinal::N
        || a == Cardinal::E && b == Cardinal::W
        || a == Cardinal::W && b == Cardinal::E
    }

    // true if b is the Cardinal on the right of a
    pub fn right(a: Cardinal, b: Cardinal) -> bool {
        a == Cardinal::N && b == Cardinal::W
        || a == Cardinal::S && b == Cardinal::E
        || a == Cardinal::E && b == Cardinal::N
        || a == Cardinal::W && b == Cardinal::S
    }

    // returns a random Cardinal
    pub fn random() -> Cardinal {
        let x = rand::thread_rng().gen_range(0..4);
        match x {
            0 => Cardinal::N,
            1 => Cardinal::S,
            2 => Cardinal::W,
            3 => Cardinal::E,
            _ => Cardinal::N
        }
    }

}

#[derive(Debug, Eq, PartialEq, PartialOrd, Copy, Clone, Hash)]
pub enum Direction {
    Right,
    Straight,
    Left
}

impl Direction {
    pub fn random() -> Direction {
        let x = rand::thread_rng().gen_range(0..3);
        match x {
            0 => Direction::Right,
            1 => Direction::Straight,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
    
}