use std::{
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread,
};

use glam::Vec3;

use crate::GigaChunk;

pub enum BackendCommand {
    RebuildChunks,
    SetCenter(Vec3),
    SetChunkLayout {
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
    },
}

pub trait BuilderBackend: Send + 'static {
    fn new(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
        command_recv: Receiver<BackendCommand>,
        mesh_send: Sender<GigaChunk>,
    ) -> Self;
    fn run(self);
}

pub struct WorldBuilder {
    command_send: Sender<BackendCommand>,
    mesh_recv: Receiver<GigaChunk>,
}

impl WorldBuilder {
    pub fn new<T: BuilderBackend>(
        center: Vec3,
        view_dist: u8,
        chunk_size: f32,
        chunk_div: u8,
    ) -> Self {
        let (command_send, command_recv) = channel();
        let (mesh_send, mesh_recv) = channel();
        let backend = T::new(
            center,
            view_dist,
            chunk_size,
            chunk_div,
            command_recv,
            mesh_send,
        );
        thread::spawn(move || backend.run());
        Self {
            command_send,
            mesh_recv,
        }
    }

    pub fn rebuild_chunks(&self) {
        if let Err(_) = self.command_send.send(BackendCommand::RebuildChunks) {
            eprintln!("failed send rebuild command. channel was closed.");
        }
    }

    pub fn set_center(&self, center: Vec3) {
        if let Err(_) = self.command_send.send(BackendCommand::SetCenter(center)) {
            eprintln!("failed send center command. channel was closed.");
        }
    }

    pub fn set_chunk_layout(&self, view_dist: u8, chunk_size: f32, chunk_div: u8) {
        if let Err(_) = self.command_send.send(BackendCommand::SetChunkLayout {
            view_dist,
            chunk_size,
            chunk_div,
        }) {
            eprintln!("failed send chunk layout command. channel was closed.");
        }
    }

    pub fn get_completed_chunk(&self) -> Option<GigaChunk> {
        match self.mesh_recv.try_recv() {
            Ok(mesh) => Some(mesh),
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
