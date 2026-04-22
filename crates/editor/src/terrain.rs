use noise::{NoiseFn, Perlin, Fbm};

pub struct TerrainMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl TerrainMesh {
    pub fn generate(size: f32, resolution: u32, height_scale: f32) -> Self {
        let fbm = Fbm::<Perlin>::new(42);
        let half = size / 2.0;
        let step = size / resolution as f32;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Generate vertices
        for z in 0..=resolution {
            for x in 0..=resolution {
                let wx = -half + x as f32 * step;
                let wz = -half + z as f32 * step;

                let nx = wx as f64 / size as f64;
                let nz = wz as f64 / size as f64;

                let height = fbm.get([nx * 3.0, nz * 3.0]) as f32 * height_scale;

                vertices.push([wx, height, wz]);
            }
        }

        // Generate indices
        let stride = resolution + 1;
        for z in 0..resolution {
            for x in 0..resolution {
                let tl = z * stride + x;
                let tr = tl + 1;
                let bl = tl + stride;
                let br = bl + 1;

                // First triangle
                indices.push(tl);
                indices.push(bl);
                indices.push(tr);

                // Second triangle
                indices.push(tr);
                indices.push(bl);
                indices.push(br);
            }
        }

        Self { vertices, indices }
    }
}