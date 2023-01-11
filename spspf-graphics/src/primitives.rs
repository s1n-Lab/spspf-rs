#[allow(non_snake_case)]
pub mod Primitive {
    use crate::{utils::sort_vertices, Color, Drawable, Vertex, PI};
    use core::ptr;
    extern crate alloc;
    use psp::{
        math,
        sys::{
            sceGuDisable, sceGumDrawArray, sceGumLoadIdentity, sceGumMatrixMode, sceGumPopMatrix,
            sceGumPushMatrix, sceGumRotateZ, sceGumTranslate, GuPrimitive, GuState, MatrixMode,
            ScePspFVector3, VertexType,
        },
        Align16,
    };
    use spspf_core::{Vec2, Vec3};

    const STEPS: i32 = 100;
    const ANGLE: f32 = PI * 2.0 / STEPS as f32;

    #[derive(Clone)]
    pub struct Rect {
        vertices: Align16<[Vertex; 4]>,
        indices: Align16<[u16; 6]>,

        position: Vec3<f32>,
        size: Vec2<f32>,
        rotation: f32,

        color: Color,
    }

    impl Rect {
        pub fn new(position: Vec3<f32>, size: Vec2<f32>, color: Color) -> Self {
            Self {
                vertices: Self::generate_vertices(size, color.clone()),
                indices: Align16([0, 1, 2, 2, 1, 3]),
                position,
                rotation: 0.0,
                size,
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
            ])
        }
    }

    impl Drawable for Rect {
        fn draw(&mut self) {
            unsafe {
                sceGuDisable(GuState::Texture2D);
                sceGumMatrixMode(MatrixMode::Model);

                sceGumPushMatrix();

                sceGumLoadIdentity();

                sceGumTranslate(&ScePspFVector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: -1.0,
                });
                sceGumRotateZ(self.rotation);

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
            self.vertices = Self::generate_vertices(self.size, self.color.clone());
        }

        fn get_pos(&mut self) -> Vec3<f32> {
            self.position
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

    #[derive(Clone)]
    pub struct Triangle {
        vertices: Align16<[Vertex; 3]>,
        position: Vec3<f32>,

        color: Color,
    }

    impl Triangle {
        pub fn new(vertices: [Vec3<f32>; 3], color: Color) -> Self {
            Self {
                vertices: Self::generate_vertices(vertices, color),
                position: Vec3::new(vertices[1].x, vertices[1].y, vertices[1].z),
                color,
            }
        }

        pub(crate) fn generate_vertices(
            vertex_pos: [Vec3<f32>; 3],
            color: Color,
        ) -> Align16<[Vertex; 3]> {
            Align16(sort_vertices(
                [
                    Vertex {
                        u: 0.0,
                        v: 0.0,
                        color: color.as_abgr(),
                        x: vertex_pos[0].x,
                        y: vertex_pos[0].y,
                        z: -1.0,
                    },
                    Vertex {
                        u: 0.0,
                        v: 0.0,
                        color: color.as_abgr(),
                        x: vertex_pos[1].x,
                        y: vertex_pos[1].y,
                        z: -1.0,
                    },
                    Vertex {
                        u: 0.0,
                        v: 0.0,
                        color: color.as_abgr(),
                        x: vertex_pos[2].x,
                        y: vertex_pos[2].y,
                        z: -1.0,
                    },
                ],
                true,
            ))
        }
    }

    impl Drawable for Triangle {
        fn draw(&mut self) {
            unsafe {
                sceGuDisable(GuState::Texture2D);
                sceGumMatrixMode(MatrixMode::Model);

                sceGumPushMatrix();

                sceGumLoadIdentity();

                /*sceGumTranslate(&ScePspFVector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: -1.0,
                });*/

                sceGumDrawArray(
                    GuPrimitive::Triangles,
                    VertexType::TEXTURE_32BITF
                        | VertexType::COLOR_8888
                        | VertexType::VERTEX_32BITF
                        | VertexType::TRANSFORM_3D,
                    3,
                    ptr::null_mut(),
                    &self.vertices as *const Align16<_> as *const _,
                );

                sceGumPopMatrix();
            }
        }

        fn get_size(&mut self) -> Vec2<f32> {
            todo!()
        }

        fn set_size(&mut self, _new_size: Vec2<f32>) {
            todo!()
        }

        fn get_pos(&mut self) -> Vec3<f32> {
            self.position
        }

        fn set_pos(&mut self, new_position: Vec3<f32>) {
            self.position = new_position;
        }

        fn get_rot(&mut self) -> f32 {
            todo!()
        }

        fn set_rot(&mut self, _new_rotation: f32) {
            todo!()
        }
    }

    #[derive(Clone)]
    pub struct Ellipse {
        vertices: Align16<[Vertex; (STEPS + 1) as usize]>,
        position: Vec3<f32>,
        radius: Vec2<f32>,

        color: Color,
    }

    impl Ellipse {
        pub fn new(center: Vec3<f32>, radius: Vec2<f32>, color: Color) -> Self {
            let mut vertices: Align16<[Vertex; (STEPS + 1) as usize]> =
                Align16([Vertex::default(); (STEPS + 1) as usize]);

            for i in 0..STEPS + 1 {
                vertices.0[i as usize].u = 0.0;
                vertices.0[i as usize].v = 0.0;
                vertices.0[i as usize].color = color.as_abgr();
                vertices.0[i as usize].x =
                    radius.x * math::sin(ANGLE as f64 * (i - 1) as f64) as f32;
                vertices.0[i as usize].y =
                    radius.y * math::cos(ANGLE as f64 * (i - 1) as f64) as f32;
                vertices.0[i as usize].z = -1.0;
            }

            Self {
                vertices: Align16(sort_vertices(vertices.0, true)),
                radius,
                position: Vec3::new(center.x, center.y, center.z),
                color,
            }
        }
    }

    impl Drawable for Ellipse {
        fn draw(&mut self) {
            unsafe {
                sceGuDisable(GuState::Texture2D);
                sceGumMatrixMode(MatrixMode::Model);

                sceGumPushMatrix();

                sceGumLoadIdentity();

                sceGumTranslate(&ScePspFVector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: -1.0,
                });

                sceGumDrawArray(
                    GuPrimitive::TriangleFan,
                    VertexType::TEXTURE_32BITF
                        | VertexType::COLOR_8888
                        | VertexType::VERTEX_32BITF
                        | VertexType::TRANSFORM_3D,
                    STEPS + 1,
                    ptr::null_mut(),
                    &self.vertices as *const Align16<_> as *const _,
                );

                sceGumPopMatrix();
            }
        }

        fn get_size(&mut self) -> Vec2<f32> {
            self.radius
        }

        fn set_size(&mut self, new_size: Vec2<f32>) {
            self.radius = new_size;
        }

        fn get_pos(&mut self) -> Vec3<f32> {
            self.position
        }

        fn set_pos(&mut self, new_position: Vec3<f32>) {
            self.position = new_position;
        }

        fn get_rot(&mut self) -> f32 {
            todo!()
        }

        fn set_rot(&mut self, _new_rotation: f32) {
            todo!()
        }
    }
}
