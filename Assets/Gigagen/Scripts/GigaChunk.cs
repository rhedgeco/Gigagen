using System;
using Gigagen.Extensions;
using Gigagen.Native;
using UnityEngine;

namespace Gigagen
{
    public class GigaChunk
    {
        private unsafe Native.GigaChunk* _nativePtr;

        internal GigaChunk()
        {
            unsafe
            {
                _nativePtr = (Native.GigaChunk*)UIntPtr.Zero;
            }
        }

        internal unsafe GigaChunk(Native.GigaChunk* nativePtr)
        {
            _nativePtr = nativePtr;
        }

        ~GigaChunk()
        {
            Unload();
        }

        public Mesh Mesh { get; } = new();

        public bool Completed
        {
            get
            {
                unsafe
                {
                    return (UIntPtr)_nativePtr != UIntPtr.Zero;
                }
            }
        }

        public Vector3 Position
        {
            get
            {
                unsafe
                {
                    if (!Completed) return Vector3.negativeInfinity;
                    return Func.get_chunk_pos(_nativePtr).ToVector3();
                }
            }
        }

        internal unsafe void Reset(Native.GigaChunk* nativePtr)
        {
            Unload();
            _nativePtr = nativePtr;
        }

        internal unsafe void Unload()
        {
            if (!Completed) return;
            Func.dispose_chunk(_nativePtr);
        }
    }
}
