use crate::constants::{HEIGHT, WIDTH};
use crate::line::Line2dZ;
use crate::point::{Point2d, Point2dZ, Point3d};
use crate::projection::Camera;
use crate::display::{Pixel, Screen};
use crate::display;
use std::cmp;

pub struct Triangle3d {
    pub points: [Point3d; 3],
    pub fill: u8,
    pub border: Option<u8>,
}
pub struct Triangle2d {
    pub points: [Point2dZ; 3],
    pub fill: u8,
    pub border: Option<u8>,
}

//Actually double or triple the area, but it doesn't matter for what I'm using it for
pub fn area_from_points(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> i64 {
    let mut area: i64 = 0;
    area += (p1.x * (p2.y - p3.y)) as i64;
    area += (p2.x * (p3.y - p1.y)) as i64;
    area += (p3.x * (p1.y - p2.y)) as i64;
    return area.abs();
}

impl Triangle2d {
    pub fn add_to_grid(&self, screen: &mut Screen) {
        let y_max = cmp::min( HEIGHT as i32 - 1, cmp::max(cmp::max(self.points[0].y as i32, self.points[1].y as i32), self.points[2].y as i32), );
        let y_min = cmp::max(0, cmp::min(cmp::min(self.points[0].y as i32, self.points[1].y as i32), self.points[2].y as i32));
        let x_max = cmp::min(WIDTH as i32 - 1, cmp::max(cmp::max(self.points[0].x as i32, self.points[1].x as i32), self.points[2].x as i32));
        let x_min = cmp::max(0, cmp::min(cmp::min(self.points[0].x as i32, self.points[1].x as i32), self.points[2].x as i32));

        let triangle_area = self.area();
        let (p1, p2, p3) = (
            self.points[0].drop_z(),
            self.points[1].drop_z(),
            self.points[2].drop_z()
        );
        let (z1, z2, z3) = (self.points[0].z as i64, self.points[1].z as i64, self.points[2].z as i64);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let p = Point2d {x, y};
                let sub1 = area_from_points(&p, &p2, &p3);
                let sub2 = area_from_points(&p, &p1, &p3);
                let sub3 = area_from_points(&p, &p1, &p2);

                if sub1 + sub2 + sub3 == triangle_area as i64 {
                    let z = ((sub1 * z1) + (sub2 * z2) + (sub3 * z3)) as f32 / triangle_area as f32 + 5.0;
                    match screen.z_buf[y as usize][x as usize] {
                        None => {
                            screen.grid[y as usize][x as usize] = self.fill;
                            screen.z_buf[y as usize][x as usize] = Some(z);
                        },
                        Some(pixel_z) => {
                            if z < pixel_z {
                                screen.grid[y as usize][x as usize] = self.fill;
                                screen.z_buf[y as usize][x as usize] = Some(z);
                            }
                        },
                    }
                }
            }
        }
    }  

    /*pub fn render_scanlines(&self, screen: &mut Screen) {
        let points = &self.points;
        let (left_point, other_points) = if points[0].x <= points[1].x && points[0].x <= points[2].x {
            (&points[0], [&points[1], &points[2]])
        } else if &points[1].x <= &points[2].x {
            (&points[1], [&points[0], &points[2]])
        } else {
            (&points[2], [&points[0], &points[1]])
        };
        let (high_point, low_point) = if other_points[0].y >= other_points[1].y {
            (&other_points[0], &other_points[1])
        } else {
            (&other_points[1], &other_points[0])
        };
        let (right_point, mid_point) = if other_points[0].x >= other_points[1].x { 
            (&other_points[0], &other_points[1])
        } else {
            (&other_points[1], &other_points[0])
        };
        let mut slope_min = (low_point.y - left_point.y) as f32 / (low_point.x - left_point.x) as f32 ;
        let mut slope_max = (high_point.y - left_point.y) as f32  / (high_point.x - left_point.x) as f32 ;
        let (mut min, mut max) = (left_point.y as f32 , left_point.y as f32 );

        //variables needed for z-buffering
        let area = self.area();
        let (p2d_1, p2d_2, p2d_3) = (points[0].drop_z(), points[1].drop_z(), points[2].drop_z());
        let (z1, z2, z3) = (points[0].z, points[1].z, points[2].z);

        // println!("slope_min: {}", slope_min); println!("slope_max: {}", slope_max); println!("left: {}", left_point); println!("right: {}", right_point); println!("mid: {}", mid_point); println!("low: {}", low_point); println!("high: {}", high_point);
        for x in left_point.x as i32..mid_point.x as i32 {
            for y in min.ceil() as i32..max as i32 {
                if x>=0 && y>= 0 && x<WIDTH as i32 && y<HEIGHT as i32 {
                    let p = Point2d {x, y};
                    let sub1 = area_from_points(&p, &p2d_2, &p2d_3);
                    let sub2 = area_from_points(&p, &p2d_1, &p2d_3);
                    let sub3 = area_from_points(&p, &p2d_1, &p2d_2);
                    let z = ((sub1 * z1) + (sub2 * z2) + (sub3 * z3)) as f32 / area as f32;
                    match screen.z_buf[y as usize][x as usize] {
                        None => {
                            screen.grid[y as usize][x as usize] = self.fill;
                            screen.z_buf[y as usize][x as usize] = Some(z);
                        },
                        Some(pixel_z) => {
                            if z <= pixel_z {
                                screen.grid[y as usize][x as usize] = self.fill;
                                screen.z_buf[y as usize][x as usize] = Some(z);
                            }
                        },
                    }
                }
            }
            min += slope_min;
            max += slope_max;
        }
        // print!("{}: ", left_point.z);
        // println!("min: {}, max: {}", min, max);
        let other_slope = (mid_point.y - right_point.y) as f32 / (mid_point.x - right_point.x) as f32;
        if mid_point.x == low_point.x && mid_point.y == low_point.y {
            slope_min = other_slope;
        } else {
            slope_max = other_slope;
        }
        for x in mid_point.x as i32..=right_point.x as i32 {
            for y in min.ceil() as i32..max as i32 {
                if x>=0 && y>= 0 && x<WIDTH as i32 && y<HEIGHT as i32 {
                    let p = Point2d {x, y};
                    let sub1 = area_from_points(&p, &p2d_2, &p2d_3);
                    let sub2 = area_from_points(&p, &p2d_1, &p2d_3);
                    let sub3 = area_from_points(&p, &p2d_1, &p2d_2);
                    let z = ((sub1 * z1) + (sub2 * z2) + (sub3 * z3)) as f32 / area as f32;
                    match screen.z_buf[y as usize][x as usize] {
                        None => {
                            screen.grid[y as usize][x as usize] = self.fill;
                            screen.z_buf[y as usize][x as usize] = Some(z);
                        },
                        Some(pixel_z) => {
                            if z <= pixel_z {
                                screen.grid[y as usize][x as usize] = self.fill;
                                screen.z_buf[y as usize][x as usize] = Some(z);
                            }
                        },
                    }
                }
            }
            min += slope_min;
            max += slope_max;
            if min == f32::NEG_INFINITY || min.is_nan() {
                min = low_point.y as f32;
            }
            if  max == f32::INFINITY || max.is_nan() {
                max = high_point.y as f32;
            }
        }
        // println!("min: {}, max: {}\n", min, max);
        // println!("MARK3");
        // self.add_border_to_grid(screen);
    }*/
    
    
    pub fn add_border_to_grid(&self, screen: &mut Screen) {
        if let Some(b) = self.border {
            Line2dZ { p1: self.points[0], p2: self.points[1], character: b}
                .add_to_grid(screen);
            Line2dZ { p1: self.points[0], p2: self.points[2], character: b}
                .add_to_grid(screen);
            Line2dZ { p1: self.points[1], p2: self.points[2], character: b}
                .add_to_grid(screen);
        }
    }

    pub fn area(&self) -> i32 {
        let mut area: i32 = 0;
        area += self.points[0].x * (self.points[1].y - self.points[2].y);
        area += self.points[1].x * (self.points[2].y - self.points[0].y);
        area += self.points[2].x * (self.points[0].y - self.points[1].y);
        return area.abs();
    }
    // pub fn is_inside(&self, p: Point2d) -> bool {
    //     let t1 = Triangle2d { points: [Point2d {x:p.x, y:p.y}, Point2d {x:self.points[0].x, y:self.points[0].y}, Point2d{x:self.points[1].x, y:self.points[1].y}], fill: b'X', border: None };
    //     let t2 = Triangle2d { points: [Point2d {x:p.x, y:p.y}, Point2d {x:self.points[0].x, y:self.points[0].y}, Point2d{x:self.points[2].x, y:self.points[2].y}], fill: b'X', border: None };
    //     let t3 = Triangle2d { points: [Point2d {x:p.x, y:p.y}, Point2d {x:self.points[1].x, y:self.points[1].y}, Point2d{x:self.points[2].x, y:self.points[2].y}], fill: b'X', border: None };
    //     return self.area() == (t1.area() + t2.area() + t3.area());
    // }
    pub fn scale(&mut self, magnitude: usize) {
        for p in &mut self.points {
            p.x *= magnitude as i32;
            p.y *= magnitude as i32;
        }
    }
}


