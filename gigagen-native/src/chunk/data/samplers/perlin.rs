use glam::Vec3;
use noise::{core::perlin::perlin_3d, permutationtable::PermutationTable};

use crate::chunk::data::{Node, NodeSampler};

pub struct PerlinSampler {
    hasher: PermutationTable,
}

impl PerlinSampler {
    pub fn new() -> Self {
        Self {
            hasher: PermutationTable::new(0),
        }
    }
}

impl NodeSampler for PerlinSampler {
    fn sample(&self, point: Vec3) -> Node {
        let dpoint = point.as_dvec3();
        let value = perlin_3d(dpoint.into(), &self.hasher);
        let fweight = (value + 1.) / 2. * u16::MAX as f64;
        Node {
            weight: fweight as u16,
        }
    }
}
