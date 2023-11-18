using Unity.Jobs.LowLevel.Unsafe;
using UnityEngine;

namespace Gigagen
{
    public class GigaWorld : MonoBehaviour
    {
        [SerializeField] private Transform targetCenter;
        [SerializeField] private byte viewDistance = 8;
        [SerializeField] [Min(1f)] private float chunkSize = 32;
        [SerializeField] private byte chunkDivisor = 32;

        private WorldBuilder _worldBuilder;

        private void Awake()
        {
            var threadCount = JobsUtility.JobWorkerMaximumCount - 2;
            var initialCenter = targetCenter ? targetCenter.position : Vector3.zero;
            _worldBuilder = WorldBuilder.CreateLocal(initialCenter, viewDistance, chunkSize, chunkDivisor, threadCount);
        }

        private void Update()
        {
            if (targetCenter) _worldBuilder.SetWorldCenter(targetCenter.position);
            _worldBuilder.PullCompletedChunks();
        }

        private void OnDrawGizmosSelected()
        {
            Gizmos.color = new Color(0, 1, 0, 0.5f);
            var worldCenter = targetCenter ? targetCenter.position : Vector3.zero;
            Gizmos.DrawWireSphere(worldCenter, viewDistance * chunkSize);
            _worldBuilder?.DrawChunkGizmos();
        }
    }
}
