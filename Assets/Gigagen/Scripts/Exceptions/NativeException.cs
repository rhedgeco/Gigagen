using System;
using System.Runtime.Serialization;
using JetBrains.Annotations;

namespace Gigagen.Exceptions
{
    public class NativeException : Exception
    {
        public NativeException()
        {
        }

        protected NativeException([NotNull] SerializationInfo info, StreamingContext context) : base(info, context)
        {
        }

        public NativeException(string message) : base(message)
        {
        }

        public NativeException(string message, Exception innerException) : base(message, innerException)
        {
        }
    }
}
