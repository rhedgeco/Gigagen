using Gigagen.Extensions;
using Gigagen.Native;
using UnityEngine;

namespace Gigagen
{
    public class WorldBuilder
    {
        private readonly unsafe Native.WorldBuilder* _nativePtr;

        private unsafe WorldBuilder(Native.WorldBuilder* nativePtr)
        {
            _nativePtr = nativePtr;
        }

        ~WorldBuilder()
        {
            unsafe
            {
                Func.dispose_world_builder(_nativePtr);
            }
        }

        public static WorldBuilder CreateLocal(Vector3 center, byte viewDist, float chunkSize, byte chunkDiv)
        {
            unsafe
            {
                var nativePtr = Func.create_local_world_builder(center.ToNative(), viewDist, chunkSize, chunkDiv);
                return new WorldBuilder(nativePtr);
            }
        }

        public void RebuildChunks()
        {
            unsafe
            {
                Func.rebuild_world_chunks(_nativePtr);
            }
        }

        public void SetCenter(Vector3 center)
        {
            unsafe
            {
                Func.set_world_center(_nativePtr, center.ToNative());
            }
        }

        public void SetChunkLayout(byte viewDist, float chunkSize, byte chunkDiv)
        {
            unsafe
            {
                Func.set_world_chunk_layout(_nativePtr, viewDist, chunkSize, chunkDiv);
            }
        }

        public bool GetCompletedMesh(out GigaMesh mesh)
        {
            unsafe
            {
                var gigaMeshOption = Func.get_completed_world_chunk(_nativePtr);
                if (!gigaMeshOption.valid)
                {
                    mesh = new GigaMesh();
                    return false;
                }

                mesh = new GigaMesh(gigaMeshOption.chunk_index, gigaMeshOption.chunk_pos.ToVector3());
                return true;
            }
        }
    }
}
