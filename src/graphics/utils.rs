use crate::graphics::Vertex;

pub(crate) fn sort_vertices<const N: usize>(vertices: [Vertex; N], clockwise: bool) -> [Vertex; N] {
    let mut vertices_sorted = vertices;
    if clockwise {
        vertices_sorted
            .sort_by(|a, b| psp::math::atan2f(b.x, b.y).total_cmp(&psp::math::atan2f(a.x, a.y)));
    } else {
        vertices_sorted
            .sort_by(|a, b| psp::math::atan2f(a.x, a.y).total_cmp(&psp::math::atan2f(b.x, b.y)));
    }

    vertices_sorted
}
