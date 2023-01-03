use core::ptr;

use psp::{
    sys::{
        sceGuEnable, sceGuTexFilter, sceGuTexImage, sceGuTexOffset, sceGuTexScale, sceGumDrawArray,
        sceGumLoadIdentity, sceGumMatrixMode, sceGumTranslate,
        sceKernelDcacheWritebackInvalidateAll, GuPrimitive, GuState, MatrixMode, MipmapLevel,
        ScePspFVector3, TextureFilter, VertexType,
    },
    Align16,
};
use spspf_core::{Vec2, Vec3};

use crate::{colors::Color, Drawable, Vertex};

pub struct Sprite<const N: usize> {
    vertices: Align16<[Vertex; 2]>,
    position: Vec3<f32>,
    _rotation: f32,
    size: Vec2<f32>,

    texture: Align16<[u8; N]>,
    color: Color,
    texture_size: f32,
}

impl<const N: usize> Sprite<N> {
    pub fn new(
        position: Vec3<f32>,
        rotation: f32,
        size: Vec2<f32>,
        texture: Align16<[u8; N]>,
        color: Color,
    ) -> Self {
        let vertices: Align16<[Vertex; 2]> = Align16([
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: 0.0,
                y: 0.0,
                z: position.z,
            },
            Vertex {
                u: size.x,
                v: size.y,
                color: color.as_abgr(),
                x: size.x,
                y: size.y,
                z: position.z,
            },
        ]);

        Self {
            vertices: vertices,
            position: position,
            _rotation: rotation,
            size: size,
            texture_size: (psp::math::sqrtf(texture.0.len() as f32 / 4.0)),
            texture: texture,
            color: color,
        }
    }
}

impl<const N: usize> Drawable for Sprite<N> {
    fn draw(&mut self) {
        unsafe {
            // Enable Texture2D
            sceGuEnable(GuState::Texture2D);

            // Reposition
            sceGumMatrixMode(MatrixMode::Model);
            sceGumLoadIdentity();
            sceGumTranslate(&ScePspFVector3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            });

            sceGuTexFilter(TextureFilter::Linear, TextureFilter::Linear);
            sceGuTexScale(1.0 / self.size.x, 1.0 / self.size.y);
            sceGuTexOffset(0.0, 0.0);
            sceKernelDcacheWritebackInvalidateAll();

            sceGuTexImage(
                MipmapLevel::None,
                self.texture_size as i32,
                self.texture_size as i32,
                self.texture_size as i32,
                &self.texture as *const Align16<_> as *const _,
            );
            sceGumDrawArray(
                GuPrimitive::Sprites,
                VertexType::TEXTURE_32BITF
                    | VertexType::COLOR_8888
                    | VertexType::VERTEX_32BITF
                    | VertexType::TRANSFORM_3D,
                2,
                ptr::null_mut(),
                &self.vertices as *const Align16<_> as *const _,
            )
        }
    }

    fn get_size(&mut self) -> Vec2<f32> {
        self.size
    }

    fn set_size(&mut self, new_size: Vec2<f32>) {
        self.size = new_size;
        let vertices: Align16<[Vertex; 2]> = Align16([
            Vertex {
                u: 0.0,
                v: 0.0,
                color: self.color.as_abgr(),
                x: 0.0,
                y: 0.0,
                z: self.position.z,
            },
            Vertex {
                u: new_size.x,
                v: new_size.y,
                color: self.color.as_abgr(),
                x: new_size.x,
                y: new_size.y,
                z: self.position.z,
            },
        ]);

        self.vertices = vertices;
    }

    fn get_pos(&mut self) -> Vec3<f32> {
        self.position.clone()
    }

    fn set_pos(&mut self, new_position: Vec3<f32>) {
        self.position = new_position;
    }

    fn get_rot(&mut self) -> f32 {
        todo!("Not implemented!");
    }

    fn set_rot(&mut self, _new_rotation: f32) {
        todo!("Not implemented!");
    }
}

fn swizzle() {}
