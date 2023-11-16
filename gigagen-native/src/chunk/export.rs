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

#[repr(C)]
pub struct Vec3Array {
    vectors: *mut NativeVec3,
    len: usize,
}

#[no_mangle]
unsafe extern "C" fn get_chunk_vertices(chunk: *mut GigaChunk) -> Vec3Array {
    let vertices = &(*chunk).mesh().vertices;
    Vec3Array {
        vectors: vertices.as_ptr() as *mut NativeVec3,
        len: vertices.len(),
    }
}

#[no_mangle]
unsafe extern "C" fn get_chunk_normals(chunk: *mut GigaChunk) -> Vec3Array {
    let normals = &(*chunk).mesh().normals;
    Vec3Array {
        vectors: normals.as_ptr() as *mut NativeVec3,
        len: normals.len(),
    }
}

#[repr(C)]
pub struct IndexArray {
    vectors: *mut i32,
    len: usize,
}

#[no_mangle]
unsafe extern "C" fn get_chunk_indices(chunk: *mut GigaChunk) -> IndexArray {
    let indices = &(*chunk).mesh().indices;
    IndexArray {
        vectors: indices.as_ptr() as *mut i32,
        len: indices.len(),
    }
}
