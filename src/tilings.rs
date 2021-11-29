use crate::maze::{Coordinates, Offset, PolygonSide, TilePolygon, Tiling};

const SQRT_3: f64 = 1.7320508075688772;

impl Tiling {
    pub fn hex() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -1,
                    coordinates: Coordinates { x: 0.0, y: -SQRT_3 },
                },
                Offset {
                    x: 1,
                    y: -1,
                    coordinates: Coordinates { x: 1.5, y: -SQRT_3 / 2.0 },
                },
                Offset {
                    x: 1,
                    y: 0,
                    coordinates: Coordinates { x: 1.5, y: SQRT_3 / 2.0 },
                },
                Offset {
                    x: 0,
                    y: 1,
                    coordinates: Coordinates { x: 0.0, y: SQRT_3 },
                },
                Offset {
                    x: -1,
                    y: 1,
                    coordinates: Coordinates { x: -1.5, y: SQRT_3 / 2.0 },
                },
                Offset {
                    x: -1,
                    y: 0,
                    coordinates: Coordinates { x: -1.5, y: -SQRT_3 / 2.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -SQRT_3 / 2.0 },
                        Coordinates { x: 0.5, y: -SQRT_3 / 2.0 },
                        Coordinates { x: 1.0, y: 0.0 },
                        Coordinates { x: 0.5, y: SQRT_3 / 2.0 },
                        Coordinates { x: -0.5, y: SQRT_3 / 2.0 },
                        Coordinates { x: -1.0, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 5,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                    ]
                },
            ],
        }
    }
    
    pub fn triangle() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: -1,
                    y: -1,
                    coordinates: Coordinates { x: -1.0, y: -SQRT_3 },
                },
                Offset {
                    x: 1,
                    y: -1,
                    coordinates: Coordinates { x: 1.0, y: -SQRT_3 },
                },
                Offset {
                    x: 1,
                    y: 1,
                    coordinates: Coordinates { x: 1.0, y: SQRT_3 },
                },
                Offset {
                    x: -1,
                    y: 1,
                    coordinates: Coordinates { x: -1.0, y: SQRT_3 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: -1,
                        coordinates: Coordinates { x: 0.0, y: -SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -SQRT_3 / 2.0 },
                        Coordinates { x: 1.0, y: SQRT_3 / 2.0 },
                        Coordinates { x: -1.0, y: SQRT_3 / 2.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: -SQRT_3 / 2.0 },
                        Coordinates { x: 1.0, y: -SQRT_3 / 2.0 },
                        Coordinates { x: 0.0, y: SQRT_3 / 2.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                    ]
                },
            ],
        }
    }
    pub fn square() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -1,
                    coordinates: Coordinates { x: 0.0, y: -1.0 },
                },
                Offset {
                    x: 1,
                    y: 0,
                    coordinates: Coordinates { x: 1.0, y: 0.0 },
                },
                Offset {
                    x: 0,
                    y: 1,
                    coordinates: Coordinates { x: 0.0, y: 1.0 },
                },
                Offset {
                    x: -1,
                    y: 0,
                    coordinates: Coordinates { x: -1.0, y: 0.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -0.5 },
                        Coordinates { x: 0.5, y: -0.5 },
                        Coordinates { x: 0.5, y: 0.5 },
                        Coordinates { x: -0.5, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
            ],            
        }
    }
    pub fn truncated_square() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -2,
                    coordinates: Coordinates { x: 0.0, y: -2.0 },
                },
                Offset {
                    x: 2,
                    y: 0,
                    coordinates: Coordinates { x: 2.0, y: 0.0 },
                },
                Offset {
                    x: 0,
                    y: 2,
                    coordinates: Coordinates { x: 0.0, y: 2.0 },
                },
                Offset {
                    x: -2,
                    y: 0,
                    coordinates: Coordinates { x: -2.0, y: 0.0 },
                },
                Offset {
                    x: -2,
                    y: -2,
                    coordinates: Coordinates { x: -2.0, y: -2.0 },
                },
                Offset {
                    x: 2,
                    y: 2,
                    coordinates: Coordinates { x: 2.0, y: 2.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -1.0 },
                        Coordinates { x: 0.5, y: -1.0 },
                        Coordinates { x: 1.0, y: -0.5 },
                        Coordinates { x: 1.0, y: 0.5 },
                        Coordinates { x: 0.5, y: 1.0 },
                        Coordinates { x: -0.5, y: 1.0 },
                        Coordinates { x: -1.0, y: 0.5 },
                        Coordinates { x: -1.0, y: -0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 0,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 6,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 1,
                        coordinates: Coordinates { x: 1.0, y: 1.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -0.5 },
                        Coordinates { x: 0.5, y: 0.0 },
                        Coordinates { x: 0.0, y: 0.5 },
                        Coordinates { x: -0.5, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 5,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 0,
                            other_side_index: 7,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                    ]
                },
            ],            
        }
    }
    pub fn tetrakis_square() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -2,
                    coordinates: Coordinates { x: 0.0, y: -2.0 },
                },
                Offset {
                    x: 2,
                    y: 0,
                    coordinates: Coordinates { x: 2.0, y: 0.0 },
                },
                Offset {
                    x: 0,
                    y: 2,
                    coordinates: Coordinates { x: 0.0, y: 2.0 },
                },
                Offset {
                    x: -2,
                    y: 0,
                    coordinates: Coordinates { x: -2.0, y: 0.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: -0.5 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: -0.5 },
                        Coordinates { x: 1.0, y: -0.5 },
                        Coordinates { x: 0.0, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 0,
                        coordinates: Coordinates { x: 0.5, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.5, y: -1.0 },
                        Coordinates { x: 0.5, y: 1.0 },
                        Coordinates { x: -0.5, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 3,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 1,
                        coordinates: Coordinates { x: 0.0, y: 0.5 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -0.5 },
                        Coordinates { x: 1.0, y: 0.5 },
                        Coordinates { x: -1.0, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: -0.5, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -1.0 },
                        Coordinates { x: 0.5, y: 0.0 },
                        Coordinates { x: -0.5, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                    ]
                },
            ],            
        }
    }
    pub fn snub_square() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 1,
                    y: -3,
                    coordinates: Coordinates { x: 1.0, y: -2.0 - SQRT_3 },
                },
                Offset {
                    x: 3,
                    y: 1,
                    coordinates: Coordinates { x: 2.0 + SQRT_3, y: 1.0 },
                },
                Offset {
                    x: -1,
                    y: 3,
                    coordinates: Coordinates { x: -1.0, y: 2.0 + SQRT_3 },
                },
                Offset {
                    x: -3,
                    y: -1,
                    coordinates: Coordinates { x: -2.0 - SQRT_3, y: -1.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: -1.0 },
                        Coordinates { x: 1.0, y: -1.0 },
                        Coordinates { x: 1.0, y: 1.0 },
                        Coordinates { x: -1.0, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 3,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 4,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 0,
                        coordinates: Coordinates { x: 1.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -1.0 },
                        Coordinates { x: SQRT_3, y: 0.0 },
                        Coordinates { x: 0.0, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 5,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 2,
                        y: 1,
                        coordinates: Coordinates { x: 1.0, y: 1.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: SQRT_3, y: -1.0 },
                        Coordinates { x: SQRT_3, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 5,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: -1.0, y: 1.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 1.0, y: SQRT_3 },
                        Coordinates { x: -1.0, y: SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 4,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 5,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 1,
                        coordinates: Coordinates { x: 0.0, y: 1.0 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: 0.0 },
                        Coordinates { x: 1.0, y: 0.0 },
                        Coordinates { x: 0.0, y: SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 5,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 2,
                        y: 2,
                        coordinates: Coordinates { x: 1.0, y: 2.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -1.0 },
                        Coordinates { x: SQRT_3, y: 0.0 },
                        Coordinates { x: SQRT_3-1.0, y: SQRT_3 },
                        Coordinates { x: -1.0, y: SQRT_3-1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 3,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 4,
                            other_side_index: 1,
                        },
                    ]
                },
            ],            
        }
    }
    pub fn cairo_pentagonal() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: -2,
                    y: -1,
                    coordinates: Coordinates { x: -2.0, y: -1.0 },
                },
                Offset {
                    x: 1,
                    y: -2,
                    coordinates: Coordinates { x: 1.0, y: -2.0 },
                },
                Offset {
                    x: 2,
                    y: 1,
                    coordinates: Coordinates { x: 2.0, y: 1.0 },
                },
                Offset {
                    x: -1,
                    y: 2,
                    coordinates: Coordinates { x: -1.0, y: 2.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: -0.5 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -1.0 },
                        Coordinates { x: 0.5, y: -1.0 },
                        Coordinates { x: 0.5, y: 0.0 },
                        Coordinates { x: 0.0, y: 0.5 },
                        Coordinates { x: -0.5, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 3,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 2,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 0,
                        coordinates: Coordinates { x: 0.5, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: 0.0 },
                        Coordinates { x: 0.0, y: -0.5 },
                        Coordinates { x: 1.0, y: -0.5 },
                        Coordinates { x: 1.0, y: 0.5 },
                        Coordinates { x: 0.0, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 2,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 3,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: 0.0, y: 0.5 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: 0.0 },
                        Coordinates { x: 0.0, y: -0.5 },
                        Coordinates { x: 0.5, y: 0.0 },
                        Coordinates { x: 0.5, y: 1.0 },
                        Coordinates { x: -0.5, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 3,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: -1,
                        y: 0,
                        coordinates: Coordinates { x: -0.5, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: -0.5 },
                        Coordinates { x: 0.0, y: -0.5 },
                        Coordinates { x: 0.5, y: 0.0 },
                        Coordinates { x: 0.0, y: 0.5 },
                        Coordinates { x: -1.0, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                    ]
                },
            ],           
        }
    }
    pub fn trihexagonal() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -2,
                    coordinates: Coordinates { x: 0.0, y: -2.0 },
                },
                Offset {
                    x: 2,
                    y: -1,
                    coordinates: Coordinates { x: SQRT_3, y: -1.0 },
                },
                Offset {
                    x: 2,
                    y: 1,
                    coordinates: Coordinates { x: SQRT_3, y: 1.0 },
                },
                Offset {
                    x: 0,
                    y: 2,
                    coordinates: Coordinates { x: 0.0, y: 2.0 },
                },
                Offset {
                    x: -2,
                    y: 1,
                    coordinates: Coordinates { x: -SQRT_3, y: 1.0 },
                },
                Offset {
                    x: -2,
                    y: -1,
                    coordinates: Coordinates { x: -SQRT_3, y: -1.0 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -SQRT_3/2.0, y: -0.5 },
                        Coordinates { x: 0.0, y: -1.0 },
                        Coordinates { x: SQRT_3/2.0, y: -0.5 },
                        Coordinates { x: SQRT_3/2.0, y: 0.5 },
                        Coordinates { x: 0.0, y: 1.0 },
                        Coordinates { x: -SQRT_3/2.0, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: -1,
                        y: 0,
                        coordinates: Coordinates { x: -SQRT_3/2.0, y: 0.5 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: SQRT_3/2.0, y: 0.5 },
                        Coordinates { x: 0.0, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 0,
                        coordinates: Coordinates { x: SQRT_3/2.0, y: 0.5 },
                    },
                    corners: vec![
                        Coordinates { x: -SQRT_3/2.0, y: 0.5 },
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 0.0, y: 1.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 5,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
            ],           
        }
    }
    pub fn rhombille() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -2,
                    coordinates: Coordinates { x: 0.0, y: -2.0*SQRT_3 },
                },
                Offset {
                    x: 2,
                    y: -2,
                    coordinates: Coordinates { x: 3.0, y: -SQRT_3 },
                },
                Offset {
                    x: 2,
                    y: 0,
                    coordinates: Coordinates { x: 3.0, y: SQRT_3 },
                },
                Offset {
                    x: 0,
                    y: 2,
                    coordinates: Coordinates { x: 0.0, y: 2.0*SQRT_3 },
                },
                Offset {
                    x: -2,
                    y: 2,
                    coordinates: Coordinates { x: -3.0, y: SQRT_3 },
                },
                Offset {
                    x: -2,
                    y: 0,
                    coordinates: Coordinates { x: -3.0, y: -SQRT_3 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: -SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: -1.0, y: 0.0 },
                        Coordinates { x: 1.0, y: 0.0 },
                        Coordinates { x: 2.0, y: SQRT_3 },
                        Coordinates { x: 0.0, y: SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: 0.0, y: SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -SQRT_3 },
                        Coordinates { x: 2.0, y: -SQRT_3 },
                        Coordinates { x: 1.0, y: 0.0 },
                        Coordinates { x: -1.0, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 2,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: -1,
                        y: 0,
                        coordinates: Coordinates { x: -1.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: -SQRT_3 },
                        Coordinates { x: 1.0, y: 0.0 },
                        Coordinates { x: 0.0, y: SQRT_3 },
                        Coordinates { x: -1.0, y: 0.0 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 3,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                    ]
                },
            ],           
        }
    }
    pub fn truncated_hex() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: 0,
                    y: -2,
                    coordinates: Coordinates { x: 0.0, y: -2.0 - SQRT_3 },
                },
                Offset {
                    x: 2,
                    y: -2,
                    coordinates: Coordinates { x: 1.5 + SQRT_3, y: -1.0 - 0.5*SQRT_3 },
                },
                Offset {
                    x: 2,
                    y: 0,
                    coordinates: Coordinates { x: 1.5 + SQRT_3, y: 1.0 + 0.5*SQRT_3 },
                },
                Offset {
                    x: 0,
                    y: 2,
                    coordinates: Coordinates { x: 0.0, y: 2.0 + SQRT_3 },
                },
                Offset {
                    x: -2,
                    y: 2,
                    coordinates: Coordinates { x: -1.5 - SQRT_3, y: 1.0 + 0.5*SQRT_3 },
                },
                Offset {
                    x: -2,
                    y: 0,
                    coordinates: Coordinates { x: -1.5 - SQRT_3, y: -1.0 - 0.5*SQRT_3 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5, y: -1.0 - 0.5*SQRT_3 },
                        Coordinates { x: 0.5, y: -1.0 - 0.5*SQRT_3 },
                        Coordinates { x: 0.5 + 0.5*SQRT_3, y: -0.5 - 0.5*SQRT_3 },
                        Coordinates { x: 1.0 + 0.5*SQRT_3, y: -0.5 },
                        Coordinates { x: 1.0 + 0.5*SQRT_3, y: 0.5 },
                        Coordinates { x: 0.5 + 0.5*SQRT_3, y: 0.5 + 0.5*SQRT_3 },
                        Coordinates { x: 0.5, y: 1.0 + 0.5*SQRT_3 },
                        Coordinates { x: -0.5, y: 1.0 + 0.5*SQRT_3 },
                        Coordinates { x: -0.5 - 0.5*SQRT_3, y: 0.5 + 0.5*SQRT_3 },
                        Coordinates { x: -1.0 + -0.5*SQRT_3, y: 0.5 },
                        Coordinates { x: -1.0 + -0.5*SQRT_3, y: -0.5 },
                        Coordinates { x: -0.5 - 0.5*SQRT_3, y: -0.5 - 0.5*SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 0,
                            other_side_index: 6,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 0,
                            other_side_index: 8,
                        },
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 10,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 6,
                            other_polygon: 0,
                            other_side_index: 4,
                        },
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: -0.5, y: 1.0 + 0.5*SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: -0.5*SQRT_3, y: -0.5 },
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: -0.5*SQRT_3, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 7,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 11,
                        },
                        PolygonSide {
                            other_tile: 5,
                            other_polygon: 0,
                            other_side_index: 3,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 1,
                        coordinates: Coordinates { x: 0.5, y: 1.0 + 0.5*SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 0.5*SQRT_3, y: -0.5 },
                        Coordinates { x: 0.5*SQRT_3, y: 0.5 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 5,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 9,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
            ],           
        }
    }
    pub fn triakis() -> Self {
        Self {
            neighbors: vec![
                Offset {
                    x: -3,
                    y: -2,
                    coordinates: Coordinates { x: -3.0, y: -SQRT_3 },
                },
                Offset {
                    x: 3,
                    y: -2,
                    coordinates: Coordinates { x: 3.0, y: -SQRT_3 },
                },
                Offset {
                    x: 3,
                    y: 2,
                    coordinates: Coordinates { x: 3.0, y: SQRT_3 },
                },
                Offset {
                    x: -3,
                    y: 2,
                    coordinates: Coordinates { x: -3.0, y: SQRT_3 },
                },
            ],
            polygons: vec![
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: -3.0, y: SQRT_3 },
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: -1.0, y: SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 1,
                            other_polygon: 5,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 0,
                        y: 1,
                        coordinates: Coordinates { x: -1.0, y: SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: -2.0, y: 0.0 },
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 1.0, y: SQRT_3 }
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 4,
                            other_polygon: 4,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 0.0, y: 2.0*SQRT_3 },
                        Coordinates { x: -1.0, y: SQRT_3 }
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 1,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 0,
                            other_side_index: 1,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 1,
                        y: 1,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 1.0, y: SQRT_3 },
                        Coordinates { x: 0.0, y: 2.0*SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 4,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 5,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 2,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 2,
                        y: 0,
                        coordinates: Coordinates { x: 0.0, y: 0.0 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 3.0, y: SQRT_3 },
                        Coordinates { x: 1.0, y: SQRT_3 },
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 2,
                            other_polygon: 1,
                            other_side_index: 2,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 5,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 0,
                        },
                    ]
                },
                TilePolygon {
                    offset: Offset {
                        x: 2,
                        y: 1,
                        coordinates: Coordinates { x: 1.0, y: SQRT_3 },
                    },
                    corners: vec![
                        Coordinates { x: 0.0, y: 0.0 },
                        Coordinates { x: 2.0, y: 0.0 },
                        Coordinates { x: -1.0, y: SQRT_3 }
                    ],
                    sides: vec![
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 4,
                            other_side_index: 1,
                        },
                        PolygonSide {
                            other_tile: 3,
                            other_polygon: 0,
                            other_side_index: 0,
                        },
                        PolygonSide {
                            other_tile: 0,
                            other_polygon: 3,
                            other_side_index: 1,
                        },
                    ]
                },
            ],      
        }
    }
}