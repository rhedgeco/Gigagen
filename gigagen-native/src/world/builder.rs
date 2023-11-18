use std::{
    sync::mpsc::{channel, Receiver, Sender, TryIter},
    thread,
};

use glam::Vec3;

use crate::ChunkData;

type ChunkIter<'a> = TryIter<'a, ChunkData>;

pub enum WorldCommand {
    SetCenter(Vec3),
}

pub trait WorldBackend: Send + 'static {
    type Settings;
    fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        command_recv: Receiver<WorldCommand>,
        chunk_send: Sender<ChunkData>,
        settings: Self::Settings,
    ) -> Self;
    fn chunk_count(&self) -> usize;
    fn run(self);
}

pub struct WorldBuilder {
    chunk_count: usize,
    command_send: Sender<WorldCommand>,
    chunk_recv: Receiver<ChunkData>,
}

impl WorldBuilder {
    pub fn new<T: WorldBackend>(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        settings: T::Settings,
    ) -> Self {
        let (command_send, command_recv) = channel();
        let (chunk_send, chunk_recv) = channel();
        let backend = T::new(
            center,
            view_dist,
            chunk_size,
            chunk_div,
            command_recv,
            chunk_send,
            settings,
        );
        let chunk_count = backend.chunk_count();
        thread::spawn(move || backend.run());
        Self {
            chunk_count,
            command_send,
            chunk_recv,
        }
    }

    pub fn chunk_count(&self) -> usize {
        self.chunk_count
    }

    pub fn set_center(&self, center: Vec3) {
        if let Err(_) = self.command_send.send(WorldCommand::SetCenter(center)) {
            eprintln!("failed send center command. channel was closed.");
        }
    }

    pub fn iter_completed_chunks(&self) -> ChunkIter {
        self.chunk_recv.try_iter()
    }
}
