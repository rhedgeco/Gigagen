use std::ops::{Index, IndexMut};

use glam::Vec3;

#[repr(C)]
pub struct Node {
    pub weight: u16,
}

pub trait NodeSampler {
    fn sample(&self, point: Vec3) -> Node;
}

pub struct ChunkData {
    index: usize,
    nodes: Box<[Node]>,
    pos: Vec3,
    size: f32,
    div: u8,
}

impl IndexMut<usize> for ChunkData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Index<usize> for ChunkData {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl ChunkData {
    pub fn new(index: usize, pos: Vec3, size: f32, div: u8, sampler: &impl NodeSampler) -> Self {
        let axis_nodes = div as usize + 2;
        let node_size = size / (div as f32 + 1.);
        let node_count = axis_nodes * axis_nodes * axis_nodes;
        let mut nodes = Vec::with_capacity(node_count);
        for x in 0..axis_nodes {
            let x_point = pos.x + x as f32 * node_size;
            for y in 0..axis_nodes {
                let y_point = pos.y + y as f32 * node_size;
                for z in 0..axis_nodes {
                    let z_point = pos.z + z as f32 * node_size;
                    let point = Vec3 {
                        x: x_point,
                        y: y_point,
                        z: z_point,
                    };
                    nodes.push(sampler.sample(point));
                }
            }
        }

        Self {
            index,
            nodes: nodes.into_boxed_slice(),
            pos,
            size,
            div,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn div(&self) -> u8 {
        self.div
    }

    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn nodes_mut(&mut self) -> &mut [Node] {
        &mut self.nodes
    }
}