impl Triangle3d {
    pub fn project(&self, camera: &Camera) -> Triangle2d {
       let p1 = self.points[0]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .rotate_z(&camera.pos, camera.rot.z)
           .project(camera);
       let p2 = self.points[1]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .rotate_z(&camera.pos, camera.rot.z)
           .project(camera);
       let p3 = self.points[2]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .rotate_z(&camera.pos, camera.rot.z)
           .project(camera);
       return Triangle2d {
           points: match (p1, p2, p3) {
               (Some(p1), Some(p2), Some(p3)) => [p1, p2, p3],
               _ => [Point2dZ { x: -1, y: -1, z: 0}, Point2dZ { x: -1, y: -1, z: 0}, Point2dZ { x: -1, y: -1, z: 0}],
           },
           fill: self.fill,
           border: self.border,
       };
   }   

   pub fn rotate_y(&self, center: &Point3d, rads: f32) -> Triangle3d {
       Triangle3d {
            points: [
               self.points[0].rotate_y(center, rads),
               self.points[1].rotate_y(center, rads),
               self.points[2].rotate_y(center, rads),
            ],
            border: self.border,
            fill: self.fill,
       }
   }
   pub fn rotate_x(&self, center: &Point3d, rads: f32) -> Triangle3d {
       Triangle3d {
            points: [
               self.points[0].rotate_x(center, rads),
               self.points[1].rotate_x(center, rads),
               self.points[2].rotate_x(center, rads),
            ],
            border: self.border,
            fill: self.fill,
       }
   }
   pub fn rotate_z(&self, center: &Point3d, rads: f32) -> Triangle3d {
       Triangle3d {
            points: [
               self.points[0].rotate_z(center, rads),
               self.points[1].rotate_z(center, rads),
               self.points[2].rotate_z(center, rads),
            ],
            border: self.border,
            fill: self.fill,
       }
   }
   pub fn rotate_y_mut(&mut self, center: &Point3d, rads: f32) {
       for p in &mut self.points {
            p.rotate_y_mut(center, rads);
       }
   }
   pub fn rotate_x_mut(&mut self, center: &Point3d, rads: f32) {
       for p in &mut self.points {
            p.rotate_x_mut(center, rads);
       }
   }
   pub fn rotate_z_mut(&mut self, center: &Point3d, rads: f32) {
       for p in &mut self.points {
            p.rotate_z_mut(center, rads);
       }
   }
   pub fn center(&self) -> Point3d {
       Point3d {
           x: (self.points[0].x + self.points[1].x + self.points[2].x) / 3.0,
           y: (self.points[0].y + self.points[1].y + self.points[2].y) / 3.0,
           z: (self.points[0].z + self.points[1].z + self.points[2].z) / 3.0,
       }
   }
}

impl Clone for Triangle3d {
    fn clone(&self) -> Self {
        Triangle3d {
            points: [self.points[0].clone(), self.points[1].clone(), self.points[2].clone()],
            fill: self.fill,
            border: self.border,
        }
    }
}

