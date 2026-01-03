use std::f32::consts::PI;

use macroquad::prelude::*;

const STAR_MAX_DEPTH: f32 = 1000.;
const SPEED: f32 = 0.8;

#[derive(Clone, Debug)]
struct Star {
    rad: f32,
    dist: f32,
}

impl Star {
    pub fn new() -> Self {
        Self {
            rad: rand::gen_range(-PI, PI),
            dist: rand::gen_range(0., STAR_MAX_DEPTH),
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.dist = (self.dist + dt + SPEED) % 1000.;
    }
}

#[macroquad::main("Starfield")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let center_x = screen_width() / 2.0;
    let center_y = screen_height() / 2.0;

    let mut stars: Vec<Star> = (0..1000).map(|_| Star::new()).collect();

    for star in stars.iter_mut() {
        star.update(0.);
    }

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        for star in stars.iter_mut() {
            let rdist = f32::powi(star.dist / STAR_MAX_DEPTH, 15);
            draw_circle(
                center_x + f32::cos(star.rad) * rdist * width,
                center_y + f32::sin(star.rad) * rdist * height,
                5.0 * rdist,
                Color {
                    r: 1.0 - rdist * 0.4,
                    g: 1.0 - rdist * 0.4,
                    b: 1.0,
                    a: rdist + 0.3,
                },
            );
            star.update(dt);
        }

        next_frame().await
    }
}
