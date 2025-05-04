use std::fs::File;
use std::io::Read;

use crate::point::Point3d;
use crate::triangle::Triangle3d;
use crate::line::Line3d;
use crate::projection::Shape3d;

pub fn read_data_file(path: &str) -> Vec<Shape3d> {
    let mut data = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    let lines: Vec<&str> = data.split("\n").filter(|l| l.len() > 0).collect();

    let shapes = lines.iter().map(|l| match &l[0..3] {
        "TRI" => Shape3d::Triangle(string_to_triangle(l)),
        "LIN" => Shape3d::Line(string_to_line(l)),
        _ => panic!("Error in format of file {}", path),
    }).collect();
    return shapes;
}

pub fn string_to_triangle(text: &str) -> Triangle3d {

    let split = text.split("}").collect::<Vec<&str>>();
    let border: u8 = split[0].chars().nth_back(0).unwrap() as u8;
    let fill: u8 = split[1].chars().nth_back(0).unwrap() as u8;

    let points_vec: Vec<Point3d> = split[2]
        .split(")")
        .filter(|l| l.len() > 0)
        .map(|s| &s.trim()[1..])
        .map(|p_str| {
            let p_split: Vec<f64> = p_str
                .split(",")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            return Point3d {
                x: p_split[0],
                y: p_split[1],
                z: p_split[2],
            };
        })
        .collect();

    let points_arr = [
        points_vec[0].clone(),
        points_vec[1].clone(),
        points_vec[2].clone(),
    ];

    return Triangle3d {
        points: points_arr,
        fill,
        border,
    };
}
pub fn string_to_line(text: &str) -> Line3d {

    let split = text.split("}").collect::<Vec<&str>>();
    let character: u8 = split[0].chars().nth_back(0).unwrap() as u8;

    let points_vec: Vec<Point3d> = split[1]
        .split(")")
        .filter(|l| l.len() > 0)
        .map(|s| &s.trim()[1..])
        .map(|p_str| {
            let p_split: Vec<f64> = p_str
                .split(",")
                .map(|s| s.trim().parse::<f64>().unwrap())
                .collect();
            return Point3d {
                x: p_split[0],
                y: p_split[1],
                z: p_split[2],
            };
        })
        .collect();

    return Line3d {
        p1: points_vec[0],
        p2: points_vec[1],
        character,
    };
}
