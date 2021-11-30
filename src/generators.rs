use std::{collections::HashMap};

use crate::maze::{Maze, Offset};
use rand::prelude::ThreadRng;

pub mod recursive_division;
pub mod ellers;

pub trait Generator {
    fn init(maze: &Maze, rand: ThreadRng, options: HashMap<&'static str, usize>) -> Self
        where Self: Sized;
    fn step(&mut self, maze: &Maze) -> Vec<GeneratorUpdate>;
    fn options(maze: &Maze) -> Vec<GeneratorOption>
        where Self: Sized;
}

pub struct GeneratorOption {
    pub min: usize,
    pub default: usize,
    pub max: usize,
    pub name: &'static str,
}

pub enum GeneratorUpdate {
    Connect(Offset, Offset),
    Disconnect(Offset, Offset),
    Primary(Offset),
    Secondary(Offset),
    Normal(Offset),
    Finished()
}
