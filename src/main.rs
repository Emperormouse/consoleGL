use crate::constants::{FPS, HEIGHT, WIDTH};
use crate::point::Point3d;
use crate::triangle::Triangle3d;
use crate::projection::{Camera, Shape3d};
use std::f64::consts::PI;
mod constants;
mod display;
mod file;
mod line;
mod point;
mod terminal;
mod triangle;
mod projection;

fn main() {
    let termios = terminal::get_terminal();

    let mut camera = Camera {
        pos: Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rot: Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };

    let center: Point3d = Point3d {
        x: 30.0,
        y: 10.0,
        z: 230.0,
    };
    let mut shapes: Vec<Shape3d> = file::read_data_file("data/house.dat");

    let delay: u64 = (1000 / FPS) as u64;
    let stdin_channel = terminal::spawn_stdin_channel();
    loop {
        let start = std::time::SystemTime::now();
        if let Ok(key) = stdin_channel.try_recv() {
            match key {
                b'x' => camera.pos.y += 3.0,
                b'z' => camera.pos.y -= 3.0,
                b'a' => {
                    camera.pos.z -= 10.0 * camera.rot.x.sin();
                    camera.pos.x -= 10.0 * camera.rot.x.cos();
                }
                b'd' => {
                    camera.pos.z += 10.0 * camera.rot.x.sin();
                    camera.pos.x += 10.0 * camera.rot.x.cos();
                }
                b'w' => {
                    camera.pos.z += 10.0 * camera.rot.x.cos();
                    camera.pos.x -= 10.0 * camera.rot.x.sin();
                }
                b's' => {
                    camera.pos.z -= 10.0 * camera.rot.x.cos();
                    camera.pos.x += 10.0 * camera.rot.x.sin();
                }
                b'h' => camera.rot.x += PI / 36.0,
                b'l' => camera.rot.x -= PI / 36.0,
                b'k' => camera.rot.y += PI / 36.0,
                b'j' => camera.rot.y -= PI / 36.0,
                b'q' => break,
                _ => (),
            }
        }
        stdin_channel.try_recv();
        stdin_channel.try_recv();

        let mut grid: [[u8; WIDTH]; HEIGHT] = [[b' '; WIDTH]; HEIGHT];

        projection::sort_by_farthest(&mut shapes, &camera);
        for s in &mut shapes {
            match s {
                Shape3d::Triangle(t) => {
                    // t.rotate_y_mut(&center, 0.034);
                    let t_2d = t.project(&camera);
                    t_2d.add_to_grid(&mut grid);
                },
                Shape3d::Line(l) => {
                    let l_2d = l.project(&camera);
                    l_2d.add_to_grid(&mut grid);
                },
            }
        }
        display::print_grid(&grid);
        println!("{}", camera.pos);
        println!("{}", camera.rotation_degrees());

        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    terminal::cleanup(&termios);
}
