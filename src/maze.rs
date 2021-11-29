use std::{collections::{HashMap, HashSet}, ops::{Add, Div, Mul, Sub}};


#[derive(Clone, Debug, PartialEq)]
pub struct Tiling {
    // Uniform tiling that consists of a set of polygons that can be stamped at the offsets given
    pub polygons: Vec<TilePolygon>,
    pub neighbors: Vec<Offset>,
}

pub struct TilingError {
    pub message: String,
}

impl Tiling {
    pub fn validate(&self) -> Result<(), TilingError> {
        let mut seen_neighbors = HashSet::new();
        
        if self.polygons.is_empty() {
            return Err(TilingError {
                message: format!("No polygons in tiling"),
            });
        }

        for (idx, neighbor) in self.neighbors.iter().enumerate() {
            if neighbor.coordinates == Coordinates::origin() {
                return Err(TilingError {
                    message: format!("Neighbor {} is at origin", idx),
                });
            }

            if !seen_neighbors.insert(*neighbor) {
                return Err(TilingError {
                    message: format!("Two neighbors have the same offset {:?}", neighbor),
                });
            }
        }

        let mut seen_polygons = HashSet::new();
        for (idx, polygon) in self.polygons.iter().enumerate() {
            if !seen_polygons.insert(polygon.offset) {
                return Err(TilingError {
                    message: format!("Two polygons have the same offset {:?}", polygon.offset),
                });
            }
            for (side_idx, side) in polygon.sides.iter().enumerate() {
                if side.other_tile > self.neighbors.len() {
                    return Err(TilingError {
                        message: format!("Side tile index {} out of bounds. Polygon {} Side {}", side.other_tile, idx, side_idx),
                    });
                }
                if side.other_polygon >= self.polygons.len() {
                    return Err(TilingError {
                        message: format!("Side polygon index {} out of bounds. Polygon {} Side {}", side.other_polygon, idx, side_idx),
                    });
                }
                let other_polygon = &self.polygons[side.other_polygon];

                if side.other_side_index >= other_polygon.sides.len() {
                    return Err(TilingError {
                        message: format!("Side polygon index {} out of bounds. Polygon {} Side {}", side.other_side_index, idx, side_idx),
                    });
                }

                let other_side = other_polygon.sides[side.other_side_index];

                if other_side.other_polygon != idx {
                    return Err(TilingError {
                        message: format!("Polygon sides are not reflexive.  Polygon {} Side {} and Polygon {} Side {} reference different polygons", idx, side_idx, side.other_polygon, side.other_side_index),
                    });
                }

                if other_side.other_side_index != side_idx {
                    return Err(TilingError {
                        message: format!("Polygon sides are not reflexive.  Polygon {} Side {} and Polygon {} Side {} reference different sides", idx, side_idx, side.other_polygon, side.other_side_index),
                    });
                }

                if (side.other_tile == 0) != (other_side.other_tile == 0) { 
                    return Err(TilingError {
                        message: format!("Polygon sides are not reflexive.  Polygon {} Side {} and Polygon {} Side {} reference different tiles", idx, side_idx, side.other_polygon, side.other_side_index),
                    });
                }

                if side.other_tile != 0 {
                    if self.neighbors[side.other_tile - 1] + self.neighbors[other_side.other_tile - 1] != Offset::origin() {
                        return Err(TilingError {
                            message: format!("Polygon sides are not reflexive.  Polygon {} Side {} and Polygon {} Side {} reference non-reflexive tiles", idx, side_idx, side.other_polygon, side.other_side_index),
                        });
                    }
                }

            }
        }
        Result::Ok(())
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
    pub coordinates: Coordinates
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        Offset {
            x: self.x + other.x,
            y: self.y + other.y,
            coordinates: self.coordinates + other.coordinates,
        }
    }
}

