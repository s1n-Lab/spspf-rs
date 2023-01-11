use psp::{
    sys::{
        sceGuEnable, sceGuTexFilter, sceGuTexFunc, sceGuTexImage, sceGuTexMode, sceGuTexOffset,
        sceGuTexScale, sceGuTexWrap, sceGumDrawArray, sceGumLoadIdentity, sceGumMatrixMode,
        sceGumPopMatrix, sceGumPushMatrix, sceGumRotateZ, sceGumTranslate, GuPrimitive, GuState,
        GuTexWrapMode, MatrixMode, MipmapLevel, ScePspFVector3, TextureColorComponent,
        TextureEffect, TextureFilter, TexturePixelFormat, VertexType,
    },
    Align16,
};
use spspf_core::{Vec2, Vec3};

use crate::{colors::Color, Drawable, Vertex, PI};

pub struct Sprite<const N: usize> {
    vertices: Align16<[Vertex; 4]>,
    indices: Align16<[u16; 6]>,
    position: Vec3<f32>,
    rotation: f32,
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
        let texture_size = psp::math::sqrtf(texture.0.len() as f32 / 4.0);

        Self {
            vertices: Self::generate_vertices(size, color),
            indices: Align16([0, 1, 2, 2, 1, 3]),
            position,
            rotation,
            size,
            texture_size,
            texture,
            color,
        }
    }

    pub(crate) fn generate_vertices(size: Vec2<f32>, color: Color) -> Align16<[Vertex; 4]> {
        Align16([
            Vertex {
                u: 0.0,
                v: 0.0,
                color: color.as_abgr(),
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            Vertex {
                u: size.x,
                v: 0.0,
                color: color.as_abgr(),
                x: size.x,
                y: 0.0,
                z: -1.0,
            },
            Vertex {
                u: 0.0,
                v: size.y,
                color: color.as_abgr(),
                x: 0.0,
                y: size.y,
                z: -1.0,
            },
            Vertex {
                u: size.x,
                v: size.y,
                color: color.as_abgr(),
                x: size.x,
                y: size.y,
                z: -1.0,
            },
        ])
    }
}

impl<const N: usize> Drawable for Sprite<N> {
    fn draw(&mut self) {
        unsafe {
            sceGuEnable(GuState::Texture2D);
            sceGumMatrixMode(MatrixMode::Model);

            sceGumPushMatrix();

            sceGumLoadIdentity();

            sceGumTranslate(&ScePspFVector3 {
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            });
            sceGumRotateZ(self.rotation);

            sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
            sceGuTexImage(
                MipmapLevel::None,
                self.texture_size as i32,
                self.texture_size as i32,
                self.texture_size as i32,
                &self.texture as *const Align16<_> as *const _,
            );
            sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgba);
            sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest);
            sceGuTexScale(1.0 / self.size.x, 1.0 / self.size.y);
            sceGuTexOffset(0.0, 0.0);
            sceGuTexWrap(GuTexWrapMode::Repeat, GuTexWrapMode::Repeat);

            sceGumDrawArray(
                GuPrimitive::Triangles,
                VertexType::TEXTURE_32BITF
                    | VertexType::INDEX_16BIT
                    | VertexType::COLOR_8888
                    | VertexType::VERTEX_32BITF
                    | VertexType::TRANSFORM_3D,
                6,
                &self.indices as *const Align16<_> as *const _,
                &self.vertices as *const Align16<_> as *const _,
            );

            sceGumPopMatrix();
        }
    }

    fn get_size(&mut self) -> Vec2<f32> {
        self.size
    }

    fn set_size(&mut self, new_size: Vec2<f32>) {
        self.size = new_size;
        let vertices = Self::generate_vertices(self.size, self.color);

        self.vertices = vertices;
    }

    fn get_pos(&mut self) -> Vec3<f32> {
        self.position.clone()
    }

    fn set_pos(&mut self, new_position: Vec3<f32>) {
        self.position = new_position;
    }

    fn get_rot(&mut self) -> f32 {
        self.rotation * (180.0 / PI)
    }

    fn set_rot(&mut self, new_rotation: f32) {
        self.rotation = new_rotation * (PI / 180.0);
    }
}

fn swizzle() {}
