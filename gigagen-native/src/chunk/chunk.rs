use super::{mesh::MeshBuilder, ChunkData, ChunkMesh};

pub struct GigaChunk {
    world_index: usize,
    data: ChunkData,
    mesh: ChunkMesh,
}

impl GigaChunk {
    pub fn new(world_index: usize, data: ChunkData, mesh_builder: &impl MeshBuilder) -> Self {
        let mesh = mesh_builder.build_mesh(&data);
        Self {
            world_index,
            data,
            mesh,
        }
    }

    pub fn world_index(&self) -> usize {
        self.world_index
    }

    pub fn data(&self) -> &ChunkData {
        &self.data
    }

    pub fn mesh(&self) -> &ChunkMesh {
        &self.mesh
    }
}
