use glam::Vec3;
use noise::{core::perlin::perlin_3d, permutationtable::PermutationTable};

use crate::chunk::{Node, NodeSampler};

pub struct PerlinSampler {
    hasher: PermutationTable,
}

impl PerlinSampler {
    const MAX_VALUE: f64 = u16::MAX as f64;

    pub fn new() -> Self {
        Self {
            hasher: PermutationTable::new(42),
        }
    }
}

impl NodeSampler for PerlinSampler {
    fn sample(&self, point: Vec3) -> Node {
        let noise_value = perlin_3d(point.as_dvec3().into(), &self.hasher);
        let weight = (noise_value * Self::MAX_VALUE).clamp(0., Self::MAX_VALUE) as u16;
        Node { weight }
    }
}
