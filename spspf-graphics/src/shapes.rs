use core::ptr;
use psp::{
    math,
    sys::{
        sceGuDisable, sceGumDrawArray, sceGumLoadIdentity, sceGumMatrixMode, sceGumTranslate,
        sceKernelDcacheWritebackInvalidateAll, GuPrimitive, GuState, MatrixMode, ScePspFVector3,
        VertexType,
    },
    Align16,
};
use spspf_core::{Vec2, Vec3};

use crate::{colors::Color, Vertex};

const STEPS: i32 = 100;
const PI: f32 = 3.1415926536;
const ANGLE: f32 = PI * 2.0 / STEPS as f32;

#[derive(Clone)]
pub enum Shape {
    Rect {
        vertices: Align16<[Vertex; 4]>,
        indices: Align16<[u16; 6]>,
        position: Vec3<f32>,
        size: Vec2<f32>,

        color: Color,
    },
    Triangle {
        vertices: Align16<[Vertex; 3]>,
        position: Vec3<f32>,

        color: Color,
    },
    Circle {
        vertices: Align16<[Vertex; (STEPS + 1) as usize]>,
        position: Vec3<f32>,

        color: Color,
    },
}

impl Shape {
    pub fn new_rect(position: Vec3<f32>, size: Vec2<f32>, color: Color) -> Shape {
        let vertices: Align16<[Vertex; 4]> = Align16([
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: size.x,
                y: 0.0,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: 0.0,
                y: size.y,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: size.x,
                y: size.y,
                z: -1.0,
            },
        ]);

        let indices: Align16<[u16; 6]> = Align16([0, 1, 2, 2, 1, 3]);

        Shape::Rect {
            vertices: vertices,
            indices: indices,
            position: position,
            size: size,
            color: color,
        }
    }

    pub fn new_triangle(vertices: [Vec3<f32>; 3], color: Color) -> Shape {
        //TODO: Normalize vertices to keep clockwise order
        let n_vertices: Align16<[Vertex; 3]> = Align16([
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: vertices[0].x,
                y: vertices[0].y,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: vertices[1].x,
                y: vertices[1].y,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: vertices[2].x,
                y: vertices[2].y,
                z: -1.0,
            },
        ]);

        Shape::Triangle {
            vertices: n_vertices,
            position: Vec3::new(vertices[1].x, vertices[1].y, vertices[1].z),
            color: color,
        }
    }

    pub fn new_circle(center: Vec3<f32>, radius: f32, color: Color) -> Shape {
        let mut vertices: Align16<[Vertex; (STEPS + 1) as usize]> =
            Align16([Vertex::default(); (STEPS + 1) as usize]);

        for i in 0..STEPS + 1 {
            vertices.0[i as usize].u = 0.0;
            vertices.0[i as usize].v = 0.0;
            vertices.0[i as usize].color = color.as_abgr();
            vertices.0[i as usize].x = radius * math::sin(ANGLE as f64 * (i - 1) as f64) as f32;
            vertices.0[i as usize].y = radius * math::cos(ANGLE as f64 * (i - 1) as f64) as f32;
            vertices.0[i as usize].z = -1.0;
        }

        Shape::Circle {
            vertices: vertices,
            position: Vec3::new(center.x, center.y, center.z),
            color: color,
        }
    }
}

impl crate::Drawable for Shape {
    fn draw(&mut self) {
        unsafe {
            sceGuDisable(GuState::Texture2D);
        }
        match self {
            Shape::Rect {
                vertices,
                indices,
                position,
                size: _,
                color: _,
            } => unsafe {
                // Reposition
                sceGumMatrixMode(MatrixMode::Model);
                sceGumLoadIdentity();
                sceGumTranslate(&ScePspFVector3 {
                    x: position.x,
                    y: position.y,
                    z: -1.0,
                });

                sceKernelDcacheWritebackInvalidateAll();
                sceGumDrawArray(
                    GuPrimitive::Triangles,
                    VertexType::TEXTURE_32BITF
                        | VertexType::INDEX_16BIT
                        | VertexType::COLOR_8888
                        | VertexType::VERTEX_32BITF
                        | VertexType::TRANSFORM_3D,
                    6,
                    indices as *const Align16<_> as *const _,
                    vertices as *const Align16<_> as *const _,
                );
            },
            Shape::Triangle {
                vertices,
                position,
                color: _,
            } => unsafe {
                // Reposition
                sceGumMatrixMode(MatrixMode::Model);
                sceGumLoadIdentity();
                sceGumTranslate(&ScePspFVector3 {
                    x: position.x,
                    y: position.y,
                    z: -1.0,
                });

                sceKernelDcacheWritebackInvalidateAll();
                sceGumDrawArray(
                    GuPrimitive::Triangles,
                    VertexType::TEXTURE_32BITF
                        | VertexType::COLOR_8888
                        | VertexType::VERTEX_32BITF
                        | VertexType::TRANSFORM_3D,
                    3,
                    ptr::null_mut(),
                    vertices as *const Align16<_> as *const _,
                );
            },
            Shape::Circle {
                vertices,
                position,
                color: _,
            } => unsafe {
                // Reposition
                sceGumMatrixMode(MatrixMode::Model);
                sceGumLoadIdentity();
                sceGumTranslate(&ScePspFVector3 {
                    x: position.x,
                    y: position.y,
                    z: -1.0,
                });

                sceKernelDcacheWritebackInvalidateAll();
                sceGumDrawArray(
                    GuPrimitive::TriangleFan,
                    VertexType::TEXTURE_32BITF
                        | VertexType::COLOR_8888
                        | VertexType::VERTEX_32BITF
                        | VertexType::TRANSFORM_3D,
                    STEPS + 1,
                    ptr::null_mut(),
                    vertices as *const Align16<_> as *const _,
                );
            },
        }
    }

    fn get_size(&mut self) -> Vec2<f32> {
        todo!("Not implemented!")
    }

    fn set_size(&mut self, _new_size: Vec2<f32>) {
        todo!("Not implemented!")
    }

    fn set_pos(&mut self, new_position: Vec3<f32>) {
        match self {
            Shape::Rect {
                vertices: _,
                indices: _,
                ref mut position,
                size: _,
                color: _,
            } => {
                *position = new_position;
            }
            Shape::Circle {
                vertices: _,
                ref mut position,
                color: _,
            } => {
                *position = new_position;
            }
            Shape::Triangle {
                vertices: _,
                ref mut position,
                color: _,
            } => {
                *position = new_position;
            }
        }
    }

    fn get_pos(&mut self) -> Vec3<f32> {
        match self {
            Shape::Rect {
                vertices: _,
                indices: _,
                position,
                size: _,
                color: _,
            } => position.clone(),
            Shape::Circle {
                vertices: _,
                position,
                color: _,
            } => position.clone(),
            Shape::Triangle {
                vertices: _,
                position,
                color: _,
            } => position.clone(),
        }
    }

    fn get_rot(&mut self) -> f32 {
        todo!("Not implemented!")
    }

    fn set_rot(&mut self, _new_rotation: f32) {
        todo!("Not implemented!")
    }
}
