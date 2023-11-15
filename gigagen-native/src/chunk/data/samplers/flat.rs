use crate::chunk::data::{Node, NodeSampler};

pub struct FlatSampler;

impl NodeSampler for FlatSampler {
    fn sample(&self, point: glam::Vec3) -> Node {
        match point.y {
            y if y < 0. => Node { weight: 1 },
            _ => Node { weight: 0 },
        }
    }
}
