use glam::Vec3;

pub struct LoadedChunk {
    pub index: usize,
    pub pos: Vec3,
}

pub struct GigaWorld {
    center: Vec3,
    view_dist: u8,
    chunk_size: f32,
    unloaded: Vec<ChunkData>,
    loaded: Vec<ChunkData>,
}

impl GigaWorld {
    pub fn new(center: Vec3, view_dist: u8, chunk_size: f32) -> Self {
        let mut world = Self {
            center,
            view_dist,
            chunk_size,
            unloaded: Vec::new(),
            loaded: Vec::new(),
        };
        world.rebuild_chunk_vecs();
        world
    }

    pub fn set_view_dist(&mut self, view_dist: u8) {
        self.view_dist = view_dist;
        self.rebuild_chunk_vecs();
    }

    pub fn set_chunk_size(&mut self, chunk_size: f32) {
        self.chunk_size = chunk_size;
        self.rebuild_chunk_vecs();
    }

    pub fn set_data(&mut self, view_dist: u8, chunk_size: f32) {
        self.view_dist = view_dist;
        self.chunk_size = chunk_size;
        self.rebuild_chunk_vecs();
    }

    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;

        // rebuild the position for any unloaded chunks
        for chunk_data in self.unloaded.iter_mut() {
            chunk_data.rebuild_pos(self.center, self.view_dist, self.chunk_size);
        }

        // rebuild and unload the position for already loaded chunks
        // iterate backwards so that removing chunks does not mess with iteration
        let half_chunk = self.chunk_size / 2.;
        let max_dist = self.view_dist as f32 * self.chunk_size;
        for index in (0..self.loaded.len() - 1).rev() {
            let chunk_data = &mut self.loaded[index];
            if (chunk_data.pos.x + half_chunk - self.center.x).abs() > max_dist
                || (chunk_data.pos.y + half_chunk - self.center.y).abs() > max_dist
                || (chunk_data.pos.z + half_chunk - self.center.z).abs() > max_dist
            {
                let mut chunk_data = self.loaded.swap_remove(index);
                chunk_data.rebuild_pos(self.center, self.view_dist, self.chunk_size);
                self.unloaded.push(chunk_data)
            }
        }
    }

    pub fn load_next_chunk(&mut self) -> Option<LoadedChunk> {
        let mut next_index = None;
        let mut min_dist = f32::MAX;
        let half_chunk = Vec3::ONE * self.chunk_size / 2.;
        for (index, chunk_data) in self.unloaded.iter().enumerate() {
            let chunk_dist = (chunk_data.pos + half_chunk).distance(self.center);
            if chunk_dist < min_dist {
                next_index = Some(index);
                min_dist = chunk_dist;
            }
        }

        let next_index = next_index?;
        let chunk_data = self.unloaded.swap_remove(next_index);
        let loaded_chunk = LoadedChunk {
            index: chunk_data.global_index,
            pos: chunk_data.pos,
        };
        self.loaded.push(chunk_data);
        Some(loaded_chunk)
    }

    fn rebuild_chunk_vecs(&mut self) {
        let axis_size = self.view_dist as usize * 2;
        self.loaded = Vec::with_capacity(axis_size * axis_size * axis_size);
        self.unloaded = Vec::with_capacity(axis_size * axis_size * axis_size);
        for x_index in 0..axis_size {
            for y_index in 0..axis_size {
                for z_index in 0..axis_size {
                    self.unloaded.push(ChunkData::new(
                        self.center,
                        self.view_dist,
                        self.chunk_size,
                        x_index,
                        y_index,
                        z_index,
                    ))
                }
            }
        }
    }
}

struct ChunkData {
    pub global_index: usize,
    pub x_index: usize,
    pub y_index: usize,
    pub z_index: usize,
    pub pos: Vec3,
}

impl ChunkData {
    pub fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        x_index: usize,
        y_index: usize,
        z_index: usize,
    ) -> Self {
        let axis_length = view_dist as usize * 2;
        let global_index = x_index + y_index * axis_length + z_index * axis_length * axis_length;
        let mut data = Self {
            global_index,
            x_index,
            y_index,
            z_index,
            pos: Vec3::ZERO,
        };
        data.rebuild_pos(center, view_dist, chunk_size);
        data
    }

    pub fn rebuild_pos(&mut self, center: Vec3, view_dist: u8, chunk_size: f32) {
        let view_radius = view_dist as f32;
        let view_diameter = view_radius * 2.;
        let x_offset = (center.x / chunk_size).round() as f32;
        let y_offset = (center.y / chunk_size).round() as f32;
        let z_offset = (center.z / chunk_size).round() as f32;
        let new_x =
            (self.x_index as f32 - x_offset).rem_euclid(view_diameter) + x_offset - view_radius;
        let new_y =
            (self.y_index as f32 - y_offset).rem_euclid(view_diameter) + y_offset - view_radius;
        let new_z =
            (self.z_index as f32 - z_offset).rem_euclid(view_diameter) + z_offset - view_radius;
        self.pos = Vec3 {
            x: new_x * chunk_size,
            y: new_y * chunk_size,
            z: new_z * chunk_size,
        };
    }
}
