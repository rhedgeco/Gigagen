using System;
using System.Runtime.InteropServices;
using Gigagen.Extensions;
using Gigagen.Native;
using Unity.Collections.LowLevel.Unsafe;
using UnityEngine;
using UnityEngine.Profiling;
using UnityEngine.Rendering;

namespace Gigagen
{
    public class ChunkData
    {
        private unsafe Native.ChunkData* _nativePtr;
        private NodeSlice _nodeSlice;

        public bool Loaded { get; private set; }

        internal unsafe ChunkData(Native.ChunkData* nativePtr)
        {
            Register(nativePtr);
        }

        ~ChunkData()
        {
            unsafe
            {
                Func.dispose_chunk(_nativePtr);
            }
        }

        public int WorldIndex
        {
            get
            {
                unsafe
                {
                    return (int)Func.get_chunk_world_index(_nativePtr);
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

        internal unsafe void Register(Native.ChunkData* nativePtr)
        {
            if ((UIntPtr)_nativePtr != UIntPtr.Zero) Func.dispose_chunk(_nativePtr);
            _nativePtr = nativePtr;
            _nodeSlice = Func.get_chunk_nodes(_nativePtr);
            Loaded = true;
        }

        internal void MarkUnloaded()
        {
            Loaded = false;
        }

        [StructLayout(LayoutKind.Sequential)]
        public struct Node
        {
            public ushort weight;
        }
    }
}
