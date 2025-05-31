use rand;
use rand::seq::SliceRandom;
use crate::projection::Shape3d;
use crate::triangle::Triangle3d;
use crate::point::Point3d;

const WIDTH: i64 = 30;
const HEIGHT: i64 = 30;
const WIDTH_U: usize = WIDTH as usize;
const HEIGHT_U: usize = HEIGHT as usize;

pub fn generate_shape_height_map(fill: u8) -> Vec<Shape3d> {
    let mut map_opt = [[None; WIDTH_U]; HEIGHT_U];
    map_opt[HEIGHT as usize/2][WIDTH as usize/2] = Some(0);
    generate_map(HEIGHT/2, WIDTH/2, &mut map_opt);

    let mut map: [[i32; WIDTH_U];HEIGHT_U] = [[0;WIDTH_U];HEIGHT_U];
    for r in 0..map_opt.len() {
        for c in 0..map_opt[0].len() {
            map[r][c] = map_opt[r][c].unwrap();
        }
    }
    return shapes_from_map(fill, &mut map);
}

pub fn generate_map(y: i64, x: i64, map: &mut [[Option<i32>; WIDTH_U]; HEIGHT_U]) {
    print_map(map);
    let val = map[y as usize][x as usize].unwrap();
    let mut adjacents: Vec<(i64, i64)> = vec![(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)];
    adjacents.shuffle(&mut rand::rng());
    for (ny, nx) in adjacents {
        if nx >= 0 && nx < WIDTH && ny >= 0 && ny < HEIGHT {
            if map[ny as usize][nx as usize] == None {
                map[ny as usize][nx as usize] = Some(val + (rand::random::<i8>()/4) as i32);
                generate_map(ny, nx, map);
            }
        }
    }
}

pub fn shapes_from_map(fill: u8, map: &mut [[i32; WIDTH_U]; HEIGHT_U]) -> Vec<Shape3d> {
    let mut shapes: Vec<Shape3d> = vec![];
    for r in 0..(map.len() - 1) {
        for c in 0..(map[0].len() - 1) {
            let t = Triangle3d {
                points: [
                    Point3d {x: (c*150) as f32, y: map[r][c] as f32, z: (r*150) as f32},
                    Point3d {x: ((c+1)*150) as f32, y: map[r][c+1] as f32, z: (r*150) as f32},
                    Point3d {x: (c*150) as f32, y: map[r+1][c] as f32, z: ((r+1)*150) as f32},
                ],
                fill,
                border: Some(b' '),
            };
            let t2 = Triangle3d {
                points: [
                    Point3d {x: ((c+1)*150) as f32, y: map[r+1][c+1] as f32, z: ((r+1)*150) as f32},
                    Point3d {x: ((c+1)*150) as f32, y: map[r][c+1] as f32, z: (r*150) as f32},
                    Point3d {x: (c*150) as f32, y: map[r+1][c] as f32, z: ((r+1)*150) as f32},
                ],
                fill,
                border: Some(b' '),
            };
            shapes.push(Shape3d::Triangle(t));
            shapes.push(Shape3d::Triangle(t2));
        }
    }
    return shapes;
}

pub fn print_map(map: &mut [[Option<i32>; WIDTH_U]; HEIGHT_U]) {
    for row in map {
        for cell in row {
            match cell {
                Some(h) => print!("X"),
                None => print!(" "),
            }
        }
        println!(" ");
    }
    println!("\n\n\n");
}


