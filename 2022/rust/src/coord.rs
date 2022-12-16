use std::fmt::{Display, self};

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
}