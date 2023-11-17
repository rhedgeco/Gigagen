use std::sync::mpsc::{Receiver, Sender, TryRecvError};

use glam::Vec3;
use rayon::{prelude::*, ThreadPoolBuilder};

use crate::{
    chunk::samplers::PerlinSampler,
    world::{BackendCommand, BuilderBackend},
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

pub struct LocalBackend {
    center: Vec3,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u8,
    loaded: Vec<ChunkLocator>,
    unloaded: Vec<ChunkLocator>,
    command_recv: Receiver<BackendCommand>,
    mesh_send: Sender<ChunkData>,
    max_cores: usize,
}

impl BuilderBackend for LocalBackend {
    fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        command_recv: Receiver<BackendCommand>,
        mesh_send: Sender<ChunkData>,
        max_cores: usize,
    ) -> Self {
        let mut backend = Self {
            center,
            view_dist,
            chunk_size,
            chunk_div,
            loaded: Vec::new(),
            unloaded: Vec::new(),
            command_recv,
            mesh_send,
            max_cores: max_cores.max(1),
        };
        backend.rebuild_chunk_vecs();
        backend
    }

    fn run(mut self) {
        let sampler = PerlinSampler::new();
        let chunk_size = self.chunk_size;
        let chunk_div = self.chunk_div;
        let mesh_send = self.mesh_send.clone();
        let max_threads = num_cpus::get().saturating_sub(4).clamp(1, self.max_cores);
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(max_threads)
            .build()
            .unwrap();
        loop {
            let backend_iter = LocalBackendIter { backend: &mut self };
            thread_pool.install(|| {
                backend_iter.par_bridge().for_each(|chunk_pos| {
                    let chunk_data = ChunkData::new(
                        chunk_pos.chunk_index,
                        chunk_pos.pos,
                        chunk_size,
                        chunk_div,
                        &sampler,
                    );
                    if let Err(_) = mesh_send.send(chunk_data) {
                        println!("failed to send completed mesh. channel is closed.");
                    }
                })
            });

            match self.command_recv.recv() {
                Ok(command) => self.process_command(command),
                Err(_) => {
                    println!("terminating builder backend. channel is closed.");
                    break;
                }
            }
        }
    }
}

impl LocalBackend {
    fn process_command(&mut self, command: BackendCommand) {
        match command {
            BackendCommand::UnloadAllChunks => self.rebuild_chunk_vecs(),
            BackendCommand::UnloadChunk(index) => self.unload_chunk(index),
            BackendCommand::SetCenter(center) => self.set_center(center),
        }
    }

    fn unload_chunk(&mut self, index: usize) {
        for i in 0..self.loaded.len() {
            if self.loaded[i].chunk_index == index {
                let chunk = self.loaded.swap_remove(i);
                self.unloaded.push(chunk);
                break;
            }
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
        let half_chunk = self.chunk_size / 2.;
        let max_dist = self.view_dist as f32 * self.chunk_size;
        for index in (0..self.loaded.len() - 1).rev() {
            let chunk = &self.loaded[index];
            if (chunk.pos.x + half_chunk - self.center.x).abs() > max_dist
                || (chunk.pos.y + half_chunk - self.center.y).abs() > max_dist
                || (chunk.pos.z + half_chunk - self.center.z).abs() > max_dist
            {
                let mut chunk = self.loaded.swap_remove(index);
                chunk.rebuild_pos(self.center, self.view_dist, self.chunk_size);
                self.unloaded.push(chunk)
            }
        }
    }

    fn rebuild_chunk_vecs(&mut self) {
        let axis_size = self.view_dist as usize * 2;
        self.loaded = Vec::with_capacity(axis_size * axis_size * axis_size);
        self.unloaded = Vec::with_capacity(axis_size * axis_size * axis_size);
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
}

struct LocalBackendIter<'a> {
    backend: &'a mut LocalBackend,
}

impl<'a> Iterator for LocalBackendIter<'a> {
    type Item = ChunkLocator;

    fn next(&mut self) -> Option<Self::Item> {
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

        // find the closest chunk to the center of the world
        let mut next_index = None;
        let mut min_dist = f32::MAX;
        let max_dist = self.backend.view_dist as f32 * self.backend.chunk_size;
        let half_chunk = Vec3::ONE * self.backend.chunk_size / 2.;
        for (index, chunk_data) in self.backend.unloaded.iter().enumerate() {
            let chunk_dist = (chunk_data.pos + half_chunk).distance(self.backend.center);
            if chunk_dist > max_dist {
                continue;
            }
            if chunk_dist < min_dist {
                next_index = Some(index);
                min_dist = chunk_dist;
            }
        }

        // move chunk from unloaded to loaded and return for processing
        let chunk_pos = self.backend.unloaded.swap_remove(next_index?);
        self.backend.loaded.push(chunk_pos);
        Some(chunk_pos)
    }
}
