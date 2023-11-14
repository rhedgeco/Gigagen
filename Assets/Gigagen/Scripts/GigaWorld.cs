using System;
using Gigagen.Native;
using Unity.Collections.LowLevel.Unsafe;
using UnityEngine;
using UnityEngine.Profiling;

namespace Gigagen
{
    public class GigaWorld : MonoBehaviour
    {
        [SerializeField] private byte viewDistance = 8;
        [SerializeField] [Min(1f)] private float chunkSize = 32;
        [SerializeField] private ushort chunkDivisor = 32;
        [SerializeField] [Min(1)] private int chunkLoadsPerFrame = 8;

        private unsafe Native.GigaWorld* _nativePtr;
        private Vector3 _currentWorldCenter;
        private byte _currentViewDistance;
        private float _currentChunkSize;
        private ushort _currentChunkDiv;
        private Vector3[] _chunkPool;

        private void Awake()
        {
            _currentWorldCenter = transform.position;
            _currentViewDistance = viewDistance;
            _currentChunkSize = chunkSize;
            _currentChunkDiv = chunkDivisor;
            ReloadChunkPool();
            unsafe
            {
                var nativeWorldCenter = UnsafeUtility.As<Vector3, Vec3>(ref _currentWorldCenter);
                _nativePtr = Func.create_world(nativeWorldCenter, _currentViewDistance,
                    _currentChunkSize, _currentChunkDiv);
            }
        }

        private void Update()
        {
            ReloadChunkData();
            ReloadWorldCenter();
            LoadMoreChunks();
        }

        private void ReloadWorldCenter()
        {
            if (Vector3.Distance(transform.position, _currentWorldCenter) < 1) return;

            Profiler.BeginSample("Move World Center");
            _currentWorldCenter = transform.position;
            unsafe
            {
                var nativeWorldCenter = UnsafeUtility.As<Vector3, Vec3>(ref _currentWorldCenter);
                Func.set_world_center(_nativePtr, nativeWorldCenter);
            }

            Profiler.EndSample();
        }

        private void ReloadChunkData()
        {
            if (_currentViewDistance == viewDistance && Math.Abs(_currentChunkSize - chunkSize) < 0.1 &&
                _currentChunkDiv == chunkDivisor) return;
            Profiler.BeginSample("Reload Chunk Data");
            _currentViewDistance = viewDistance;
            _currentChunkSize = chunkSize;
            _currentChunkDiv = chunkDivisor;
            ReloadChunkPool();
            unsafe
            {
                Func.set_world_data(_nativePtr, _currentViewDistance, _currentChunkSize, _currentChunkDiv);
            }

            Profiler.EndSample();
        }

        private void LoadMoreChunks()
        {
            Profiler.BeginSample("Load Next Chunk");
            for (var i = 0; i < chunkLoadsPerFrame; i++)
                unsafe
                {
                    var chunkData = Func.load_next_world_chunk(_nativePtr);
                    if (!chunkData.valid) return;
                    var chunkPos = UnsafeUtility.As<Vec3, Vector3>(ref chunkData.pos);
                    _chunkPool[chunkData.index] = chunkPos;
                }

            Profiler.EndSample();
        }

        private void ReloadChunkPool()
        {
            var axisLength = _currentViewDistance * 2;
            _chunkPool = new Vector3[axisLength * axisLength * axisLength];
        }

        private void OnDrawGizmosSelected()
        {
            Gizmos.color = new Color(0, 1, 0, 0.5f);
            Gizmos.DrawWireCube(transform.position, Vector3.one * viewDistance * 2 * chunkSize);

            if (_chunkPool == null) return;
            var chunkSizeVector = Vector3.one * _currentChunkSize;
            var halfChunkSize = Vector3.one * _currentChunkSize / 2;
            Gizmos.color = new Color(1, 1, 0, 0.2f);
            foreach (var chunkPos in _chunkPool)
            {
                if (chunkPos == Vector3.zero) continue;
                var chunkCenter = chunkPos + halfChunkSize;
                Gizmos.DrawWireCube(chunkCenter, chunkSizeVector);
            }

            Gizmos.DrawWireCube(transform.position, Vector3.one * viewDistance * 2 * chunkSize);
        }

        private void OnDestroy()
        {
            unsafe
            {
                Func.dispose_world(_nativePtr);
            }
        }
    }
}
