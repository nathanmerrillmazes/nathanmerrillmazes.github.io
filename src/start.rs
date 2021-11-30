#[cfg(console_error_panic_hook)]
extern crate console_error_panic_hook;

use std::collections::HashSet;

use rand::thread_rng;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{generators::{Generator, GeneratorUpdate, recursive_division::RecursiveDivision}, maze::*, tilings};

#[wasm_bindgen]
pub struct CanvasData {
    canvas: CanvasRenderingContext2d,
    bounding_box: Rectangle,
    center: Coordinates,
    maze: Maze,
    finished: bool,
    generator: Box<dyn Generator>,
    primary: HashSet<Offset>,
    secondary: HashSet<Offset>,
}

static HIGHLIGHT: &str = "#FAD02C";
static SECONDARY_HIGHLIGHT: &str = "#90ADC6";
static FILLED: &str = "#333652";
static OPEN: &str = "#E9EAEc";

impl CanvasData {
    pub fn new(canvas: HtmlCanvasElement, scale: f64, rotation: f64) -> Self {
        let width = canvas.get_attribute("width").and_then(|a| a.as_str().parse().ok()).unwrap_or(500.0);
        let height = canvas.get_attribute("height").and_then(|a| a.as_str().parse().ok()).unwrap_or(500.0);
        
        let context_element = canvas.get_context("2d").ok().unwrap().unwrap();
        let canvas_rendering = context_element.dyn_into::<web_sys::CanvasRenderingContext2d>().ok().unwrap();
    
        let bounding_box = Rectangle {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };
        let tiling = tilings().into_iter().find_map(|(name, tiling)|if name == "Square" {Some(tiling)} else {None}).unwrap();
        let center = Coordinates {
            x: bounding_box.width / 2.0,
            y: bounding_box.height / 2.0,
        };
    
        let maze = Maze::new(tiling, bounding_box, center, scale, rotation);
        
        let options = RecursiveDivision::options(&maze);
        let option_values = options.iter().map(|option| (option.name, option.default)).collect();

        Self {
            canvas: canvas_rendering,
            bounding_box,
            generator: Box::new(RecursiveDivision::init(&maze, thread_rng(), option_values)),
            maze,
            center,
            finished: false,
            primary: HashSet::new(),
            secondary: HashSet::new(),
        }
    }

    pub fn render(&self) {
        log::info!("{:?}", self.bounding_box);
        self.canvas.clear_rect(0.0, 0.0, self.bounding_box.width, self.bounding_box.height);
        
        for cell in self.maze.cells.values() {
            self.render_cell(cell);
        }
    }

    pub fn process_updates(&mut self, updates: Vec<GeneratorUpdate>) -> HashSet<Offset> {
        let mut renders = HashSet::new();
        for update in updates {
            match update {
                GeneratorUpdate::Connect(a, b) => {
                    self.maze.connect(a, b);
                    renders.insert(a);
                    renders.insert(b);
                }
                GeneratorUpdate::Disconnect(a, b) => {
                    self.maze.disconnect(a, b);
                    renders.insert(a);
                    renders.insert(b);
                }
                GeneratorUpdate::Finished() => {
                    self.finished = true;
                }
                GeneratorUpdate::Normal(cell) => {
                    renders.insert(cell);
                    self.primary.remove(&cell);
                    self.secondary.remove(&cell);
                }
                GeneratorUpdate::Primary(cell) => {
                    renders.insert(cell);
                    self.primary.insert(cell);
                    self.secondary.remove(&cell);
                }
                GeneratorUpdate::Secondary(cell) => {
                    renders.insert(cell);
                    self.primary.remove(&cell);
                    self.secondary.insert(cell);
                }
            }
        }
        renders
    }

