use std::{collections::{BTreeSet, HashMap, HashSet}};

use rand::{Rng, prelude::{IteratorRandom, ThreadRng}};

use crate::maze::{Adjacency, Maze, Offset};

use super::{Generator, GeneratorOption, GeneratorUpdate};

pub struct Ellers {
    rng: ThreadRng,
    sets: Vec<HashSet<Offset>>,
    queue: BTreeSet<Offset>,
}

impl Generator for Ellers {
    fn init(maze: &Maze, rng: ThreadRng, _: HashMap<&'static str, usize>) -> Self
    {
        let queue: BTreeSet<Offset> = maze.cells.keys().copied().collect();


        Ellers { rng, sets: Vec::new(), queue }

    }

    fn step(&mut self, maze: &Maze) -> Vec<GeneratorUpdate> {
        let mut updates = vec![];
        let mut primaries = HashSet::new();
        let mut secondaries = HashSet::new();
        loop {
            if let Some(current_cell) = self.next_cell() {
                let mut current_set = self.sets.iter().position(|set| set.contains(&current_cell)).unwrap_or_else(|| {
                    let mut set = HashSet::new();
                    set.insert(current_cell);
                    self.sets.push(set);
                    self.sets.len() - 1
                });
                
                let mut neighbors = self.get_neighbors(maze, &self.sets[current_set]);       
                for &cell in &self.sets[current_set] {
                    secondaries.insert(cell);
                }

                self.sets[current_set].remove(&current_cell);

                if let Some(required_neighbor) = neighbors.iter().choose(&mut self.rng).copied() {
                    while let Some(neighbor) = neighbors.pop() {
                        if self.sets[current_set].contains(&neighbor.offset) {
                            continue;
                        }

                        if required_neighbor == neighbor || self.rng.gen_bool(0.5) {
                            if let Some(neighbor_set) = self.sets.iter().position(|set| set.contains(&neighbor.offset)) {
                                let removed_set = self.sets.remove(neighbor_set);
                                if neighbor_set < current_set {
                                    current_set -= 1;
                                }
                                let new_neighbors = self.get_neighbors(maze, &removed_set);
                                neighbors.extend(new_neighbors);
                                self.sets[current_set].extend(removed_set);
                            } else {
                                self.sets[current_set].insert(neighbor.offset);
                            }

                            primaries.insert(neighbor.cell);
                            primaries.insert(neighbor.offset);
                            updates.push(GeneratorUpdate::Connect(neighbor.cell, neighbor.offset));                     
                        }
                    }
                    if self.sets[current_set].is_empty() {
                        self.sets.remove(current_set);
                    }
                    break; 
                }
                
                if self.sets[current_set].is_empty() {
                    self.sets.remove(current_set);
                }
            } else {
                updates.push(GeneratorUpdate::Finished());
                break;
            }
        }
        updates.extend(secondaries.difference(&primaries).copied().map(GeneratorUpdate::Secondary));
        updates.extend(primaries.into_iter().map(GeneratorUpdate::Primary));
        updates
    }


    fn options(_: &Maze) -> Vec<GeneratorOption> {
        vec![]
    }
}

impl Ellers {
    fn next_cell(&mut self) -> Option<Offset> {
        let next = self.queue.iter().next().copied();
        if let Some(cell) = next {
            self.queue.remove(&cell);
        }
        next
    }

    fn get_neighbors(&self, maze: &Maze, set: &HashSet<Offset>) -> Vec<Adjacency> {
        let mut neighbors: Vec<Adjacency> = set.iter()
            .flat_map(|&offset| 
                maze.adjacencies(offset)
                    .filter(|adj| adj.offset > adj.cell && !set.contains(&adj.offset)))
            .collect();
        
        neighbors.sort_unstable_by_key(|f| f.offset);
        neighbors
    }
}