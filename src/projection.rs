use crate::triangle;
use crate::line;
use crate::point;

pub enum Shape3d {
    Triangle(triangle::Triangle3d),
    Line(line::Line3d)       
}
impl Shape3d {
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Shape3d::Triangle(t) => {
                t.points[0].x += x; t.points[0].y += y; t.points[0].z += z;
                t.points[1].x += x; t.points[1].y += y; t.points[1].z += z;
                t.points[2].x += x; t.points[2].y += y; t.points[2].z += z;
            },
            Shape3d::Line(l) => {
                l.p1.x += x; l.p1.y += y; l.p1.z += z;
                l.p2.x += x; l.p2.y += y; l.p2.z += z;
            },
        }
    }
}

pub struct Camera {
    pub pos: point::Point3d,
    pub rot: point::Point3d,
}

pub fn sort_by_farthest(vector: &mut Vec<Shape3d>, camera: &Camera) {
    let distance = |s: &Shape3d, p: &point::Point3d| match s {
        Shape3d::Triangle(t) => point::distance(&t.center(), p),
        Shape3d::Line(l) => point::distance(&l.center(), p),
    };
    for i in 1..vector.len() {
        let mut p_idx = i;
        let tmp_distance = distance(&vector[i], &camera.pos);
        while p_idx != 0 && tmp_distance > distance(&vector[p_idx - 1], &camera.pos) {
            vector.swap(p_idx, p_idx - 1);
            p_idx -= 1;
        }
    }
}

