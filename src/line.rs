use crate::constants::{HEIGHT, WIDTH};
use crate::point::{self, Point2d, Point2dZ, Point3d};
use crate::projection::Camera;
use crate::display::Screen;

pub struct Line3d {
    pub p1: Point3d,
    pub p2: Point3d,
    pub character: u8,
}
pub struct Line2dZ {
    pub p1: Point2dZ,
    pub p2: Point2dZ,
    pub character: u8,
}
pub struct Line2d {
    pub p1: Point2d,
    pub p2: Point2d,
    pub character: u8,
}

impl Line3d {
    pub fn project(&self, camera: &Camera) -> Line2dZ {
        let p1 = self.p1
            .rotate_y(&camera.pos, camera.rot.x)
            .rotate_x(&camera.pos, camera.rot.y)
            .project(camera);
        let p2 = self.p2
            .rotate_y(&camera.pos, camera.rot.x)
            .rotate_x(&camera.pos, camera.rot.y)
            .project(camera);
        let (p1_unwrapped, p2_unwrapped) = match (p1, p2) {
            (Some(p1), Some(p2)) => (p1, p2),
            _ => (Point2dZ { x: -1, y: -1, z: 0 }, Point2dZ {x: -1, y: -1, z: 0 }),
        };
        return Line2dZ {
            p1: p1_unwrapped,
            p2: p2_unwrapped,
            character: self.character,
        };
    }
    pub fn center(&self) -> Point3d {
        Point3d {
            x: (self.p1.x + self.p2.x) / 2.0,
            y: (self.p1.y + self.p2.y) / 2.0,
            z: (self.p1.z + self.p2.z) / 2.0,
        }
    }
    pub fn length(&self) -> f32 {
        point::distance(&self.p1, &self.p2)
    }
   pub fn rotate_y_mut(&mut self, center: &Point3d, rads: f32) {
       self.p1.rotate_y_mut(center, rads);
       self.p2.rotate_y_mut(center, rads);
   }
   pub fn rotate_x_mut(&mut self, center: &Point3d, rads: f32) {
       self.p1.rotate_x_mut(center, rads);
       self.p2.rotate_x_mut(center, rads);
   }
   pub fn rotate_z_mut(&mut self, center: &Point3d, rads: f32) {
       self.p1.rotate_z_mut(center, rads);
       self.p2.rotate_z_mut(center, rads);
   }

    pub fn add_to_grid(&self, screen: &mut Screen) {
        let (p1, p2) = (Point2d {x:self.p1.x as i32, y:self.p1.y as i32}, Point2d {x:self.p2.x as i32, y:self.p2.y as i32});
        // if !p1.is_in_screen() && !p2.is_in_screen() {
            // if self.is_in_screen() {
                // return;
            // }
        // }
        let mut current_p = Point2d { x: p1.x, y: p1.y };
        let diff_x = p2.x - current_p.x;
        let diff_y = p2.y - current_p.y;

        let ratio: f32 = if diff_x == 0 {
            9000.0
        } else {
            diff_y as f32 / diff_x as f32
        };

        let mut count_x: u16 = 0;
        let mut count_y: u16 = 0;
        let x_increment = if diff_x > 0 { 1 } else { -1 };
        let y_increment = if diff_y > 0 { 1 } else { -1 };
        while current_p.x != p2.x || current_p.y != p2.y {
            if current_p.x >= 0
                && current_p.x < WIDTH as i32
                && current_p.y >= 0
                && current_p.y < HEIGHT as i32
            {
                screen.grid[current_p.y as usize][current_p.x as usize] = self.character;
            }
            if (count_x as f32 * ratio.abs()) < count_y as f32 && current_p.x != p2.x {
                current_p.x += x_increment;
                count_x += 1;
            } else {
                if current_p.y != p2.y {
                    current_p.y += y_increment;
                }
                count_y += 1;
            }
        }
    }

}

fn is_ccw(a: &Point2d, b: &Point2d, c: &Point2d) -> bool {
    return (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x);
}

fn lines_intersect(a: &Point2d, b: &Point2d, c: &Point2d, d: &Point2d) -> bool {
    return is_ccw(a, c, d) != is_ccw(b, c, d) && is_ccw(a, b, c) != is_ccw(a, b, d);
}

const CORNER1: Point2d = Point2d { x: 0, y: 0 };
const CORNER2: Point2d = Point2d {
    x: 0,
    y: HEIGHT as i32,
};
const CORNER3: Point2d = Point2d {
    x: WIDTH as i32,
    y: 0,
};
const CORNER4: Point2d = Point2d {
    x: WIDTH as i32,
    y: HEIGHT as i32,
};

impl Line2d {
    pub fn is_in_screen(&self) -> bool {
        return lines_intersect(&self.p1, &self.p2, &CORNER1, &CORNER2)
            || lines_intersect(&self.p1, &self.p2, &CORNER1, &CORNER3)
            || lines_intersect(&self.p1, &self.p2, &CORNER2, &CORNER4)
            || lines_intersect(&self.p1, &self.p2, &CORNER3, &CORNER4);
    }

}

impl Line2dZ {
    pub fn add_to_grid(&self, screen: &mut Screen) {
        let (p1, p2) = (&self.p1.drop_z(), &self.p2.drop_z());
        if !p1.is_in_screen() && !p2.is_in_screen() {
            if (Line2d {p1: *p1, p2: *p2, character: self.character}).is_in_screen() {
                return;
            }
        }
        let mut current_p = Point2d { x: p1.x, y: p1.y };
        let diff_x = p2.x - current_p.x;
        let diff_y = p2.y - current_p.y;

        let ratio: f32 = if diff_x == 0 {
            9000.0
        } else {
            diff_y as f32 / diff_x as f32
        };

        let mut count_x: u16 = 0;
        let mut count_y: u16 = 0;
        let x_increment = if diff_x > 0 { 1 } else { -1 };
        let y_increment = if diff_y > 0 { 1 } else { -1 };
        let mut count = 0;
        while current_p.x != p2.x || current_p.y != p2.y {
            count += 1;
            if (count_x as f32 * ratio.abs()) < count_y as f32 && current_p.x != p2.x {
                current_p.x += x_increment;
                count_x += 1;
            } else {
                if current_p.y != p2.y {
                    current_p.y += y_increment;
                }
                count_y += 1;
            }
        }
        let mut current_p = Point2d { x: p1.x, y: p1.y };
        (count_x, count_y) = (0, 0);
        let z_slope = (self.p2.z - self.p1.z) as f32 / count as f32;
        let mut z = self.p1.z as f32;
        while current_p.x != p2.x || current_p.y != p2.y {
            if current_p.x >= 0
                && current_p.x < WIDTH as i32
                && current_p.y >= 0
                && current_p.y < HEIGHT as i32
            {
                let (x, y) = (current_p.x as usize, current_p.y as usize);
                match screen.z_buf[y][x] {
                    None => {
                        screen.grid[y][x] = self.character;
                        screen.z_buf[y][x] = Some(z);
                    },
                    Some(pixel_z) => {
                        if z <= pixel_z + 5.0 {
                            screen.grid[y][x] = self.character;
                            screen.z_buf[y][x] = Some(z);
                        }
                    },
                }
            }
            if (count_x as f32 * ratio.abs()) < count_y as f32 && current_p.x != p2.x {
                current_p.x += x_increment;
                count_x += 1;
            } else {
                if current_p.y != p2.y {
                    current_p.y += y_increment;
                }
                count_y += 1;
            }
            z += z_slope;
        }
    }
}