    fn render_cell(&self, cell: &Cell) {
        let fill = if self.primary.contains(&cell.offset) {
            HIGHLIGHT
        } else if self.secondary.contains(&cell.offset) {
            SECONDARY_HIGHLIGHT
        } else if cell.is_open() {
            OPEN
        } else {
            FILLED
        };

        let polygon = self.maze.get_polygon(cell);

        // Fill:
        self.canvas.begin_path();
        for corner in polygon.corners {
            let corner_coordinates = cell.offset.coordinates + scale(rotate(*corner, Coordinates::origin(), self.maze.rotation), Coordinates::origin(), self.maze.scaling);
            self.canvas.line_to(corner_coordinates.x, corner_coordinates.y); 
        }
        self.canvas.close_path();
        self.canvas.set_fill_style(&JsValue::from_str(fill));
        self.canvas.fill();

        // Walls:
        for (i, corner) in polygon.corners.iter().enumerate() {
            if cell.walls[i] {
                let next_corner = polygon.corners[(i+1) % polygon.corners.len()];
                self.canvas.begin_path();
                let start_position = cell.offset.coordinates + scale(rotate(*corner, Coordinates::origin(), self.maze.rotation), Coordinates::origin(), self.maze.scaling);
                let stop_position = cell.offset.coordinates + scale(rotate(next_corner, Coordinates::origin(), self.maze.rotation), Coordinates::origin(), self.maze.scaling);
                self.canvas.move_to(start_position.x, start_position.y);
                self.canvas.line_to(stop_position.x, stop_position.y);
                self.canvas.stroke();
            }
        }
        
    }

    pub fn reset(&mut self, maze: Maze) {
        self.maze = maze;
        let options = RecursiveDivision::options(&self.maze);
        let option_values = options.iter().map(|option| (option.name, option.default)).collect();
        self.generator = Box::new(RecursiveDivision::init(&self.maze, thread_rng(), option_values));
        self.finished = false;
        self.primary.clear();
        self.secondary.clear();
        self.render();
    }

    pub fn step(&mut self, iterations: u32) -> bool {
        let mut renders = vec![];
        for _ in 0..iterations {
            let updates = self.generator.step(&mut self.maze);
            renders.extend(self.process_updates(updates));
        }
        
        for render in renders {
            if let Some(cell) = self.maze.cells.get(&render) {
                self.render_cell(cell);
            }
        }

        self.finished
    }
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn step(canvas_data: &mut CanvasData, iterations: u32) -> bool {
    canvas_data.step(iterations)
}

fn tilings() -> Vec<(&'static str, Tiling)> {
    vec![
        ("Square", tilings::SQUARE),
        ("Hexagon", tilings::HEX),
        ("Triangular", tilings::TRIANGLE),
        ("Truncated Square", tilings::TRUNCATED_SQUARE),
        ("Tetrakis Square", tilings::TETRAKIS_SQUARE),
        ("Snub Square", tilings::SNUB_SQUARE),
        ("Cairo Pentagonal", tilings::CAIRO_PENTAGONAL),
        ("Trihexagonal", tilings::TRIHEXAGONAL),
        ("Rhombille", tilings::RHOMBILLE),
        ("Truncated Hexagonal", tilings::TRUNCATED_HEX),
        ("Triakis", tilings::TETRAKIS),
    ]
}

#[wasm_bindgen(start)]
#[allow(dead_code)]
pub fn start() {
    #[cfg(console_error_panic_hook)]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn reset(canvas_data: &mut CanvasData) {
    canvas_data.reset(Maze::new(canvas_data.maze.tiling, canvas_data.bounding_box, canvas_data.center, canvas_data.maze.scaling, canvas_data.maze.rotation));
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn wasm_init(canvas: web_sys::HtmlCanvasElement, scale: f64, rotation: f64) -> CanvasData {
    let data = CanvasData::new(canvas, scale, rotation);
    data.render();
    data
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn get_tilings() -> js_sys::Array {
    let arr = js_sys::Array::new();
    for (name, _) in tilings().into_iter() {
        arr.push(&JsValue::from_str(name));
    }
    arr
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn set_tiling(tiling_name: String, canvas_data: &mut CanvasData) {
    let tiling = tilings().into_iter().find_map(|(tile_name, tiling)|if tiling_name == tile_name {Some(tiling)} else {None}).unwrap();
    canvas_data.reset(Maze::new(tiling, canvas_data.bounding_box, canvas_data.center, canvas_data.maze.scaling, canvas_data.maze.rotation));
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn set_rotation(rotation: f64, canvas_data: &mut CanvasData) {
    canvas_data.reset(Maze::new(canvas_data.maze.tiling, canvas_data.bounding_box, canvas_data.center, canvas_data.maze.scaling, rotation));
}

#[wasm_bindgen]
#[allow(dead_code)]
pub fn set_scale(scale: f64, canvas_data: &mut CanvasData) {
    log::info!("Set Scale");
    canvas_data.reset(Maze::new(canvas_data.maze.tiling, canvas_data.bounding_box, canvas_data.center, scale, canvas_data.maze.rotation));
}
