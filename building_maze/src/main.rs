use std::collections::{HashMap, HashSet};
use std::hash::Hash;


pub type Cell = (u32, u32);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Wall { Top, Right, Bottom, Left }

#[derive(Debug, Clone)]
pub struct Maze { 
    width: u32,
    height: u32,
    walls: HashMap<Cell, HashSet<Wall>>,
}

impl Maze {
    pub fn new(width:u32, height:u32) -> Maze {
        let walls = HashMap::new();
        Maze { width, height, walls }
    }

    pub fn add_cell(&mut self, cell: Cell, walls: &[Wall]) -> &mut Maze {
        self.walls.insert(cell, walls.into_iter().copied().collect());
        self
    }

    
}


fn main() {
    let mut maze = Maze::new(2,2);
    maze
    .add_cell((0,0), &[Wall::Left, Wall::Right])
    .add_cell((1,0), &[Wall::Left, Wall::Bottom])
    .add_cell((1,1), &[Wall::Bottom, Wall::Right])
    .add_cell((0,1), &[Wall::Top, Wall::Left]);
    println!("Maze: {:?}", &maze);
}
