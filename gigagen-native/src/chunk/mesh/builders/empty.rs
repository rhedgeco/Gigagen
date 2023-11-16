use crate::chunk::{mesh::MeshBuilder, ChunkData, ChunkMesh};

pub struct EmptyMeshBuilder;

impl MeshBuilder for EmptyMeshBuilder {
    fn build_mesh(&self, chunk_data: &ChunkData) -> ChunkMesh {
        let _ = chunk_data;
        ChunkMesh::new()
    }
}
