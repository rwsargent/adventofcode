use std::{fmt::{Display, self}, ops::{Sub, Add}};

#[derive(Debug, Default, PartialEq, Eq, Clone,Copy, Hash)]
pub struct Coord {
    pub x: i32, 
    pub y: i32,
}

impl Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coord {
    pub fn from_usize(x: usize, y:usize) -> Self{
        Coord { x: (x as i32), y: (y as i32) }
    }
    pub fn up(self) -> Self{
        let mut clone = self.clone();
        clone.y -= 1;
        clone
    }

    pub fn down(self) -> Self {
        let mut clone = self.clone();
        clone.y += 1;
        clone
    }

    pub fn left(self) -> Self {
        let mut clone = self.clone();
        clone.x -= 1;
        clone
    }

    pub fn right(self) -> Self {
        let mut clone = self.clone();
        clone.x += 1;
        clone
    }

    pub fn nor_e(self) -> Self {
        self.up().right()
    }

    pub fn se(self) -> Self {
        self.down().right()
    }

    pub fn sw(self) -> Self {
        self.down().left()
    }

    pub fn nw(self) -> Self {
        self.up().left()
    }

    pub fn dist(&self, other: &Coord) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2)) as f64).sqrt()
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![self.clone().up(), self.clone().down(), self.clone().left(), self.clone().right(),
             self.clone().nor_e(), self.clone().se(), self.clone().sw(), self.clone().nw()]
    }

    pub fn cardinal_neighbors(&self) -> Vec<Self> {
        vec![self.clone().up(), self.clone().down(), self.clone().right(), self.clone().left(),]
    }

    pub fn unitize(&self) -> Self{
        Coord {x: self.x.max(-1).min(1),
        y : self.y.max(-1).min(1)}
    }

    pub fn all_between(&self, other: &Coord) -> Vec<Coord> {
        let vec = (*other - *self).unitize();
        let mut out = Vec::new();
        let mut cursor = *self;
        while cursor != *other {
            out.push(cursor);
            cursor = cursor + vec;
        }
        out
    }

    pub fn manhattan(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn in_direction(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::N => self.down(),
            Direction::E => self.right(),
            Direction::S => self.up(),
            Direction::W => self.left(),
            Direction::NE => self.nor_e(),
            Direction::SE => self.se(),
            Direction::SW => self.sw(),
            Direction::NW => self.nw(),
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

pub enum Direction {
    N, E, S, W, NE, SE, SW, NW
}