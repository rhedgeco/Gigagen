use crate::chunk::ChunkData;

pub trait MeshBuilder {
    fn build_mesh(&self, data: &ChunkData) -> ChunkMesh;
}

pub struct ChunkMesh {}
