using System;
using System.Runtime.InteropServices;
using Gigagen.Extensions;
using Gigagen.Native;
using UnityEngine;

namespace Gigagen
{
    public class GigaChunk
    {
        private unsafe ChunkData* _nativePtr;
        private NodeSlice _nodeSlice;

        internal unsafe GigaChunk(ChunkData* nativePtr)
        {
            Load(nativePtr);
        }

        ~GigaChunk()
        {
            unsafe
            {
                Func.dispose_chunk(_nativePtr);
            }
        }

        public bool Loaded { get; private set; }

        public int Index
        {
            get
            {
                unsafe
                {
                    return (int)Func.get_chunk_index(_nativePtr);
                }
            }
        }

        public Vector3 Position
        {
            get
            {
                unsafe
                {
                    return Func.get_chunk_pos(_nativePtr).ToVector3();
                }
            }
        }

        public Vector3 Center => Position + Vector3.one * Size / 2;

        public float Size
        {
            get
            {
                unsafe
                {
                    return Func.get_chunk_size(_nativePtr);
                }
            }
        }

        public byte Divisions
        {
            get
            {
                unsafe
                {
                    return Func.get_chunk_div(_nativePtr);
                }
            }
        }

        public Node this[int index]
        {
            get
            {
                unsafe
                {
                    return *(Node*)((long)_nodeSlice.ptr + index * sizeof(Node));
                }
            }
        }

        internal unsafe void Load(ChunkData* nativePtr)
        {
            if ((UIntPtr)_nativePtr != UIntPtr.Zero) Func.dispose_chunk(_nativePtr);
            _nativePtr = nativePtr;
            _nodeSlice = Func.get_chunk_nodes(_nativePtr);
            Loaded = true;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Node
        {
            public ushort weight;
        }
    }
}
