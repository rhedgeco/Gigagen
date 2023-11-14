use std::{mem::transmute, ptr::null};

use crate::GigaNode;

use super::GigaWorld;

#[repr(C)]
struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Into<glam::Vec3> for Vec3 {
    fn into(self) -> glam::Vec3 {
        unsafe { transmute(self) }
    }
}

impl From<glam::Vec3> for Vec3 {
    fn from(value: glam::Vec3) -> Self {
        unsafe { transmute(value) }
    }
}

#[repr(C)]
struct LoadedChunkReturn {
    valid: bool,
    index: usize,
    pos: Vec3,
    nodes: *const GigaNode,
    node_len: usize,
}

#[no_mangle]
unsafe extern "C" fn create_world(
    center: Vec3,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u16,
) -> *mut GigaWorld {
    Box::into_raw(Box::new(GigaWorld::new(
        center.into(),
        view_dist,
        chunk_size,
        chunk_div,
    )))
}

#[no_mangle]
unsafe extern "C" fn dispose_world(world: *mut GigaWorld) {
    drop(Box::from_raw(world));
}

#[no_mangle]
unsafe extern "C" fn set_world_center(world: *mut GigaWorld, center: Vec3) {
    (*world).set_center(center.into());
}

#[no_mangle]
unsafe extern "C" fn set_world_view_dist(world: *mut GigaWorld, view_dist: u8) {
    (*world).set_view_dist(view_dist);
}

#[no_mangle]
unsafe extern "C" fn set_world_chunk_size(world: *mut GigaWorld, chunk_size: f32) {
    (*world).set_chunk_size(chunk_size);
}

#[no_mangle]
unsafe extern "C" fn set_world_chunk_div(world: *mut GigaWorld, chunk_div: u16) {
    (*world).set_chunk_div(chunk_div);
}

#[no_mangle]
unsafe extern "C" fn set_world_data(
    world: *mut GigaWorld,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u16,
) {
    (*world).set_data(view_dist, chunk_size, chunk_div);
}

#[no_mangle]
unsafe extern "C" fn load_next_world_chunk(world: *mut GigaWorld) -> LoadedChunkReturn {
    match (*world).load_next_chunk() {
        Some(loaded) => LoadedChunkReturn {
            valid: true,
            index: loaded.index,
            pos: loaded.pos.into(),
            nodes: loaded.nodes.as_ptr(),
            node_len: loaded.nodes.len(),
        },
        None => LoadedChunkReturn {
            valid: false,
            index: 0,
            pos: glam::Vec3::ZERO.into(),
            nodes: null(),
            node_len: 0,
        },
    }
}
