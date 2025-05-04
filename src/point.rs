use crate::constants::{HEIGHT, WIDTH};
use std::f64::consts::PI;
use crate::projection::Camera;

#[derive(Copy, Clone)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Point3d {
    pub fn project(&self, camera: &Camera) -> Option<Point2d> {
        let relative = Point3d {
            x: self.x - camera.pos.x,
            y: self.y - camera.pos.y,
            z: self.z - camera.pos.z,
        };

        if relative.z <= 110.0 {
            return None;
        }

        let scale = 100.0 / (relative.z as f64 - 100.0);

        return Some(Point2d {
            x: (scale * relative.x) as i32 + (WIDTH as i32) / 2,
            y: (scale * relative.y) as i32 + (HEIGHT as i32) / 2,
        });
    }
    pub fn rotate_x(&self, center: &Point3d, rad: f64) -> Point3d {
        let y: f64 = (self.y - center.y) as f64;
        let z: f64 = (self.z - center.z) as f64;

        let mut y_rot: f64 = -z * rad.sin() + y * rad.cos();
        let mut z_rot: f64 = z * rad.cos() + y * rad.sin();

        y_rot += center.y;
        z_rot += center.z;

        return Point3d {
            x: self.x,
            y: y_rot,
            z: z_rot,
        };
    }
    pub fn rotate_y(&self, center: &Point3d, rad: f64) -> Point3d {
        let x: f64 = (self.x - center.x) as f64;
        let z: f64 = (self.z - center.z) as f64;

        let mut x_rot: f64 = x * rad.cos() + z * rad.sin();
        let mut z_rot: f64 = -x * rad.sin() + z * rad.cos();

        x_rot += center.x;
        z_rot += center.z;

        return Point3d {
            x: x_rot,
            y: self.y,
            z: z_rot,
        };
    }
    pub fn rotate_y_degrees(&self, center: &Point3d, degrees: f64) -> Point3d {
        return self.rotate_y(center, degrees.to_radians());
    }
    pub fn rotate_y_mut(&mut self, center: &Point3d, rad: f64) {
        let x: f64 = (self.x - center.x) as f64;
        let z: f64 = (self.z - center.z) as f64;

        self.x = x * rad.cos() + z * rad.sin() + center.x;
        self.z = -x * rad.sin() + z * rad.cos() + center.z;
    }
}
impl Camera {
    pub fn rotation_degrees(&self) -> Point3d {
        return Point3d {
            x: self.rot.x * 180.0 / PI,
            y: self.rot.y * 180.0 / PI,
            z: self.rot.z * 180.0 / PI,
        };
    }
}

impl Point2d {
    pub fn is_in_screen(&self) -> bool {
        return self.x >= 0 && self.x < WIDTH as i32 && self.y >= 0 && self.y < HEIGHT as i32;
    }
}

pub fn distance(p1: &Point3d, p2: &Point3d) -> f64 {
    let mut distance = (p2.x - p1.x) * (p2.x - p1.x);
    distance += (p2.y - p1.y) * (p2.y - p1.y);
    distance += (p2.z - p1.z) * (p2.z - p1.z);
    return distance.sqrt();
}

impl std::fmt::Display for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::fmt::Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "POS: ({}, {}, {}) ROT: ({}, {}, {}) ",
            self.pos.x, self.pos.y, self.pos.z, self.rot.x, self.rot.y, self.rot.z
        )
    }
}
