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

    let mut shapes: Vec<Shape3d> = vec![];
    for l in lines {
        match &l[0..3] {
            "TRI" => {
                let t = string_to_triangle(l);

                if let Some(b) = t.border {
                    shapes.push(Shape3d::Line(Line3d {p1:t.points[0], p2:t.points[1], character: b}));
                    shapes.push(Shape3d::Line(Line3d {p1:t.points[0], p2:t.points[2], character: b}));
                    shapes.push(Shape3d::Line(Line3d {p1:t.points[1], p2:t.points[2], character: b}));
                }

                let mut vec: Vec<Shape3d> = vec![];
                bisect_triangle(Triangle3d {
                    points: t.points,
                    fill: t.fill,
                    border: None,
                }, &mut vec);
                for s in vec {
                    shapes.push(s);
                }
            },
            "LIN" => {
                let l = string_to_line(l);

                let mut vec: Vec<Shape3d> = vec![];
                bisect_line(l, &mut vec);
                for s in vec {
                    shapes.push(s);
                }
            },
            _ => panic!("Error in format of file {}", path),
        }
    }
    // let mut tmp: Vec<Shape3d> = vec![];
    // for s in &mut shapes {
    //     match s {
    //         Shape3d::Triangle(t) => {
    //             tmp.push(Shape3d::Line(Line3d {p1: t.points[0], p2: t.points[1], character: b'X'}));
    //             tmp.push(Shape3d::Line(Line3d {p1: t.points[0], p2: t.points[2], character: b'X'}));
    //             tmp.push(Shape3d::Line(Line3d {p1: t.points[1], p2: t.points[2], character: b'X'}));
    //         },
    //         _ => (),
        // }
    // }
    // for s in tmp {
        // shapes.push(s);
    // }
    return shapes;
}

pub fn string_to_triangle(text: &str) -> Triangle3d {

    let split = text.split("}").collect::<Vec<&str>>();
    let border: Option<u8> = match split[0].chars().nth_back(0).unwrap() as u8 {
        b'N' => None,
        letter => Some(letter),
    };
    let fill: u8 = split[1].chars().nth_back(0).unwrap() as u8;

    let points_vec: Vec<Point3d> = split[2]
        .split(")")
        .filter(|l| l.len() > 0)
        .map(|s| &s.trim()[1..])
        .map(|p_str| {
            let p_split: Vec<f32> = p_str
                .split(",")
                .map(|s| s.trim().parse::<f32>().unwrap())
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
            let p_split: Vec<f32> = p_str
                .split(",")
                .map(|s| s.trim().parse::<f32>().unwrap())
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

pub fn bisect_triangle(t: Triangle3d, vec: &mut Vec<Shape3d>) {
    let s1 = Line3d{p1: t.points[0], p2: t.points[1], character:0};
    let s2 = Line3d{p1: t.points[0], p2: t.points[2], character:0};
    let s3 = Line3d{p1: t.points[1], p2: t.points[2], character:0};
    let (l1, l2, l3) = (s1.length(), s2.length(), s3.length());
    if l1 <= 400.0 && l2 <= 400.0 && l3 <= 400.0 {
        vec.push(Shape3d::Triangle(t));
        return;
    }
    println!("l1: {}, l2: {}, l3: {}", l1, l2, l3);

    if l1 >= l2 && l1 >= l3 {
        let m = s1.center();
        bisect_triangle(Triangle3d {
            points: [t.points[0], m, t.points[2]],
            fill: t.fill,
            border: t.border,
        }, vec);
        bisect_triangle(Triangle3d {
            points: [m, t.points[1], t.points[2]],
            fill: t.fill,
            border: t.border,
        }, vec);
    } else if l2 >= l3 {
        let m = s2.center();
        bisect_triangle(Triangle3d {
            points: [t.points[0], m, t.points[1]],
            fill: t.fill,
            border: t.border,
        }, vec);
        bisect_triangle(Triangle3d {
            points: [m, t.points[1], t.points[2]],
            fill: t.fill,
            border: t.border,
        }, vec);
    } else {
        let m = s3.center();
        bisect_triangle(Triangle3d {
            points: [t.points[0], m, t.points[1]],
            fill: t.fill,
            border: t.border,
        }, vec);
        bisect_triangle(Triangle3d {
            points: [m, t.points[0], t.points[2]],
            fill: t.fill,
            border: t.border,
        }, vec);
    }
}

pub fn bisect_line(l: Line3d, vec: &mut Vec<Shape3d>) {
    let m = l.center();
    if l.length() <= 300.0 {
        vec.push(Shape3d::Line(l));
        return;
    }
    let l1 = Line3d{p1: l.p1, p2: m, character: l.character};
    let l2 = Line3d{p1: m, p2: l.p2, character: l.character};
    bisect_line(l1, vec);
    bisect_line(l2, vec);
}

