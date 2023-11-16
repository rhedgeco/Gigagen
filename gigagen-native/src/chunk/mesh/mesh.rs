use glam::Vec3;

use crate::chunk::ChunkData;

pub trait MeshBuilder {
    fn build_mesh(&self, chunk_data: &ChunkData) -> ChunkMesh;
}

#[derive(Default)]
pub struct ChunkMesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<i32>,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self::default()
    }
}
