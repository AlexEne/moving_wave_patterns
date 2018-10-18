extern crate minifb;

use minifb::{Key, Window, Scale, WindowOptions};
use std::f64::consts::PI;
use std::time::{Instant, Duration};
use std::time;

fn circle(x: f64, y: f64, x_offset: f64, y_offset: f64, radius: f64) -> bool {
    let x_distance = (x - x_offset) as f64;
    let y_distance = (y - y_offset) as f64;
    let distance = (y_distance * y_distance + x_distance * x_distance).sqrt();//(x as u32 ^ y as u32) >> 3 & 0x03;
    distance <= (radius as f64)
}



fn main() {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const CHAR_SIZE: usize = 8;

    let mut buffer = vec![0u32; WIDTH * HEIGHT].into_boxed_slice();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2
        },
    ).unwrap();

    let mut tile = Vec::new();
    const TILE_SIZE: usize = 32;

    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let mut c = 0;

            let circle_x = (TILE_SIZE as f64) / 2.0 + -1.0;
            let circle_y =  (TILE_SIZE as f64) / 2.0 + 0.0;
            let circle_rad = (TILE_SIZE / 2) as f64;


            let circle2_x = (TILE_SIZE as f64) / 2.0 + 1.0;
            let circle2_y = (TILE_SIZE as f64) / 2.0 + 0.0;
            let circle2_rad = (TILE_SIZE / 4) as f64;

            // if circle(x as _, y as _, circle_x, larrge_circle_y, large_circle_rad) 
            //    ||  circle(x as _, y as _, large_circle_x + (TILE_SIZE as f64), larrge_circle_y, large_circle_rad)
            //     // ||  circle(x as _, y as _, large_circle_x, larrge_circle_y + (TILE_SIZE as f64), large_circle_rad)
            //     // ||  circle(x as _, y as _, large_circle_x + (TILE_SIZE as f64), larrge_circle_y + (TILE_SIZE as f64) , large_circle_rad)
            // {
            //     c = 1;
            // }

            if circle(x as _, y as _, circle_x, circle_y, circle_rad) {
                c = 2;
            }
            if circle(x as _, y as _, circle2_x, circle2_y, circle2_rad) {
                c = 3;
            }
            
            tile.push(c);
        }
    }

    let tile = tile.into_boxed_slice();
    let start_time = time::Instant::now();
    let pallete = [
        0xb5c2b7,
        0x8c93a8,
        0x62466b,
        0x45364b,
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let time = time::Instant::now() - start_time;
        let time = time * 3;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let tile_x = (x  / CHAR_SIZE) as i32;
                let tile_y = (y / CHAR_SIZE) as i32;


                let offset_x = (x as i32) + (((tile_x as f64) * 0.2 + (tile_y as f64) * 0.1 + time.as_secs_f64()).sin() * 16.0) as i32;
                let offset_y = (y as i32) + (((tile_y as f64) * 0.2 + (tile_x as f64) * 0.1 + time.as_secs_f64() * 0.9).sin() * 16.0) as i32;

                let c: u32 = tile[(offset_y as usize % TILE_SIZE) * TILE_SIZE + (offset_x as usize % TILE_SIZE)];
                let output_c = pallete[c as usize];
                buffer[y*WIDTH + x] = output_c;
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}


trait Miliseconds {
    fn as_milis(&self) -> u64;
    fn as_milis_f64(&self) -> f64;
    fn as_secs_f64(&self) -> f64;
}

impl Miliseconds for std::time::Duration {
    fn as_milis(&self) -> u64 {
        (self.as_secs() as f64 * 1000.0 + self.subsec_nanos() as f64 * 1e-6) as u64
    }

    fn as_milis_f64(&self) -> f64 {
        self.as_milis() as f64
    }

    fn as_secs_f64(&self) -> f64 {
        self.as_secs() as f64 + self.subsec_nanos() as f64 * 1e-9
    }
}