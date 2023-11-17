use std::ptr::null_mut;

use crate::{export_utils::vec3::NativeVec3, ChunkData, WorldBuilder};

use super::backends::LocalBackend;

#[no_mangle]
unsafe extern "C" fn create_local_world_builder(
    center: NativeVec3,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u8,
    max_cores: usize,
) -> *mut WorldBuilder {
    Box::into_raw(Box::new(WorldBuilder::new::<LocalBackend>(
        center.into(),
        view_dist,
        chunk_size,
        chunk_div,
        max_cores,
    )))
}

#[no_mangle]
unsafe extern "C" fn dispose_world_builder(world_builder: *mut WorldBuilder) {
    drop(Box::from_raw(world_builder))
}

#[no_mangle]
unsafe extern "C" fn unload_all_world_chunks(world_builder: *mut WorldBuilder) {
    (*world_builder).unload_all_chunks();
}

#[no_mangle]
unsafe extern "C" fn unload_world_chunk(world_builder: *mut WorldBuilder, index: usize) {
    (*world_builder).unload_chunk(index);
}

#[no_mangle]
unsafe extern "C" fn set_world_center(world_builder: *mut WorldBuilder, center: NativeVec3) {
    (*world_builder).set_center(center.into());
}

#[no_mangle]
unsafe extern "C" fn get_completed_world_chunk(world_builder: *mut WorldBuilder) -> *mut ChunkData {
    match (*world_builder).get_completed_chunk() {
        Some(chunk) => Box::into_raw(Box::new(chunk)),
        None => null_mut(),
    }
}
