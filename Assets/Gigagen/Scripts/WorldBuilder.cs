using System;
using Gigagen.Extensions;
using Gigagen.Native;
using Unity.Jobs.LowLevel.Unsafe;
using UnityEngine;
using UnityEngine.Profiling;

namespace Gigagen
{
    public class WorldBuilder
    {
        private readonly unsafe Native.WorldBuilder* _nativePtr;
        private readonly GigaChunk[] _chunkPool;

        public Vector3 Center { get; private set; }

        private unsafe WorldBuilder(Native.WorldBuilder* nativePtr)
        {
            _nativePtr = nativePtr;
            var chunkCount = (int)Func.get_world_chunk_count(_nativePtr);
            _chunkPool = new GigaChunk[chunkCount];
        }

        ~WorldBuilder()
        {
            unsafe
            {
                Func.dispose_world_builder(_nativePtr);
            }
        }

        public static WorldBuilder CreateLocal(Vector3 center, byte viewDistance, float chunkSize, byte chunkDivisor,
            int threadCount = 0)
        {
            unsafe
            {
                var maxThreads = JobsUtility.JobWorkerMaximumCount;
                var clampedThreadCount = Mathf.Clamp(threadCount, 1, maxThreads);
                var nativePtr = Func.create_local_world_builder(center.ToNative(), viewDistance, chunkSize,
                    chunkDivisor, (nuint)clampedThreadCount);
                return new WorldBuilder(nativePtr);
            }
        }

        public void SetWorldCenter(Vector3 center)
        {
            Profiler.BeginSample("WorldBuilder.SetWorldCenter");
            unsafe
            {
                Func.set_world_center(_nativePtr, center.ToNative());
                Center = center;
            }

            Profiler.EndSample();
        }

        public void PullCompletedChunks()
        {
            Profiler.BeginSample("WorldBuilder.PullCompletedChunks");
            unsafe
            {
                while (true)
                {
                    var chunkPtr = Func.get_next_completed_chunk(_nativePtr);
                    if ((UIntPtr)chunkPtr == UIntPtr.Zero) break;
                    var chunkIndex = Func.get_chunk_index(chunkPtr);
                    var currentChunk = _chunkPool[chunkIndex];
                    if (currentChunk != null) currentChunk.Load(chunkPtr);
                    else _chunkPool[chunkIndex] = new GigaChunk(chunkPtr);
                }
            }

            Profiler.EndSample();
        }

        public void DrawChunkGizmos()
        {
            Profiler.BeginSample("WorldBuilder.DrawChunkGizmos");
            foreach (var chunkData in _chunkPool)
            {
                if (chunkData == null) continue;
                Gizmos.color = chunkData.Loaded
                    ? new Color(1, 1, 0, 0.2f)
                    : new Color(1, 0, 0, 0.1f);
                var chunkSizeVector = chunkData.Size * Vector3.one;
                Gizmos.DrawWireCube(chunkData.Center, chunkSizeVector);
            }

            Profiler.EndSample();
        }
    }
}
