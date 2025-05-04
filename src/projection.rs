use crate::triangle;
use crate::line;
use crate::point;

pub enum Shape3d {
    Triangle(triangle::Triangle3d),
    Line(line::Line3d)       
}

pub struct Camera {
    pub pos: point::Point3d,
    pub rot: point::Point3d,
}

pub fn sort_by_farthest(vector: &mut Vec<Shape3d>, camera: &Camera) {
    let center = |s: &Shape3d| match s {
        Shape3d::Triangle(t) => t.center(),
        Shape3d::Line(l) => l.center(),
    };
    for i in 1..vector.len() {
        let mut p_idx = i;
        let tmp_distance = point::distance(&center(&vector[i]), &camera.pos);
        while p_idx != 0 && tmp_distance > point::distance(&center(&vector[p_idx - 1]), &camera.pos) {
            vector.swap(p_idx, p_idx - 1);
            p_idx -= 1;
        }
    }
}

