use glam::Vec3;

use crate::chunk::{
    mesh::{march_tables, MeshBuilder},
    ChunkData, ChunkMesh,
};

pub struct SimpleMeshBuilder;

impl MeshBuilder for SimpleMeshBuilder {
    fn build_mesh(&self, chunk_data: &ChunkData) -> ChunkMesh {
        let mut mesh = ChunkMesh::new();
        let axis_cubes = chunk_data.div() as usize + 1;
        let axis_nodes = chunk_data.div() as usize + 2;
        let axis_nodes_squared = axis_nodes * axis_nodes;
        for x in 0..axis_cubes {
            for y in 0..axis_cubes {
                for z in 0..axis_cubes {
                    let chunk_nodes = [
                        &chunk_data[x + 0 + (y + 0) * axis_nodes + (z + 0) * axis_nodes_squared],
                        &chunk_data[x + 0 + (y + 0) * axis_nodes + (z + 1) * axis_nodes_squared],
                        &chunk_data[x + 1 + (y + 0) * axis_nodes + (z + 1) * axis_nodes_squared],
                        &chunk_data[x + 1 + (y + 0) * axis_nodes + (z + 0) * axis_nodes_squared],
                        &chunk_data[x + 0 + (y + 1) * axis_nodes + (z + 0) * axis_nodes_squared],
                        &chunk_data[x + 0 + (y + 1) * axis_nodes + (z + 1) * axis_nodes_squared],
                        &chunk_data[x + 1 + (y + 1) * axis_nodes + (z + 1) * axis_nodes_squared],
                        &chunk_data[x + 1 + (y + 1) * axis_nodes + (z + 0) * axis_nodes_squared],
                    ];

                    let mut cube_index = 0u8;
                    match true {
                        _ if chunk_nodes[0].weight > 0 => cube_index |= 0b0000_0001,
                        _ if chunk_nodes[1].weight > 0 => cube_index |= 0b0000_0010,
                        _ if chunk_nodes[2].weight > 0 => cube_index |= 0b0000_0100,
                        _ if chunk_nodes[3].weight > 0 => cube_index |= 0b0000_1000,
                        _ if chunk_nodes[4].weight > 0 => cube_index |= 0b0001_0000,
                        _ if chunk_nodes[5].weight > 0 => cube_index |= 0b0010_0000,
                        _ if chunk_nodes[6].weight > 0 => cube_index |= 0b0100_0000,
                        _ if chunk_nodes[7].weight > 0 => cube_index |= 0b1000_0000,
                        _ => (),
                    }

                    if cube_index == 0 || cube_index == 255 {
                        continue;
                    }

                    let cube_offset = cube_index as usize * 16;
                    for i in (0..16).skip(3) {
                        let tri_index = i + cube_offset;
                        let r1i = march_tables::TRIANGULATION[tri_index];
                        if r1i == -1 {
                            break;
                        }

                        let voxel_offset = Vec3 {
                            x: x as f32,
                            y: y as f32,
                            z: z as f32,
                        } * chunk_data.size();

                        let r1i = r1i as usize;
                        let r2i = march_tables::TRIANGULATION[tri_index + 1] as usize;
                        let r3i = march_tables::TRIANGULATION[tri_index + 2] as usize;

                        let r1 = &march_tables::CORNER_RAY_FROM_EDGE[r1i];
                        let r2 = &march_tables::CORNER_RAY_FROM_EDGE[r2i];
                        let r3 = &march_tables::CORNER_RAY_FROM_EDGE[r3i];

                        let w1 = chunk_nodes[r1.corner as usize].weight as f32 / 255.;
                        let w2 = chunk_nodes[r2.corner as usize].weight as f32 / 255.;
                        let w3 = chunk_nodes[r3.corner as usize].weight as f32 / 255.;

                        let v1 = (r1.origin + (r1.dir * w1)) * chunk_data.size();
                        let v2 = (r2.origin + (r2.dir * w2)) * chunk_data.size();
                        let v3 = (r3.origin + (r3.dir * w3)) * chunk_data.size();
                        let normal = (v2 - v1).cross(v3 - v1);

                        let vertex_count = mesh.vertices.len();
                        mesh.vertices.push(v1 + voxel_offset);
                        mesh.vertices.push(v2 + voxel_offset);
                        mesh.vertices.push(v3 + voxel_offset);

                        mesh.normals.push(normal);
                        mesh.normals.push(normal);
                        mesh.normals.push(normal);

                        mesh.indices.push(vertex_count as i32);
                        mesh.indices.push(vertex_count as i32 + 1);
                        mesh.indices.push(vertex_count as i32 + 2);
                    }
                }
            }
        }

        mesh
    }
}
