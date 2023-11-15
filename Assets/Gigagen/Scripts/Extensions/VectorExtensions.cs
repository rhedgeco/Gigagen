using Gigagen.Native;
using UnityEngine;

namespace Gigagen.Extensions
{
    public static class VectorExtensions
    {
        internal static NativeVec3 ToNative(in this Vector3 vector)
        {
            return new NativeVec3
            {
                x = vector.x,
                y = vector.y,
                z = vector.z
            };
        }

        internal static Vector3 ToVector3(in this NativeVec3 vector)
        {
            return new Vector3
            {
                x = vector.x,
                y = vector.y,
                z = vector.z
            };
        }
    }
}
