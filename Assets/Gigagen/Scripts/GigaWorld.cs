using System;
using UnityEngine;

namespace Gigagen
{
    public class GigaWorld : MonoBehaviour
    {
        [SerializeField] private byte viewDistance = 8;
        [SerializeField] [Min(1f)] private float chunkSize = 32;
        [SerializeField] private byte chunkDivisor = 32;
        [SerializeField] private int maxChunkLoadsPerFrame;

        private WorldBuilder _worldBuilder;
        private Vector3 _currentWorldCenter;
        private byte _currentViewDistance;
        private float _currentChunkSize;
        private byte _currentChunkDiv;

        private void Awake()
        {
            _currentWorldCenter = transform.position;
            _currentViewDistance = viewDistance;
            _currentChunkSize = chunkSize;
            _currentChunkDiv = chunkDivisor;
            _worldBuilder = WorldBuilder.CreateLocal(_currentWorldCenter, _currentViewDistance,
                _currentChunkSize, _currentChunkDiv);
        }

        private void Update()
        {
            TryReloadChunkLayout();
            TryReloadWorldCenter();
            _worldBuilder.LoadPendingChunks(maxChunkLoadsPerFrame);
        }

        private void TryReloadWorldCenter()
        {
            var newPosition = transform.position;
            if (Vector3.Distance(newPosition, _currentWorldCenter) < 1) return;
            _currentWorldCenter = newPosition;
            _worldBuilder.SetCenter(newPosition);
        }

        private void TryReloadChunkLayout()
        {
            if (viewDistance == _currentViewDistance &&
                chunkDivisor == _currentChunkDiv &&
                Math.Abs(chunkSize - _currentChunkSize) < 0.001) return;
            _currentViewDistance = viewDistance;
            _currentChunkSize = chunkSize;
            _currentChunkDiv = chunkDivisor;
            _worldBuilder.SetChunkLayout(_currentViewDistance, _currentChunkSize, _currentChunkDiv);
        }

        private void OnDrawGizmosSelected()
        {
            Gizmos.color = new Color(0, 1, 0, 0.5f);
            Gizmos.DrawWireCube(transform.position, Vector3.one * viewDistance * 2 * chunkSize);

            if (_worldBuilder == null) return;
            var chunkSizeVector = Vector3.one * _currentChunkSize;
            var halfChunkSize = Vector3.one * _currentChunkSize / 2;
            var maxDistance = _currentViewDistance * _currentChunkSize;
            foreach (var chunk in _worldBuilder.GetCompletedChunks())
            {
                var chunkCenter = chunk.Position + halfChunkSize;

                // set color to red if chunk is out of range
                Gizmos.color = Vector3.Distance(chunkCenter, _currentWorldCenter) < maxDistance
                    ? new Color(1, 1, 0, 0.2f)
                    : new Color(1, 0, 0, 0.1f);

                Gizmos.DrawWireCube(chunkCenter, chunkSizeVector);
            }

            Gizmos.DrawWireCube(transform.position, Vector3.one * viewDistance * 2 * chunkSize);
        }
    }
}
