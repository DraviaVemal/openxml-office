
using System;
using System.Runtime.InteropServices;
using draviavemal.openxml_office.global_2007;
using Google.FlatBuffers;
using openxml_office_fbs.spreadsheet;

namespace draviavemal.openxml_office.spreadsheet_2007
{

    public class Worksheet
    {
        private readonly ulong ffiWorksheet;

        internal Worksheet(ulong ffiWorksheet)
        {
            this.ffiWorksheet = ffiWorksheet;
        }

        private delegate sbyte FfiBufferCall(
            IntPtr inBuffer,
            UIntPtr inBufferSize,
            out IntPtr outBuffer,
            out UIntPtr outBufferSize,
            out IntPtr errorMsg
        );

        private delegate sbyte FfiVoidCall(
            IntPtr inBuffer,
            UIntPtr inBufferSize,
            out IntPtr errorMsg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "set_column_ref_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_column_ref_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "set_column_index_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_column_index_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "set_row_index_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_row_index_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "free_buffer", CallingConvention = CallingConvention.Cdecl)]
        private static extern void ffi_free_buffer(
            IntPtr buffer,
            UIntPtr buffer_size
        );

        public void SetColumnRefProperties(string cellRef, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsCellRef = builder.CreateString(cellRef);
            Offset<worksheet_column_properties> fbsColumnRefProperties = worksheet_column_properties.Createworksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<worksheet_set_column_ref_properties> fbsSetColumnRefProperties = worksheet_set_column_ref_properties.Createworksheet_set_column_ref_properties(builder, ffiWorksheet, fbsCellRef, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnRefProperties.Value);
            InvokeVoidFfi(ffi_set_column_ref_properties, builder);
        }

        public void SetColumnindexProperties(ushort cellIndex, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<worksheet_column_properties> fbsColumnRefProperties = worksheet_column_properties.Createworksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<worksheet_set_column_index_properties> fbsSetColumnIndexProperties = worksheet_set_column_index_properties.Createworksheet_set_column_index_properties(builder, ffiWorksheet, cellIndex, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnIndexProperties.Value);
            InvokeVoidFfi(ffi_set_column_index_properties, builder);
        }

        public void SetRowindexProperties(ushort cellIndex, RowProperties rowRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<worksheet_row_properties> fbsRowRefProperties = worksheet_row_properties.Createworksheet_row_properties(builder, rowRefProperties.Height, rowRefProperties.Hidden, rowRefProperties.TickTop, rowRefProperties.ThickBottom);
            Offset<worksheet_set_row_index_properties> fbsSetRowIndexProperties = worksheet_set_row_index_properties.Createworksheet_set_row_index_properties(builder, ffiWorksheet, cellIndex, fbsRowRefProperties);
            builder.Finish(fbsSetRowIndexProperties.Value);
            InvokeVoidFfi(ffi_set_row_index_properties, builder);
        }


        private static byte[] InvokeBufferFfi(FfiBufferCall ffiCall, FlatBufferBuilder builder)
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

        private static void InvokeVoidFfi(FfiVoidCall ffiCall, FlatBufferBuilder builder)
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