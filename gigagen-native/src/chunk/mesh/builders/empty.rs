use crate::chunk::{mesh::MeshBuilder, ChunkData, ChunkMesh};

pub struct EmptyMeshBuilder;

impl MeshBuilder for EmptyMeshBuilder {
    fn build_mesh(&self, data: &ChunkData) -> ChunkMesh {
        let _ = data;
        ChunkMesh {}
    }
}