impl PartialEq for Offset {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::hash::Hash for Offset {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Offset {
    pub fn origin() -> Offset {
        Offset {
            x: 0,
            y: 0,
            coordinates: Coordinates { x: 0.0, y: 0.0 },
        }
    }
}

impl Eq for Offset {}

impl Sub for Offset {
    type Output = Offset;

    fn sub(self, other: Offset) -> Offset {
        Offset {
            x: self.x - other.x,
            y: self.y - other.y,
            coordinates: self.coordinates - other.coordinates,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

impl Coordinates {
    pub fn origin() -> Coordinates {
        Coordinates { x: 0.0, y: 0.0 }
    }
}

impl Div<f64> for Coordinates {
    type Output = Coordinates;

    fn div(self, other: f64) -> Coordinates {
        Coordinates {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Mul<f64> for Coordinates {
    type Output = Coordinates;

    fn mul(self, other: f64) -> Coordinates {
        Coordinates {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, other: Coordinates) -> Coordinates {
        Coordinates {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TilePolygon {
    pub offset: Offset,
    pub corners: Vec<Coordinates>,
    pub sides: Vec<PolygonSide>,  // In the order of the corners
}

impl TilePolygon {
    pub fn contains_point(&self, point: Coordinates) -> bool {
        let mut inside = false;
        for i in 0 .. self.corners.len() {
            let j = (i + 1) % self.corners.len();
            if (self.corners[i].y > point.y) != (self.corners[j].y > point.y) {
                if point.x < (self.corners[j].x - self.corners[i].x) * (point.y - self.corners[i].y) / (self.corners[j].y - self.corners[i].y) + self.corners[i].x {
                    inside = !inside;
                }
            }
        }

        inside
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PolygonSide {
    pub other_tile: usize, // 0 is self, otherwise references Tiling.neighbors[index-1]
    pub other_polygon: usize,
    pub other_side_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    pub offset: Offset,
    pub walls: Vec<bool>,
    polygon: usize,
}

impl Cell {
    pub fn is_open(&self) -> bool {
        self.walls.iter().any(|w| !*w)
    }
}

pub struct Maze {
    pub cells: HashMap<Offset, Cell>, // Offset -> index in Tiling.polygons
    tiling: Tiling,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn contains(self: Rectangle, coordinates: Coordinates) -> bool {
        coordinates.x >= self.x && coordinates.x <= self.x + self.width &&
            coordinates.y >= self.y && coordinates.y <= self.y + self.height
    }
}

impl Maze {
    pub fn new(tiling: Tiling, bounding_box: Rectangle, center: Coordinates) -> Maze {
        let start = Offset {
            x: 0,
            y: 0,
            coordinates: center,
        };

        if !bounding_box.contains(start.coordinates) {
            
            panic!("Bounding box does not contain center!");
        }

        if let Some(error) = tiling.validate().err() {
            panic!("Invalid tiling: {}", error.message);
        }

        let mut seen = HashSet::new();
        seen.insert(start);
        let mut queue = vec![start];
        let mut cells = HashMap::new();        
        
        while let Some(tile) = queue.pop() { 
            let mut added = false;
            for (index, polygon) in tiling.polygons.iter().enumerate() {
                let cell_offset = tile + polygon.offset;
                if !polygon.corners.iter().all(|corner| bounding_box.contains(cell_offset.coordinates + *corner)){
                    continue;
                }
                
                added = true;

                if cells.insert(cell_offset, Cell {
                    offset: cell_offset,
                    polygon: index,
                    walls: vec![true; polygon.sides.len()],
                }).is_some() {
                    panic!("Duplicate cell offset: {:?}", cell_offset);
                }
            }

            if added {
                for &neighbor in &tiling.neighbors {
                    let next_tile = tile + neighbor;
                    if seen.insert(next_tile) {
                        queue.push(next_tile);
                    }
                }
            }
        }
        
        if cells.is_empty() {
            panic!("Bounding box does not contain center!");
        }

        Maze { cells, tiling }
    }

    
    pub fn cell_containing_point(&self, coordinates: Coordinates) -> Option<&Cell> {
        for cell in self.cells.values() {
            let polygon = self.get_polygon(cell);
            if polygon.contains_point(coordinates - cell.offset.coordinates) {
                return Some(cell);
            }
        }

        None
    }

    fn adjacent_offsets<'a>(&'a self, offset: Offset) -> impl Iterator<Item = (usize, Offset)> + '_ {
        let cell = &self.cells[&offset]; 
        self.tiling.polygons[cell.polygon].sides.iter()
            .map(move |side| self.calculate_offset(cell, *side))
            .enumerate()
            .filter(move |(_, offset)| self.cells.contains_key(&offset))
    }

    pub fn adjacent_cells<'a>(&'a self, cell: Offset) -> impl Iterator<Item = (usize, &'a Cell)> + '_ {
        self.adjacent_offsets(cell).map(move |(side, offset)| (side, &self.cells[&offset]))
    }
 
    pub fn connect(&mut self, cell: Offset, other: Offset) {
        let (cell_wall, other_wall) = {
            if let Some((cell_wall, _)) = self.adjacent_offsets(cell).find(|(_, c)| *c == other) {
                if let Some((other_wall, _)) = self.adjacent_offsets(other).find(|(_, c)| *c == cell) {
                    (cell_wall, other_wall)
                } else {
                    panic!("Cell {:?} is not adjacent to {:?}", cell, other);
                }
            } else {
                panic!("Cell {:?} is not adjacent to {:?}", other, cell);
            }
        };
        
        self.cells.get_mut(&cell).unwrap().walls[cell_wall] = false;
        self.cells.get_mut(&other).unwrap().walls[other_wall] = false;
    }

    pub fn disconnect(&mut self, cell: Offset, other: Offset) {
        let (cell_wall, other_wall) = {
            if let Some((cell_wall, _)) = self.adjacent_offsets(cell).find(|(_, c)| *c == other) {
                if let Some((other_wall, _)) = self.adjacent_offsets(other).find(|(_, c)| *c == cell) {
                    (cell_wall, other_wall)
                } else {
                    panic!("Cell {:?} is not adjacent to {:?}", cell, other);
                }
            } else {
                panic!("Cell {:?} is not adjacent to {:?}", other, cell);
            }
        };
        
        self.cells.get_mut(&cell).unwrap().walls[cell_wall] = true;
        self.cells.get_mut(&other).unwrap().walls[other_wall] = true;
    }
    
    fn connected_offsets<'a>(&'a self, cell: &'a Cell) -> impl Iterator<Item = Offset> + '_ {
        self.tiling.polygons[cell.polygon].sides.iter()
            .zip(&cell.walls)
            .filter(|(_, b)| **b)
            .map(|(side, _)| side)
            .map(move |side| self.calculate_offset(cell, *side))
            .filter(move |offset| self.cells.contains_key(&offset))
    }

    pub fn connected_cells<'a>(&'a self, cell: &'a Cell) -> impl Iterator<Item = &'a Cell> + '_ {
        self.connected_offsets(cell).map(move |offset| &self.cells[&offset])
    }

    pub fn get_polygon(&self, cell: &Cell) -> &TilePolygon {
        &self.tiling.polygons[cell.polygon]
    }

    fn calculate_offset(&self, cell: &Cell, side: PolygonSide) -> Offset {
        let cell_polygon = &self.tiling.polygons[cell.polygon];
        let other_polygon = &self.tiling.polygons[side.other_polygon];
        let tile_offset = match side.other_tile {
            0 => Offset::origin(),
            x => self.tiling.neighbors[x - 1]
        };
        let offset = cell.offset - cell_polygon.offset + other_polygon.offset + tile_offset;

        if offset == cell.offset {
            panic!("Offset is the same as cell offset! Cell: {:?} Cell Polygon: {:?} Other Polygon: {:?} Other Tile: {:?}", cell.offset, cell_polygon, other_polygon, tile_offset);
        }

        offset
    }
}