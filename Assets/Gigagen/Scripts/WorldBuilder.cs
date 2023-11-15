using System;
using System.Collections.Generic;
using System.Linq;
using Gigagen.Extensions;
using Gigagen.Native;
using UnityEngine;

namespace Gigagen
{
    public class WorldBuilder
    {
        private readonly unsafe Native.WorldBuilder* _nativePtr;
        private readonly List<GigaChunk> _chunkPool;

        private unsafe WorldBuilder(Native.WorldBuilder* nativePtr, int chunkCount)
        {
            _nativePtr = nativePtr;
            _chunkPool = Enumerable.Repeat<GigaChunk>(null, chunkCount).ToList();
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
                var axisLength = viewDist * 2;
                var chunkCount = axisLength * axisLength * axisLength;
                var nativePtr = Func.create_local_world_builder(center.ToNative(), viewDist, chunkSize, chunkDiv);
                return new WorldBuilder(nativePtr, chunkCount);
            }
        }

        public void RebuildChunks()
        {
            foreach (var chunk in _chunkPool)
            {
                if (chunk == null) continue;
                chunk.Unload();
            }

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

            var axisLength = viewDist * 2;
            var chunkCount = axisLength * axisLength * axisLength;
            while (_chunkPool.Count > chunkCount) _chunkPool.RemoveAt(_chunkPool.Count - 1);
            while (_chunkPool.Count < chunkCount) _chunkPool.Add(null);
            RebuildChunks();
        }

        public void LoadPendingChunks(int max = 0)
        {
            unsafe
            {
                var count = 0;
                while (true)
                {
                    var chunkPtr = Func.get_completed_world_chunk(_nativePtr);
                    if ((UIntPtr)chunkPtr == UIntPtr.Zero) break;
                    var worldIndex = (int)Func.get_chunk_world_index(chunkPtr);
                    var gigaChunk = _chunkPool[worldIndex];
                    if (gigaChunk != null) gigaChunk.Reset(chunkPtr);
                    else _chunkPool[worldIndex] = new GigaChunk(chunkPtr);
                    if (++count >= max && max > 0) break;
                }
            }
        }

        public IEnumerable<GigaChunk> GetCompletedChunks()
        {
            foreach (var chunk in _chunkPool)
            {
                if (chunk is not { Completed: true }) continue;
                yield return chunk;
            }
        }
    }
}
