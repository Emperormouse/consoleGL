use crate::constants::{HEIGHT, WIDTH};
const BORDER: [u8; WIDTH + 2] = [b'='; WIDTH + 2];

pub trait Shape2d {
    fn add_to_grid(&self, grid: &mut [[u8; WIDTH]; HEIGHT]);
}

pub struct Pixel {
    pub character: u8,
    pub z: Option<i32>,
}

pub struct Screen {
    pub grid: [[u8; WIDTH]; HEIGHT],
    pub z_buf: [[Option<f32>; WIDTH]; HEIGHT],
}

pub fn print_grid(screen: &Screen) {
    unsafe {
        println!("\n\n\n\n\n\n\n{}", std::str::from_utf8_unchecked(&BORDER));
        for line in screen.grid.iter().rev() {
            println!("{}", std::str::from_utf8_unchecked(line));
        }
        println!("{}", std::str::from_utf8_unchecked(&BORDER));
    }
}
