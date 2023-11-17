use std::{
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread,
};

use glam::Vec3;

use crate::ChunkData;

pub enum BackendCommand {
    UnloadAllChunks,
    UnloadChunk(usize),
    SetCenter(Vec3),
}

pub trait BuilderBackend: Send + 'static {
    fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        command_recv: Receiver<BackendCommand>,
        chunk_send: Sender<ChunkData>,
        max_cores: usize,
    ) -> Self;
    fn run(self);
}

pub struct WorldBuilder {
    command_send: Sender<BackendCommand>,
    chunk_recv: Receiver<ChunkData>,
}

impl WorldBuilder {
    pub fn new<T: BuilderBackend>(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        max_cores: usize,
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
            max_cores,
        );
        thread::spawn(move || backend.run());
        Self {
            command_send,
            chunk_recv,
        }
    }

    pub fn unload_all_chunks(&self) {
        if let Err(_) = self.command_send.send(BackendCommand::UnloadAllChunks) {
            eprintln!("failed send rebuild command. channel was closed.");
        }
    }

    pub fn unload_chunk(&self, index: usize) {
        if let Err(_) = self.command_send.send(BackendCommand::UnloadChunk(index)) {
            eprintln!("failed send rebuild command. channel was closed.");
        }
    }

    pub fn set_center(&self, center: Vec3) {
        if let Err(_) = self.command_send.send(BackendCommand::SetCenter(center)) {
            eprintln!("failed send center command. channel was closed.");
        }
    }

    pub fn get_completed_chunk(&self) -> Option<ChunkData> {
        match self.chunk_recv.try_recv() {
            Ok(chunk) => Some(chunk),
            Err(error) => match error {
                TryRecvError::Disconnected => {
                    eprintln!("could not receive chunks. connection closed.");
                    None
                }
                TryRecvError::Empty => None,
            },
        }
    }
}
