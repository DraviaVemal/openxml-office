// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;

namespace draviavemal.openxml_office.global_2007
{
    /// <summary>
    /// Internal helper that centralises the native FFI invocation plumbing shared across
    /// every OpenXML-Office component. This type is intentionally <c>internal</c> so the
    /// native buffer marshaling can be reused from any namespace inside the library while
    /// staying inaccessible to consumers of the package.
    /// </summary>
    internal static class FfiInterop
    {
        /// <summary>
        /// Delegate describing an FFI call that returns a serialized FlatBuffers payload.
        /// </summary>
        internal delegate sbyte FfiBufferCall(
            IntPtr inBuffer,
            UIntPtr inBufferSize,
            out IntPtr outBuffer,
            out UIntPtr outBufferSize,
            out IntPtr errorMsg
        );

        /// <summary>
        /// Delegate describing an FFI call that does not return a payload.
        /// </summary>
        internal delegate sbyte FfiVoidCall(
            IntPtr inBuffer,
            UIntPtr inBufferSize,
            out IntPtr errorMsg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "free_buffer", CallingConvention = CallingConvention.Cdecl)]
        private static extern void ffi_free_buffer(
            IntPtr buffer,
            UIntPtr buffer_size
        );

        /// <summary>
        /// Invokes a native FFI call that returns a serialized FlatBuffers payload and
        /// copies the response into a managed byte array before releasing the native buffer.
        /// </summary>
        internal static byte[] InvokeBufferFfi(FfiBufferCall ffiCall, FlatBufferBuilder builder)
        {
            byte[] requestBuffer = builder.SizedByteArray();
            unsafe
            {
                fixed (byte* requestPtr = requestBuffer)
                {
                    sbyte statusCode = ffiCall(
                        (IntPtr)requestPtr,
                        new UIntPtr((uint)requestBuffer.Length),
                        out IntPtr responsePtr,
                        out UIntPtr responseSize,
                        out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                    int responseLength = (int)responseSize.ToUInt64();
                    byte[] responseBuffer = new byte[responseLength];
                    if (responseLength > 0)
                    {
                        Marshal.Copy(responsePtr, responseBuffer, 0, responseLength);
                    }
                    if (responsePtr != IntPtr.Zero)
                    {
                        ffi_free_buffer(responsePtr, responseSize);
                    }
                    return responseBuffer;
                }
            }
        }

        /// <summary>
        /// Invokes a native FFI call that does not return a payload.
        /// </summary>
        internal static void InvokeVoidFfi(FfiVoidCall ffiCall, FlatBufferBuilder builder)
        {
            byte[] requestBuffer = builder.SizedByteArray();
            unsafe
            {
                fixed (byte* requestPtr = requestBuffer)
                {
                    sbyte statusCode = ffiCall(
                        (IntPtr)requestPtr,
                        new UIntPtr((uint)requestBuffer.Length),
                        out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }
    }
}
