use crate::{export_utils::vec3::NativeVec3, ChunkData};

use super::Node;

#[no_mangle]
unsafe extern "C" fn dispose_chunk(chunk_data: *mut ChunkData) {
    drop(Box::from_raw(chunk_data));
}

#[no_mangle]
unsafe extern "C" fn get_chunk_index(chunk_data: *mut ChunkData) -> usize {
    (*chunk_data).index()
}

#[no_mangle]
unsafe extern "C" fn get_chunk_pos(chunk_data: *mut ChunkData) -> NativeVec3 {
    (*chunk_data).pos().into()
}

#[no_mangle]
unsafe extern "C" fn get_chunk_size(chunk_data: *mut ChunkData) -> f32 {
    (*chunk_data).size()
}

#[no_mangle]
unsafe extern "C" fn get_chunk_div(chunk_data: *mut ChunkData) -> u8 {
    (*chunk_data).div()
}

#[repr(C)]
struct NodeSlice {
    ptr: *mut Node,
    len: usize,
}

#[no_mangle]
unsafe extern "C" fn get_chunk_nodes(chunk_data: *mut ChunkData) -> NodeSlice {
    let nodes = (*chunk_data).nodes_mut();
    NodeSlice {
        ptr: nodes.as_mut_ptr(),
        len: nodes.len(),
    }
}
