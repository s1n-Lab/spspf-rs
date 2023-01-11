#[allow(non_snake_case)]
pub mod Primitive {
    use crate::{utils::sort_vertices, Color, Drawable, Vertex, PI};
    use core::ptr;
    extern crate alloc;
    use alloc::vec::Vec;
    use psp::{
        math,
        sys::{
            sceGuDisable, sceGumDrawArray, sceGumLoadIdentity, sceGumMatrixMode, sceGumPopMatrix,
            sceGumPushMatrix, sceGumRotateZ, sceGumScale, sceGumTranslate, GuPrimitive, GuState,
            MatrixMode, ScePspFVector3, VertexType,
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
        scale: Vec2<f32>,
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
                scale: Vec2::new(1.0, 1.0),
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
                sceGumScale(&ScePspFVector3 {
                    x: self.scale.x,
                    y: self.scale.y,
                    z: 1.0,
                });

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

        fn get_scale(&mut self) -> Vec2<f32> {
            self.scale
        }

        fn set_scale(&mut self, new_scale: Vec2<f32>) {
            self.scale = new_scale;
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
        rotation: f32,
        scale: Vec2<f32>,

        color: Color,
    }

    impl Triangle {
        pub fn new(vertices: [Vec3<f32>; 3], color: Color) -> Self {
            Self {
                vertices: Self::generate_vertices(vertices, color),
                position: Vec3::new(vertices[1].x, vertices[1].y, vertices[1].z),
                rotation: 0.0,
                scale: Vec2::new(1.0, 1.0),
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

                sceGumRotateZ(self.rotation);
                sceGumScale(&ScePspFVector3 {
                    x: self.scale.x,
                    y: self.scale.y,
                    z: 1.0,
                });

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
            // Gets smallest and largest X
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            for i in 0..self.vertices.0.len() {
                if self.vertices.0[i].x < min_x {
                    min_x = self.vertices.0[i].x
                }
                if self.vertices.0[i].x > max_x {
                    max_x = self.vertices.0[i].x
                }
            }

            // Gets smallest and largest Y
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;
            for i in 0..self.vertices.0.len() {
                if self.vertices.0[i].y < min_y {
                    min_y = self.vertices.0[i].y
                }
                if self.vertices.0[i].y > max_y {
                    max_y = self.vertices.0[i].y
                }
            }

            // Returns difference between largest and smallest coordinates in each axis
            Vec2::new(max_x - min_x, max_y - min_y)
        }

        fn set_size(&mut self, new_size: Vec2<f32>) {
            let old_size = self.get_size();
            let difference = Vec2::new(new_size.x - old_size.x, new_size.y - old_size.y);

            // Gets smallest and largest X vertex
            let mut min_x = f32::MAX;
            let mut min_x_id: Vec<usize> = Vec::new();
            for i in 0..self.vertices.0.len() {
                if self.vertices.0[i].x < min_x {
                    min_x = self.vertices.0[i].x;
                    min_x_id = alloc::vec![i];
                }
                if self.vertices.0[i].x == min_x {
                    min_x_id.push(i)
                }
            }

            // Gets smallest and largest Y vertex
            let mut min_y = f32::MAX;
            let mut min_y_id: Vec<usize> = Vec::new();
            for i in 0..self.vertices.0.len() {
                if self.vertices.0[i].y < min_y {
                    min_y = self.vertices.0[i].y;
                    min_y_id = alloc::vec![i];
                }
                if self.vertices.0[i].x == min_x {
                    min_x_id.push(i)
                }
            }

            // Moves all axis except the smallest ones on each axis by the difference
            let mut vertices = self.vertices;
            for i in 0..vertices.0.len() {
                if !min_x_id.contains(&i) {
                    vertices.0[i].x += difference.x
                }
                if !min_y_id.contains(&i) {
                    vertices.0[i].y += difference.y
                }
            }
            self.vertices = Self::generate_vertices(
                [
                    Vec3::new(vertices.0[0].x, vertices.0[0].y, -vertices.0[0].z),
                    Vec3::new(vertices.0[1].x, vertices.0[1].y, -vertices.0[1].z),
                    Vec3::new(vertices.0[2].x, vertices.0[2].y, -vertices.0[2].z),
                ],
                self.color,
            );
        }

        fn get_scale(&mut self) -> Vec2<f32> {
            self.scale
        }

        fn set_scale(&mut self, new_scale: Vec2<f32>) {
            self.scale = new_scale;
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
    pub struct Ellipse {
        vertices: Align16<[Vertex; (STEPS + 1) as usize]>,

        position: Vec3<f32>,
        rotation: f32,
        radius: Vec2<f32>,
        scale: Vec2<f32>,

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
                scale: Vec2::new(1.0, 1.0),
                position: Vec3::new(center.x, center.y, center.z),
                rotation: 0.0,
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
                sceGumRotateZ(self.rotation);
                sceGumScale(&ScePspFVector3 {
                    x: self.scale.x,
                    y: self.scale.y,
                    z: 1.0,
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

        fn get_scale(&mut self) -> Vec2<f32> {
            self.scale
        }

        fn set_scale(&mut self, new_scale: Vec2<f32>) {
            self.scale = new_scale;
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
}
