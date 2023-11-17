using System;
using System.Collections.Generic;
using System.Linq;
using Gigagen.Extensions;
using Gigagen.Native;
using UnityEngine;
using UnityEngine.Profiling;

namespace Gigagen
{
    public class WorldBuilder
    {
        private readonly unsafe Native.WorldBuilder* _nativePtr;
        private readonly List<ChunkData> _chunkPool;

        public Vector3 Center { get; private set; }
        public readonly byte ViewDist;
        public readonly float ChunkSize;
        public readonly byte ChunkDiv;

        private unsafe WorldBuilder(Native.WorldBuilder* nativePtr, Vector3 center, byte viewDist, float chunkSize,
            byte chunkDiv)
        {
            _nativePtr = nativePtr;
            var axisLength = viewDist * 2;
            var chunkCount = axisLength * axisLength * axisLength;
            _chunkPool = Enumerable.Repeat<ChunkData>(null, chunkCount).ToList();

            Center = center;
            ViewDist = viewDist;
            ChunkSize = chunkSize;
            ChunkDiv = chunkDiv;
        }

        ~WorldBuilder()
        {
            unsafe
            {
                Func.dispose_world_builder(_nativePtr);
            }
        }

        public static WorldBuilder CreateLocal(Vector3 center, byte viewDist, float chunkSize, byte chunkDiv,
            int maxCores = int.MaxValue)
        {
            unsafe
            {
                var nativePtr = Func.create_local_world_builder(
                    center.ToNative(), viewDist, chunkSize, chunkDiv, (nuint)maxCores);
                return new WorldBuilder(nativePtr, center, viewDist, chunkSize, chunkDiv);
            }
        }

        public void UnloadAllChunks()
        {
            Profiler.BeginSample("WorldBuilder.UnloadAllChunks");
            unsafe
            {
                foreach (var chunkData in _chunkPool) chunkData?.MarkUnloaded();
                Func.unload_all_world_chunks(_nativePtr);
            }

            Profiler.EndSample();
        }

        public void SetWorldCenter(Vector3 center)
        {
            Profiler.BeginSample("WorldBuilder.SetWorldCenter");
            unsafe
            {
                // add half a chunk before unloading so there is no way for the
                // backend to disagree with what chunks should be loaded
                var unloadDistance = ChunkSize * ViewDist + ChunkSize / 2;
                foreach (var chunkData in _chunkPool)
                {
                    if (chunkData == null || Vector3.Distance(Center, chunkData.Center) < unloadDistance) continue;
                    Func.unload_world_chunk(_nativePtr, (nuint)chunkData.WorldIndex);
                    chunkData.MarkUnloaded();
                }

                Func.set_world_center(_nativePtr, center.ToNative());
                Center = center;
            }

            Profiler.EndSample();
        }

        public void PullChunkUpdates()
        {
            Profiler.BeginSample("WorldBuilder.PullChunkUpdates");
            unsafe
            {
                while (true)
                {
                    var chunkPtr = Func.get_completed_world_chunk(_nativePtr);
                    if ((UIntPtr)chunkPtr == UIntPtr.Zero) break;
                    var worldIndex = (int)Func.get_chunk_world_index(chunkPtr);
                    var gigaChunk = _chunkPool[worldIndex];
                    if (gigaChunk != null) gigaChunk.Register(chunkPtr);
                    else _chunkPool[worldIndex] = new ChunkData(chunkPtr);
                }
            }

            Profiler.EndSample();
        }

        public void DrawChunkGizmos()
        {
            Profiler.BeginSample("WorldBuilder.DrawChunkGizmos");
            var chunkSizeVector = Vector3.one * ChunkSize;
            foreach (var chunkData in _chunkPool)
            {
                if (chunkData == null) continue;
                Gizmos.color = chunkData.Loaded
                    ? new Color(1, 1, 0, 0.2f)
                    : new Color(1, 0, 0, 0.1f);
                Gizmos.DrawWireCube(chunkData.Center, chunkSizeVector);
            }

            Profiler.EndSample();
        }

        public IEnumerable<ChunkData> GetChunkData()
        {
            return _chunkPool.Where(chunkData => chunkData != null);
        }
    }
}
