use std::{cmp::{max, min}, collections::{HashMap, HashSet}};

use rand::prelude::{IteratorRandom, ThreadRng};

use crate::maze::{Maze, Offset};

use super::{Generator, GeneratorOption, GeneratorUpdate};

pub struct RecursiveDivisionNode {
    stack: Vec<Offset>,
    cells: HashSet<Offset>,
    id: usize,
    current: Offset,
    finished: bool,
}

pub struct RecursiveDivision {
    rng: ThreadRng,
    nodes: Vec<RecursiveDivisionNode>,
    finished: bool,
}

impl Generator for RecursiveDivision {
    fn step(&mut self, maze: &Maze) -> Vec<GeneratorUpdate> {
        let mut changes = Vec::new();

        if !self.finished {

            for node_idx in 0..self.nodes.len() {
                
                let next_neighbor = {
                    let adjacent_cells: Vec<Offset> = 
                    maze.adjacent_cells(self.nodes[node_idx].current)
                        .map(|(_, cell)| cell.offset)
                        .filter(|offset| {
                            for other in &self.nodes {
                                if other.id == self.nodes[node_idx].id && other.cells.contains(offset) {
                                    return false;
                                }
                            }
                            return true
                        })
                        .collect();

                    adjacent_cells.iter().choose(&mut self.rng).copied()
                };

                let mut updates = None;

                if let Some(next) = next_neighbor {
                    for node in &self.nodes {
                        if node.id != node_idx && node.cells.contains(&next) {
                            let new_id = min(node.id, self.nodes[node_idx].id);
                            let old_id = max(node.id, self.nodes[node_idx].id);
                            updates = Some((new_id, old_id));
                            break;
                        }
                    }

                    if let Some((new_id, old_id)) = updates {
                        for node in &mut self.nodes {
                            if node.id == old_id {
                                node.id = new_id;
                            }
                        }
                    }
                }

                let node = &mut self.nodes[node_idx];

                if let Some(next) = next_neighbor {
                    changes.push(GeneratorUpdate::Connect(node.current, next));
                    changes.push(GeneratorUpdate::Secondary(node.current));
                    changes.push(GeneratorUpdate::Primary(next));
                    node.cells.insert(next);
                    node.stack.push(node.current);
                    node.current = next;
                } else {
                    changes.push(GeneratorUpdate::Normal(node.current));

                    if let Some(backtrack) = node.stack.pop() {
                        changes.push(GeneratorUpdate::Primary(backtrack));
                        node.current = backtrack;
                    } else {
                        node.finished = true;
                    }
                }
            }
            if self.nodes.iter().all(|n| n.finished){
                self.finished = true;
                changes.push(GeneratorUpdate::Finished());
            }
        } else {
            changes.push(GeneratorUpdate::Finished());
        }

        changes
    }

    fn init(maze: &Maze, mut rng: ThreadRng, options: HashMap<&'static str, usize>) -> Self {
        let mut nodes = Vec::new();
        let mut selected = HashSet::new();
        for i in 0..options["Threads"] {
            let current = loop {
                let current = *maze.cells.keys().choose(&mut rng).unwrap();
                if selected.insert(current) {
                    break current;
                }
            };

            let mut cells = HashSet::new();
            cells.insert(current);

            nodes.push(RecursiveDivisionNode {
                stack: Vec::new(),
                cells,
                id: i,
                current,
                finished: false,
            });
        }
        RecursiveDivision {
            nodes,
            rng,
            finished: false,
        }
    }

    fn options(maze: &Maze) -> Vec<GeneratorOption> {
        vec![
            GeneratorOption {
                min: 1,
                default: 10,
                max: maze.cells.len() / 4,
                name: "Threads",
            },
        ]
    }
}