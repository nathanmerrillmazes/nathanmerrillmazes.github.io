use std::{collections::{HashMap, HashSet}, ops::{Add, Div, Mul, Sub}};


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tiling {
    // Uniform tiling that consists of a set of polygons that can be stamped at the offsets given
    pub polygons: &'static[TilePolygon],
    pub neighbors: &'static[Offset],
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TilePolygon {
    pub offset: Offset,
    pub corners: &'static [Coordinates],
    pub sides: &'static [PolygonSide],  // In the order of the corners
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
    pub tiling: Tiling,
    pub scaling: f64,
    pub rotation: f64
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn contains(self, coordinates: Coordinates) -> bool {
        coordinates.x >= self.x && coordinates.x <= self.x + self.width &&
            coordinates.y >= self.y && coordinates.y <= self.y + self.height
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Adjacency {
    pub index: usize,
    pub offset: Offset
}

pub fn rotate(point: Coordinates, origin: Coordinates, angle: f64) -> Coordinates {
    let distance = point - origin;
    let sin = angle.to_radians().sin();
    let cos = angle.to_radians().cos();
    Coordinates {
        x: origin.x + cos * distance.x - sin * distance.y,
        y: origin.y + sin * distance.x + cos * distance.y,
    }
}

pub fn scale(point: Coordinates, origin: Coordinates, scale: f64) -> Coordinates {
    (point - origin) * scale + origin
}

impl Maze {
    pub fn new(tiling: Tiling, bounding_box: Rectangle, center: Coordinates, scaling: f64, rotation: f64) -> Maze {
        let start = Offset {
            x: 0,
            y: 0,
            coordinates: center,
        };

        debug_assert!(bounding_box.contains(start.coordinates), "Bounding box does not contain center!");
        debug_assert!(tiling.validate().is_ok(), "{}", tiling.validate().err().unwrap().message);

        let mut seen = HashSet::new();
        seen.insert(start);
        let mut queue = vec![start];
        let mut cells = HashMap::new();        
        
        while let Some(tile) = queue.pop() { 
            let mut added = false;
            for (index, polygon) in tiling.polygons.iter().enumerate() {
                let mut cell_offset = tile + polygon.offset;
                cell_offset.coordinates = scale(rotate(cell_offset.coordinates, center, rotation), center, scaling);

                if !polygon.corners.iter().all(|corner| {
                    let corner_coordinates = scale(rotate(*corner, Coordinates::origin(), rotation), Coordinates::origin(), scaling);
                    bounding_box.contains(corner_coordinates + cell_offset.coordinates)
                }){
                    continue;
                }
                
                added = true;

                let inserted = cells.insert(cell_offset, Cell {
                    offset: cell_offset,
                    polygon: index,
                    walls: vec![true; polygon.sides.len()],
                });
                debug_assert!(inserted.is_none(), "Duplicate cell offset: {:?}", cell_offset)
            }

            if added {
                for &neighbor in tiling.neighbors {
                    let next_tile = tile + neighbor;
                    if seen.insert(next_tile) {
                        queue.push(next_tile);
                    }
                }
            }
        }

        debug_assert!(!cells.is_empty(), "Bounding box does not contain center!");

        Maze { cells, tiling, scaling, rotation }
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

    pub fn adjacencies<'a>(&'a self, offset: Offset) -> impl Iterator<Item = Adjacency> + '_ {
        let cell = &self.cells[&offset]; 
        self.tiling.polygons[cell.polygon].sides.iter()
            .enumerate()
            .filter_map(move |(index, side)|  {
                let offset = self.calculate_offset(cell, *side);
                if self.cells.contains_key(&offset) {
                    Some(Adjacency{index, offset})
                } else {
                    None
                }
            })
    }
 
    pub fn connect(&mut self, cell: Offset, other: Offset) {
        self.set_wall(cell, other, false);
        self.set_wall(other, cell, false);
    }

    pub fn disconnect(&mut self, cell: Offset, other: Offset) {
        self.set_wall(cell, other, true);
        self.set_wall(other, cell, true);        
    }

    fn set_wall(&mut self, cell: Offset, to: Offset, wall: bool) {
        let adjacency = self.adjacencies(cell).find(|adj| adj.offset == to);

        if let Some(cell_adjacency) = adjacency {
            self.cells.get_mut(&cell).unwrap().walls[cell_adjacency.index] = wall;
        } else {
            debug_assert!(false, "Cell {:?} is not adjacent to {:?}", to, cell);
        }
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

    pub fn get_polygon(&self, cell: &Cell) -> TilePolygon {
        self.tiling.polygons[cell.polygon]
    }

    fn calculate_offset(&self, cell: &Cell, side: PolygonSide) -> Offset {
        let cell_polygon = &self.tiling.polygons[cell.polygon];
        let other_polygon = &self.tiling.polygons[side.other_polygon];
        let tile_offset = match side.other_tile {
            0 => Offset::origin(),
            x => self.tiling.neighbors[x - 1]
        };
        let offset = cell.offset - cell_polygon.offset + other_polygon.offset + tile_offset;

        debug_assert!(offset != cell.offset, "Offset is the same as cell offset! Cell: {:?} Cell Polygon: {:?} Other Polygon: {:?} Other Tile: {:?}", cell.offset, cell_polygon, other_polygon, tile_offset);

        offset
    }
}