use crate::constants::{HEIGHT, WIDTH};
use crate::line::Line2d;
use crate::point::{Point2d, Point3d};
use crate::projection::Camera;
use crate::display;
use std::cmp;

pub struct Triangle2d {
    pub points: [Point2d; 3],
    pub fill: u8,
    pub border: u8,
}
impl Triangle2d {
    pub fn new(points: [[i32; 2]; 3], fill: u8, border: u8) -> Triangle2d {
        Triangle2d {
            points: [
                Point2d {
                    x: points[0][0],
                    y: points[0][1],
                },
                Point2d {
                    x: points[1][0],
                    y: points[1][1],
                },
                Point2d {
                    x: points[2][0],
                    y: points[2][1],
                },
            ],
            fill,
            border,
        }
    }
    pub fn new_plain(points: [[i32; 2]; 3]) -> Triangle2d {
        Self::new(points, b'X', b'b')
    }
    pub fn add_to_grid(&self, grid: &mut [[u8; WIDTH]; HEIGHT]) {
       let y_max = cmp::min( HEIGHT as i32 - 1, cmp::max(cmp::max(self.points[0].y, self.points[1].y), self.points[2].y), );
       let y_min = cmp::max(0, cmp::min(cmp::min(self.points[0].y, self.points[1].y), self.points[2].y));
       let x_max = cmp::min(WIDTH as i32 - 1, cmp::max(cmp::max(self.points[0].x, self.points[1].x), self.points[2].x));
       let x_min = cmp::max(0, cmp::min(cmp::min(self.points[0].x, self.points[1].x), self.points[2].x));
       for x in x_min..=x_max {
           for y in y_min..=y_max {
               if self.is_inside(Point2d { x, y }) {
                   grid[y as usize][x as usize] = self.fill;
               }
           }
       }
       self.add_border_to_grid(grid);
    }   

    fn add_border_to_grid(&self, grid: &mut [[u8; WIDTH]; HEIGHT]) {
        Line2d { p1: self.points[0], p2: self.points[1], character: self.border}
            .add_to_grid(grid);
        Line2d { p1: self.points[0], p2: self.points[2], character: self.border}
            .add_to_grid(grid);
        Line2d { p1: self.points[1], p2: self.points[2], character: self.border}
            .add_to_grid(grid);
    }
    // pub fn fill_rec(p: Point2d, grid: &mut [[u8; WIDTH]; HEIGHT]) {
    // grid[p.y][p.x] = b't';
    // }
    pub fn area(&self) -> i32 {
        let mut area: i32 = 0;
        area += self.points[0].x * (self.points[1].y - self.points[2].y);
        area += self.points[1].x * (self.points[2].y - self.points[0].y);
        area += self.points[2].x * (self.points[0].y - self.points[1].y);
        return area.abs();
    }
    pub fn is_inside(&self, p: Point2d) -> bool {
        let t1 = Triangle2d::new_plain([[p.x, p.y], [self.points[0].x, self.points[0].y], [self.points[1].x, self.points[1].y]]);
        let t2 = Triangle2d::new_plain([[p.x, p.y], [self.points[0].x, self.points[0].y], [self.points[2].x, self.points[2].y]]);
        let t3 = Triangle2d::new_plain([[p.x, p.y], [self.points[1].x, self.points[1].y], [self.points[2].x, self.points[2].y]]);
        return self.area() == (t1.area() + t2.area() + t3.area());
    }
    pub fn scale(&mut self, magnitude: usize) {
        for p in &mut self.points {
            p.x *= magnitude as i32;
            p.y *= magnitude as i32;
        }
    }
}


pub struct Triangle3d {
    pub points: [Point3d; 3],
    pub fill: u8,
    pub border: u8,
}
impl Triangle3d {
    pub fn new(points: [[f64; 3]; 3], fill: u8, border: u8) -> Triangle3d {
        Triangle3d {
            points: [
                Point3d {
                    x: points[0][0],
                    y: points[0][1],
                    z: points[0][2],
                },
                Point3d {
                    x: points[1][0],
                    y: points[1][1],
                    z: points[1][2],
                },
                Point3d {
                    x: points[2][0],
                    y: points[2][1],
                    z: points[2][2],
                },
            ],
            fill,
            border,
        }
    }
    pub fn project(&self, camera: &Camera) -> Triangle2d {
       let p1 = self.points[0]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .project(camera);
       let p2 = self.points[1]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .project(camera);
       let p3 = self.points[2]
           .rotate_y(&camera.pos, camera.rot.x)
           .rotate_x(&camera.pos, camera.rot.y)
           .project(camera);
       return Triangle2d {
           points: match (p1, p2, p3) {
               (Some(p1), Some(p2), Some(p3)) => [p1, p2, p3],
               _ => [Point2d { x: -1, y: -1 }, Point2d { x: -1, y: -1 }, Point2d { x: -1, y: -1 }],
           },
           fill: self.fill,
           border: self.border,
       };
   }   
   pub fn rotate_y_mut(&mut self, center: &Point3d, degrees: f64) {
       for p in &mut self.points {
           p.rotate_y_mut(center, degrees)
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

