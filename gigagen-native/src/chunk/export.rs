use crate::{export_utils::vec3::NativeVec3, GigaChunk};

#[no_mangle]
unsafe extern "C" fn dispose_chunk(chunk: *mut GigaChunk) {
    drop(Box::from_raw(chunk));
}

#[no_mangle]
unsafe extern "C" fn get_chunk_pos(chunk: *mut GigaChunk) -> NativeVec3 {
    (*chunk).data().pos().into()
}

#[no_mangle]
unsafe extern "C" fn get_chunk_world_index(chunk: *mut GigaChunk) -> usize {
    (*chunk).world_index()
}
