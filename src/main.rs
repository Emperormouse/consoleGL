use crate::constants::{FPS, HEIGHT, WIDTH};
use crate::point::{distance, Point3d};
use crate::triangle::Triangle3d;
use crate::projection::{Camera, Shape3d};
use crate::procedural::generate_shape_height_map;

use std::f32::consts::PI;

use device_query::{DeviceQuery, DeviceState, Keycode};

mod constants;
mod display;
mod file;
mod line;
mod point;
mod terminal;
mod triangle;
mod projection;
mod procedural;
mod triangulate;

fn main() {

    let mut camera = Camera {
        pos: Point3d {
            x: 0.0,
            y: 300.0,
            z: 0.0,
        },
        rot: Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    };


    //REFERENCE POINTS
    let ref_p1 = Point3d { x: 0.0, y: 0.0, z: 0.0, };
    let ref_p2 = Point3d { x: 500.0, y: -100.0, z: 0.0, };
    let ref_p3 = Point3d { x: -500.0, y: 0.0, z: 500.0, };
    let ref_p4 = Point3d { x: 300.0, y: 400.0, z: 0.0, };
    
    let mut terrain = generate_shape_height_map(b'-');

    let delay: u64 = (1000 / FPS) as u64;
    let mut move_per_frame = 100.0 / FPS as f32;
    let radians_per_frame = PI / 1.5 / FPS as f32;
    let device_state = DeviceState::new();
    let mut keys: Vec<Keycode> = device_state.get_keys();
    let mut last_keys: Vec<Keycode> = device_state.get_keys();
    let mut real_fps = 0.0;

    'game_loop: loop {
        let start_frame_time = std::time::SystemTime::now();
        last_keys.clear();
        for key in &keys {
            last_keys.push(*key);
        }
        keys = device_state.get_keys();

        //CONTROLS
        if keys.contains(&Keycode::LShift) && !last_keys.contains(&Keycode::LShift) {
            move_per_frame = 200.0 / FPS as f32;
        }
        if !keys.contains(&Keycode::LShift) && last_keys.contains(&Keycode::LShift) {
            move_per_frame = 100.0 / FPS as f32;
        }
        camera.rot.y = 0.0; camera.rot.z = 0.0;
        for key in &keys {
            match key {
                Keycode::X  => camera.pos.y += move_per_frame,
                Keycode::Z  => camera.pos.y -= move_per_frame,

                // Keycode::Left  => camera.rot.z += radians_per_frame,
                // Keycode::Right  => camera.rot.z -= radians_per_frame,
                // Keycode::Up  => camera.rot.x -= radians_per_frame,
                // Keycode::Down  => camera.rot.x += radians_per_frame,

                Keycode::H | Keycode::Left => camera.rot.y += radians_per_frame,
                Keycode::L | Keycode::Right => camera.rot.y -= radians_per_frame,
                Keycode::K | Keycode::Up => camera.rot.x += radians_per_frame / 2.0,
                Keycode::J | Keycode::Down => camera.rot.x -= radians_per_frame / 2.0,

                Keycode::A  => camera.pos.x -= move_per_frame,
                Keycode::D  => camera.pos.x += move_per_frame,
                Keycode::W  => camera.pos.z += move_per_frame,
                Keycode::S  => camera.pos.z -= move_per_frame,

                Keycode::Space => camera.pos.z += move_per_frame,

                Keycode::Q => break 'game_loop,
                _ => (),
            }
        }

        let mut screen = display::Screen {
            grid: [[b' '; WIDTH]; HEIGHT],
            z_buf: [[None; WIDTH]; HEIGHT],
        };

        let mut render_shapes = |shape_vec: &mut Vec<Shape3d>| {
            for s in shape_vec {
                match s {
                    Shape3d::Triangle(t) => {
                        t.rotate_y_mut(&camera.pos, camera.rot.y);
                        t.rotate_z_mut(&camera.pos, camera.rot.z);
                        let t_2d = t.project(&Camera {
                            pos: camera.pos,
                            rot: Point3d {x:0.0, y:camera.rot.x, z:0.0}
                        });
                        // println!("({}, {}, {})", t_2d.points[0], t_2d.points[1], t_2d.points[2]);
                        t_2d.add_to_grid(&mut screen);
                        t_2d.add_border_to_grid(&mut screen);
                    },
                    Shape3d::Line(l) => {
                        l.rotate_y_mut(&camera.pos, camera.rot.y);
                        l.rotate_z_mut(&camera.pos, camera.rot.z);
                        let l_2d = l.project(&Camera {
                            pos: camera.pos,
                            rot: Point3d {x:0.0, y:camera.rot.x, z:0.0}
                        });
                        l_2d.add_to_grid(&mut screen);
                    },
                }
            }
        };
        render_shapes(&mut terrain);
        display::print_grid(&screen);

        for ref_p in &mut [ref_p1, ref_p2, ref_p3, ref_p4] {
            ref_p.rotate_y_mut(&camera.pos, camera.rot.y);
            ref_p.rotate_z_mut(&camera.pos, camera.rot.z);
        }
        //DISTANCE CALCULATIONS
        let d1 = distance(&camera.pos, &ref_p1);
        let d2 = distance(&camera.pos, &ref_p2);
        let d3 = distance(&camera.pos, &ref_p3);
        let d4 = distance(&camera.pos, &ref_p4);
        let real_location = triangulate::triangulate(ref_p1, ref_p2, ref_p3, ref_p4, d1, d2, d3, d4);

        println!("REAL: {}", real_location);
        println!("{}", camera.pos);
        println!("{}", camera.rotation_degrees());
        println!("FPS: {}", real_fps);

        let time_processing = std::time::SystemTime::now().duration_since(start_frame_time).unwrap().as_millis();
        println!("PROCESS: {}", time_processing);
        if (time_processing as u64) < delay {
            std::thread::sleep(std::time::Duration::from_millis(delay - time_processing as u64));
        }
        let total_time = std::time::SystemTime::now().duration_since(start_frame_time).unwrap().as_millis();
        real_fps = 1000.0 / total_time as f64;
    }
}
