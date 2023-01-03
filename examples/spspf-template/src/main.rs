#![no_std]
#![no_main]

use psp::Align16;
use spspf_core::{input::{InputManager, Buttons}, Vec2, Vec3};
use spspf_graphics::{canvas::Canvas, shapes::Shape, colors::Colors, sprite::Sprite, Drawable};

psp::module!("SPSPF - Demo", 1, 1);

pub static FERRIS: Align16<[u8; 128 * 128 * 4 as usize]> = Align16(*include_bytes!("../ferris.bin"));

fn psp_main() {
    let mut running = true;
    let mut canvas = Canvas::new();
    let mut input_manager = InputManager::new();
    
    let mut rect = Shape::new_rect(
        Vec3::new(240.0, 136.0, -1.0),
        Vec2::new(240.0, 136.0),
        Colors::BLUE.as_color(),
    );

    let mut triangle = Shape::new_triangle(
        [
            Vec3::new(0.0, 272.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(480.0, 0.0, -1.0),
        ],
        Colors::RED.as_color(),
    );

    let mut circle = Shape::new_circle(Vec3::new(240.0, 136.0, -1.0), 50.0, Colors::WHITE.as_color());

    let mut sprite = Sprite::new(
        Vec3::new(217.5, 113.5, -1.0),
        0.0,
        Vec2::new(45.0, 45.0),
        FERRIS,
        Colors::WHITE.as_color(),
    );

    let mut draw_rect = true;
    let mut draw_triangle = true;
    let mut draw_circle = true;
    
    while running {
        canvas.start_frame();
        input_manager.update();

        canvas.clear(Colors::BLACK.as_color());

        if draw_triangle {
            triangle.draw();
        }
        if draw_rect {
            rect.draw();
        }
        if draw_circle {
            circle.draw();
        }

        sprite.draw();
        
        if input_manager.is_key_down_changed(Buttons::Square) {
            draw_rect = !draw_rect;
        }
        if input_manager.is_key_down_changed(Buttons::Triangle) {
            draw_triangle = !draw_triangle;
        }
        if input_manager.is_key_down_changed(Buttons::Circle) {
            draw_circle = !draw_circle;
        }

        // Move sprite
        if input_manager.is_key_down(Buttons::Left) {
            let mut pos = sprite.get_pos();
            if pos.x > 0.0 { pos.x -= 1.0; sprite.set_pos(pos); }
        }
        if input_manager.is_key_down(Buttons::Right) {
            let mut pos = sprite.get_pos();
            pos.x += 1.0;
            sprite.set_pos(pos)
        }
        if input_manager.is_key_down(Buttons::Up) {
            let mut pos = sprite.get_pos();
            if pos.y > 0.0 { pos.y -= 1.0; sprite.set_pos(pos); }
        }
        if input_manager.is_key_down(Buttons::Down) {
            let mut pos = sprite.get_pos();
            pos.y += 1.0;
            sprite.set_pos(pos)
        }
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
            }            
            sprite.set_size(size)
        }
        if input_manager.is_key_down(Buttons::Start) {
            let mut size = sprite.get_size();
            size.x += 1.0;
            size.y += 1.0;
            sprite.set_size(size)
        }

        running = input_manager.is_key_up(Buttons::RTrigger);

        canvas.end_frame();
    }

    canvas.terminate();
}
