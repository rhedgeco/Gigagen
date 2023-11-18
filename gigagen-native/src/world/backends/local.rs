use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, Sender, TryRecvError},
    Arc,
};

use glam::Vec3;
use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    ThreadPoolBuilder,
};

use crate::{
    chunk::samplers::PerlinSampler,
    world::builder::{WorldBackend, WorldCommand},
    ChunkData,
};

#[derive(Clone, Copy)]
struct ChunkLocator {
    pub chunk_index: usize,
    pub x_index: usize,
    pub y_index: usize,
    pub z_index: usize,
    pub pos: Vec3,
}

impl ChunkLocator {
    pub fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        x_index: usize,
        y_index: usize,
        z_index: usize,
    ) -> Self {
        let axis_length = view_dist as usize * 2;
        let chunk_index = x_index + y_index * axis_length + z_index * axis_length * axis_length;
        let mut data = Self {
            chunk_index,
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

pub struct LocalWorldSettings {
    pub thread_count: usize,
}

pub struct LocalWorldBackend {
    center: Vec3,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u8,
    chunk_count: usize,
    loaded: Vec<ChunkLocator>,
    unloaded: Vec<ChunkLocator>,
    command_recv: Receiver<WorldCommand>,
    chunk_send: Sender<ChunkData>,
    settings: LocalWorldSettings,
}

impl WorldBackend for LocalWorldBackend {
    type Settings = LocalWorldSettings;

    fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        command_recv: Receiver<WorldCommand>,
        chunk_send: Sender<ChunkData>,
        settings: LocalWorldSettings,
    ) -> Self {
        let mut new = Self {
            center,
            view_dist,
            chunk_size,
            chunk_div,
            chunk_count: 0,
            loaded: Vec::new(),
            unloaded: Vec::new(),
            command_recv,
            chunk_send,
            settings,
        };
        new.initialize();
        new
    }

    fn chunk_count(&self) -> usize {
        self.chunk_count
    }

    fn run(mut self) {
        let sampler = PerlinSampler::new();
        let chunk_size = self.chunk_size;
        let chunk_div = self.chunk_div;
        let mesh_send = self.chunk_send.clone();
        let closed_flag = Arc::new(AtomicBool::new(false));
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(self.settings.thread_count)
            .build()
            .unwrap();

        loop {
            thread_pool.install(|| {
                UnloadedChunkIter::new(&mut self)
                    .par_bridge()
                    .for_each(|chunk| {
                        if closed_flag.load(Ordering::Relaxed) {
                            return;
                        }

                        let chunk_data = ChunkData::new(
                            chunk.chunk_index,
                            chunk.pos,
                            chunk_size,
                            chunk_div,
                            &sampler,
                        );

                        if let Err(_) = mesh_send.send(chunk_data) {
                            println!("failed to send completed mesh. channel is closed");
                            closed_flag.store(true, Ordering::Relaxed);
                        }
                    })
            });

            if closed_flag.load(Ordering::Relaxed) {
                break;
            }

            match self.command_recv.recv() {
                Ok(command) => self.process_command(command),
                Err(_) => break,
            }
        }

        println!("terminating local world backend. channel is closed.");
    }
}

impl LocalWorldBackend {
    fn initialize(&mut self) {
        let axis_size = self.view_dist as usize * 2;
        self.chunk_count = axis_size * axis_size * axis_size;
        self.loaded = Vec::with_capacity(self.chunk_count);
        self.unloaded = Vec::with_capacity(self.chunk_count);
        for x_index in 0..axis_size {
            for y_index in 0..axis_size {
                for z_index in 0..axis_size {
                    self.unloaded.push(ChunkLocator::new(
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

    fn process_command(&mut self, command: WorldCommand) {
        match command {
            WorldCommand::SetCenter(center) => self.set_center(center),
        }
    }

    fn set_center(&mut self, center: Vec3) {
        self.center = center;

        // rebuild the position for any unloaded chunks
        for chunk_data in self.unloaded.iter_mut() {
            chunk_data.rebuild_pos(self.center, self.view_dist, self.chunk_size);
        }

        // rebuild and unload the position for already loaded chunks
        // iterate backwards so that removing chunks does not mess with iteration
        let half_chunk_size = self.chunk_size / 2.;
        let half_chunk_vec = Vec3::ONE * half_chunk_size;
        let max_dist = self.view_dist as f32 * self.chunk_size + half_chunk_size;
        for index in (0..self.loaded.len() - 1).rev() {
            let chunk = &self.loaded[index];
            if (chunk.pos + half_chunk_vec).distance(self.center) > max_dist {
                let mut chunk = self.loaded.swap_remove(index);
                chunk.rebuild_pos(self.center, self.view_dist, self.chunk_size);
                self.unloaded.push(chunk);
            }
        }
    }
}

struct UnloadedChunkIter<'a> {
    backend: &'a mut LocalWorldBackend,
}

impl<'a> UnloadedChunkIter<'a> {
    pub fn new(backend: &'a mut LocalWorldBackend) -> Self {
        Self { backend }
    }
}

impl Iterator for UnloadedChunkIter<'_> {
    type Item = ChunkLocator;

    fn next(&mut self) -> Option<Self::Item> {
        // if there are no unloaded chunks, early return None
        if self.backend.unloaded.is_empty() {
            return None;
        }

        // process all commands
        // return none if the receive pipeline is disconnected
        // break and continue processing if there are no more commands
        loop {
            match self.backend.command_recv.try_recv() {
                Ok(command) => self.backend.process_command(command),
                Err(error) => match error {
                    TryRecvError::Disconnected => return None,
                    TryRecvError::Empty => break,
                },
            }
        }

        // find next unloaded chunk closest to the center of the world
        let mut min_index = None;
        let mut min_dist = f32::MAX;
        let half_chunk_vec = Vec3::ONE * self.backend.chunk_size / 2.;
        let max_dist = self.backend.view_dist as f32 * self.backend.chunk_size;
        for (index, chunk) in self.backend.unloaded.iter().enumerate() {
            let chunk_dist = (chunk.pos + half_chunk_vec).distance(self.backend.center);
            if chunk_dist > max_dist {
                continue;
            } else if chunk_dist < min_dist {
                min_index = Some(index);
                min_dist = chunk_dist;
            }
        }

        // if there was not a chunk found within the max range early return None
        // otherwise place the chunk in the loaded vec and return
        let chunk = self.backend.unloaded.swap_remove(min_index?);
        self.backend.loaded.push(chunk);
        Some(chunk)
    }
}
