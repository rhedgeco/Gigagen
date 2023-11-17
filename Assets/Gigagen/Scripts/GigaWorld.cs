using UnityEngine;

namespace Gigagen
{
    public class GigaWorld : MonoBehaviour
    {
        [SerializeField] private byte viewDistance = 8;
        [SerializeField] [Min(1f)] private float chunkSize = 32;
        [SerializeField] private byte chunkDivisor = 32;

        private WorldBuilder _worldBuilder;

        private void Awake()
        {
            _worldBuilder = WorldBuilder.CreateLocal(transform.position, viewDistance, chunkSize, chunkDivisor);
        }

        private void Update()
        {
            _worldBuilder.SetWorldCenter(transform.position);
            _worldBuilder.PullChunkUpdates();
        }

        private void OnDrawGizmosSelected()
        {
            Gizmos.color = new Color(0, 1, 0, 0.5f);
            Gizmos.DrawWireSphere(transform.position, viewDistance * chunkSize);
            _worldBuilder?.DrawChunkGizmos();
        }
    }
}
