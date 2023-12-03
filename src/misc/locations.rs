/// Represents a location in three-dimensional space, defined by its x, y, z coordinates, yaw, and pitch.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

/// Represents a vector in three-dimensional space, defined by a starting location and offset values in the x, y, and z directions.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
    pub start: Location,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

/// Represents a bounding box in three-dimensional space, defined by its minimum and maximum locations.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BoundingBox {
    pub min: Location,
    pub max: Location,
}

/// Represents a quaternion, used for representing rotations in three-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Represents a ray in three-dimensional space, defined by its origin, direction, and distance.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ray {
    pub origin: Location,
    pub direction: Vector,
    pub distance: f64,
}

/// Represents a coordinate frame in three-dimensional space, defined by its x, y, and z axes.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoordinateFrame {
    pub x: Vector,
    pub y: Vector,
    pub z: Vector,
}