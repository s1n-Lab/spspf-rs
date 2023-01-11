#![no_std]
#![no_main]

extern crate alloc;
use psp::Align16;
use spspf_core::{
    input::{Buttons, InputManager},
    Vec2, Vec3,
};
use spspf_graphics::{Canvas, Colors, Drawable, Primitive, Sprite};

psp::module!("SPSPF - Demo", 1, 1);

pub static FERRIS: Align16<[u8; 128 * 128 * 4 as usize]> =
    Align16(*include_bytes!("../ferris.bin"));

fn psp_main() {
    let mut canvas = Canvas::new();
    let mut input_manager = InputManager::new();

    let mut rect = Primitive::Rect::new(
        Vec3::new(240.0, 136.0, -1.0),
        Vec2::new(240.0, 136.0),
        Colors::BLUE.as_color(),
    );

    let mut triangle = Primitive::Triangle::new(
        [
            Vec3::new(0.0, 272.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(480.0, 0.0, -1.0),
        ],
        Colors::RED.as_color(),
    );

    let mut ellipse = Primitive::Ellipse::new(
        Vec3::new(240.0, 136.0, -1.0),
        Vec2::new(75.0, 50.0),
        Colors::WHITE.as_color(),
    );

    let mut sprite = Sprite::new(
        Vec3::new(217.5, 113.5, -1.0),
        0.0,
        Vec2::new(45.0, 45.0),
        FERRIS,
        Colors::WHITE.as_color(),
    );

    let mut draw_rect = true;
    let mut draw_triangle = true;
    let mut draw_ellipse = true;

    loop {
        canvas.start_frame();
        input_manager.update();

        canvas.clear(Colors::BLACK.as_color());

        if draw_triangle {
            triangle.draw();
        }
        if draw_rect {
            rect.draw();
        }
        if draw_ellipse {
            ellipse.draw();
        }

        sprite.draw();

        if input_manager.is_key_down_changed(Buttons::Square) {
            draw_rect = !draw_rect;
        }
        if input_manager.is_key_down_changed(Buttons::Triangle) {
            draw_triangle = !draw_triangle;
        }
        if input_manager.is_key_down_changed(Buttons::Circle) {
            draw_ellipse = !draw_ellipse;
        }

        if input_manager.is_key_down_changed(Buttons::Cross) {
            break;
        }

        // Scale sprite
        if input_manager.is_key_down(Buttons::Left) {
            let mut scale = sprite.get_scale();
            if scale.x > 0.0 {
                scale.x -= 0.05;
                sprite.set_scale(scale);
            }
        }
        if input_manager.is_key_down(Buttons::Right) {
            let mut scale = sprite.get_scale();
            scale.x += 0.05;
            sprite.set_scale(scale);
        }
        if input_manager.is_key_down(Buttons::Up) {
            let mut scale = sprite.get_scale();
            if scale.y > 0.0 {
                scale.y -= 0.05;
                sprite.set_scale(scale);
            }
        }
        if input_manager.is_key_down(Buttons::Down) {
            let mut scale = sprite.get_scale();
            scale.y += 0.05;
            sprite.set_scale(scale);
        }

        // Move sprite
        let analog = input_manager.get_analog_pos();
        let mut pos = sprite.get_pos();
        pos.x += analog.x as f32 * 1.0;
        pos.y += analog.y as f32 * 1.0;
        sprite.set_pos(pos);

        if input_manager.is_key_down(Buttons::Select) {
            let mut size = sprite.get_size();
            if size.x > 1.0 {
                size.x -= 1.0;
                size.y -= 1.0;
                sprite.set_size(size)
            }
        }
        if input_manager.is_key_down(Buttons::Start) {
            let mut size = sprite.get_size();
            size.x += 1.0;
            size.y += 1.0;
            sprite.set_size(size)
        }

        // Rotates sprite
        if input_manager.is_key_down(Buttons::LTrigger) {
            let mut rot = sprite.get_rot();
            if rot <= 1.0 {
                rot = 360.0;
            }
            rot -= 2.0;
            sprite.set_rot(rot);
        }
        if input_manager.is_key_down(Buttons::RTrigger) {
            let mut rot = sprite.get_rot();
            if rot >= 359.0 {
                rot = 0.0;
            }
            rot += 2.0;
            sprite.set_rot(rot);
        }

        canvas.end_frame();
    }

    canvas.terminate();
}
