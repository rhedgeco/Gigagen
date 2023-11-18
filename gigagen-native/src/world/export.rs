use std::ptr::null_mut;

use crate::{export_utils::vec3::NativeVec3, ChunkData};

use super::{
    backends::local::{LocalWorldBackend, LocalWorldSettings},
    WorldBuilder,
};

#[no_mangle]
unsafe extern "C" fn create_local_world_builder(
    center: NativeVec3,
    view_dist: u8,
    chunk_size: f32,
    chunk_div: u8,
    thread_count: usize,
) -> *mut WorldBuilder {
    Box::into_raw(Box::new(WorldBuilder::new::<LocalWorldBackend>(
        center.into(),
        view_dist,
        chunk_size,
        chunk_div,
        LocalWorldSettings { thread_count },
    )))
}

#[no_mangle]
unsafe extern "C" fn dispose_world_builder(world: *mut WorldBuilder) {
    drop(Box::from_raw(world));
}

#[no_mangle]
unsafe extern "C" fn get_world_chunk_count(world: *mut WorldBuilder) -> usize {
    (*world).chunk_count()
}

#[no_mangle]
unsafe extern "C" fn set_world_center(world: *mut WorldBuilder, center: NativeVec3) {
    (*world).set_center(center.into());
}

#[no_mangle]
unsafe extern "C" fn get_next_completed_chunk(world: *mut WorldBuilder) -> *mut ChunkData {
    match (*world).iter_completed_chunks().next() {
        Some(chunk_data) => Box::into_raw(Box::new(chunk_data)),
        None => null_mut(),
    }
}
