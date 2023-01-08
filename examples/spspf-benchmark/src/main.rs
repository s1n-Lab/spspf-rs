#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

use spspf_core::{input::{InputManager, Buttons}, Vec2, Vec3};
use spspf_graphics::{Drawable, canvas::Canvas, Primitive, colors::Colors};


psp::module!("SPSPF - Demo", 1, 1);

fn psp_main() {
    let mut running = true;
    let mut canvas = Canvas::new();
    let mut input_manager = InputManager::new();

    let mut rects: Vec<(Primitive::Rect, Vec2<f32>)> = Vec::new();

    while running {
        canvas.start_frame();
        input_manager.update();

        canvas.clear(Colors::BLACK.as_color());

        for rect_id in 0..rects.len() {
            let mut pos = rects[rect_id].0.get_pos();
            let mut size = rects[rect_id].0.get_size();
            if pos.x + size.x > 480.0 || pos.x < 0.0 { rects[rect_id].1.x *= -1.0  }
            if pos.y + size.y > 272.0 || pos.y < 0.0 { rects[rect_id].1.y *= -1.0  }
            pos.x += rects[rect_id].1.x;
            pos.y += rects[rect_id].1.y;
            rects[rect_id].0.set_pos(pos);
            rects[rect_id].0.draw();
        }

        if input_manager.is_key_down_changed(Buttons::Cross) {
            let new_rect = Primitive::Rect::new(
                Vec3::new(240.0, 136.0, -1.0),
                Vec2::new(5.0, 5.0),
                Colors::BLUE.as_color(),
            );
            rects.push((new_rect.clone(), Vec2::new(1.0, 1.0)));
        }

        running = input_manager.is_key_up(Buttons::Triangle);

        canvas.end_frame();
    }

    canvas.terminate();
}
