use crate::brush::{Brush, Plane};
use crate::types::Vec3;

const EPSILON: f32 = 0.001;

pub struct Mesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

fn intersect_three_planes(p1: &Plane, p2: &Plane, p3: &Plane) -> Option<Vec3> {
    let n1 = p1.normal;
    let n2 = p2.normal;
    let n3 = p3.normal;

    let det = n1.x * (n2.y * n3.z - n2.z * n3.y)
        - n1.y * (n2.x * n3.z - n2.z * n3.x)
        + n1.z * (n2.x * n3.y - n2.y * n3.x);

    if det.abs() < EPSILON {
        return None;
    }

    let x = p1.distance * (n2.y * n3.z - n2.z * n3.y)
        - n1.y * (p2.distance * n3.z - n2.z * p3.distance)
        + n1.z * (p2.distance * n3.y - n2.y * p3.distance);

    let y = n1.x * (p2.distance * n3.z - n2.z * p3.distance)
        - p1.distance * (n2.x * n3.z - n2.z * n3.x)
        + n1.z * (n2.x * p3.distance - p2.distance * n3.x);

    let z = n1.x * (n2.y * p3.distance - p2.distance * n3.y)
        - n1.y * (n2.x * p3.distance - p2.distance * n3.x)
        + p1.distance * (n2.x * n3.y - n2.y * n3.x);

    Some(Vec3 {
        x: x / det,
        y: y / det,
        z: z / det,
    })
}

fn point_inside_planes(point: &Vec3, planes: &[Plane], except: [usize; 3]) -> bool {
    for (i, plane) in planes.iter().enumerate() {
        if except.contains(&i) {
            continue;
        }
        let dot = plane.normal.x * point.x
            + plane.normal.y * point.y
            + plane.normal.z * point.z;

        if dot - plane.distance > EPSILON {
            return false;
        }
    }
    true
}

fn point_on_plane(point: &Vec3, plane: &Plane) -> bool {
    let dot = plane.normal.x * point.x
        + plane.normal.y * point.y
        + plane.normal.z * point.z;
    (dot - plane.distance).abs() < EPSILON
}

fn sort_vertices_by_angle(verts: &[Vec3], normal: &Vec3) -> Vec<Vec3> {
    if verts.is_empty() {
        return Vec::new();
    }

    let centre = verts.iter().fold(
        Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        |acc, v| Vec3 {
            x: acc.x + v.x,
            y: acc.y + v.y,
            z: acc.z + v.z,
        }
    );
    let count = verts.len() as f32;
    let centre = Vec3 {
        x: centre.x / count,
        y: centre.y / count,
        z: centre.z / count,
    };

    let ref_vec = Vec3 {
        x: verts[0].x - centre.x,
        y: verts[0].y - centre.y,
        z: verts[0].z - centre.z,
    };

    let mut indexed: Vec<(usize, f32)> = verts.iter().enumerate().map(|(i, v)| {
        let to_v = Vec3 {
            x: v.x - centre.x,
            y: v.y - centre.y,
            z: v.z - centre.z,
        };

        let cross = Vec3 {
            x: ref_vec.y * to_v.z - ref_vec.z * to_v.y,
            y: ref_vec.z * to_v.x - ref_vec.x * to_v.z,
            z: ref_vec.x * to_v.y - ref_vec.y * to_v.x,
        };

        let dot_cross = cross.x * normal.x
            + cross.y * normal.y
            + cross.z * normal.z;

        let dot = ref_vec.x * to_v.x
            + ref_vec.y * to_v.y
            + ref_vec.z * to_v.z;

        let angle = dot_cross.atan2(dot);
        (i, angle)
    }).collect();

    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    indexed.iter().map(|(i, _)| verts[*i]).collect()
}

fn triangulate(verts: &[Vec3], inverted: bool) -> Vec<u32> {
    let mut indices = Vec::new();
    for i in 1..(verts.len() - 1) {
        if inverted {
            indices.push(0u32);
            indices.push((i + 1) as u32);
            indices.push(i as u32);
        } else {
            indices.push(0u32);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }
    }
    indices
}

pub fn brush_to_mesh(brush: &Brush) -> Mesh {
    let planes = &brush.planes;
    let mut vertices: Vec<Vec3> = Vec::new();

    // Step 1 - find all valid vertices
    for i in 0..planes.len() {
        for j in (i + 1)..planes.len() {
            for k in (j + 1)..planes.len() {
                if let Some(point) = intersect_three_planes(
                    &planes[i],
                    &planes[j],
                    &planes[k],
                ) {
                    if point_inside_planes(&point, planes, [i, j, k]) {
                        vertices.push(point);
                    }
                }
            }
        }
    }

    // Step 2 - build faces
    let mut final_vertices: Vec<[f32; 3]> = Vec::new();
    let mut final_indices: Vec<u32> = Vec::new();

    for plane in planes.iter() {
        let face_verts: Vec<Vec3> = vertices
            .iter()
            .filter(|v| point_on_plane(v, plane))
            .copied()
            .collect();

        if face_verts.len() < 3 {
            continue;
        }

        let sorted = sort_vertices_by_angle(&face_verts, &plane.normal);

        let base = final_vertices.len() as u32;
        let tris = triangulate(&sorted, brush.inverted);

        for v in &sorted {
            final_vertices.push([v.x, v.y, v.z]);
        }

        for idx in tris {
            final_indices.push(base + idx);
        }
    }

    Mesh {
        vertices: final_vertices,
        indices: final_indices,
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::brush::Brush;
    use uuid::Uuid;

    fn make_box_brush(half_size: f32) -> Brush {
        let mut brush = Brush::new(Uuid::new_v4());
        let s = half_size;

        // 6 planes for a box centered at origin
        brush.planes = vec![
            Plane { normal: Vec3 { x:  1.0, y: 0.0, z: 0.0 }, distance:  s }, // right
            Plane { normal: Vec3 { x: -1.0, y: 0.0, z: 0.0 }, distance:  s }, // left
            Plane { normal: Vec3 { x: 0.0, y:  1.0, z: 0.0 }, distance:  s }, // top
            Plane { normal: Vec3 { x: 0.0, y: -1.0, z: 0.0 }, distance:  s }, // bottom
            Plane { normal: Vec3 { x: 0.0, y: 0.0, z:  1.0 }, distance:  s }, // front
            Plane { normal: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, distance:  s }, // back
        ];

        brush
    }

    #[test]
    fn box_brush_has_8_unique_vertices() {
        let brush = make_box_brush(1.0);
        let mesh = brush_to_mesh(&brush);

        // A box has 8 unique corner vertices
        // (mesh may have duplicates per face, so check >= 8)
        assert!(mesh.vertices.len() >= 8,
                "Expected at least 8 vertices, got {}", mesh.vertices.len());
    }

    #[test]
    fn box_brush_has_correct_index_count() {
        let brush = make_box_brush(1.0);
        let mesh = brush_to_mesh(&brush);

        // 6 faces * 2 triangles * 3 indices = 36
        assert_eq!(mesh.indices.len(), 36,
                   "Expected 36 indices, got {}", mesh.indices.len());
    }

    #[test]
    fn inverted_box_has_same_counts() {
        let mut brush = make_box_brush(1.0);
        brush.inverted = true;
        let mesh = brush_to_mesh(&brush);

        assert!(mesh.vertices.len() >= 8);
        assert_eq!(mesh.indices.len(), 36);
    }
}