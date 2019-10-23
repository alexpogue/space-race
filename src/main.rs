extern crate sdl2;

use std::process;
use std::collections::HashSet;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

mod ratelimiter;
mod ship;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx
        .window("rust_to_js", 640, 480)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut renderer = match window
        .renderer()
        .build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

    let mut limiter = ratelimiter::RateLimiter::new(60);

    let mut rect = Rect::new(10, 10, 10, 10);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut ship = ship::Ship::new(50, 50);

    let mut main_loop = || {
        let pressed_keys:HashSet<Keycode> = events.keyboard_state().pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        if pressed_keys.contains(&Keycode::Left) {
            ship.accelerate_x(-1);
        }
        if pressed_keys.contains(&Keycode::Right) {
            ship.accelerate_x(1);
        }
        if pressed_keys.contains(&Keycode::Up) {
            ship.accelerate_y(-1);
        }
        if pressed_keys.contains(&Keycode::Down) {
            ship.accelerate_y(1);
        }

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                _ => {}
            }
        }

        rect.x = ship.x();
        rect.y = ship.y();
        ship.update();

        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();
        let _ = renderer.set_draw_color(white);
        let _ = renderer.fill_rect(rect);
        let _ = renderer.present();
        limiter.limit()
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
