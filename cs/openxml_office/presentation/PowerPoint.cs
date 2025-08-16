using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.presentation_2007;

namespace draviavemal.openxml_office.presentation_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Power Point presentation
    /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
    /// </summary>
    public class PowerPoint : PrivacyProperties
    {
        private readonly IntPtr ffiPowerPoint;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte power_point_create(
            [MarshalAs(UnmanagedType.LPStr)] string optional_string,
            IntPtr buffer,
            int length,
            out IntPtr powerPoint,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte power_point_save_as(
            IntPtr powerPointPtr,
            [MarshalAs(UnmanagedType.LPStr)] string file_name,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
        /// </summary>
        public PowerPoint(PowerPointProperties powerPointProperties)
        {
            FlatBufferBuilder builder = new(1024);
            PresentationPropertiesModel.StartPresentationPropertiesModel(builder);
            PresentationPropertiesModel.AddIsInMemory(builder, powerPointProperties.IsInMemory);
            Offset<PresentationPropertiesModel> powerPointPropertiesModel = PresentationPropertiesModel.EndPresentationPropertiesModel(builder);
            builder.Finish(powerPointPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = power_point_create(null, bufferIntPtr, bufferSize, out ffiPowerPoint, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
		/// </summary>
        public PowerPoint(string fileName, PowerPointProperties powerPointProperties)
        {
            FlatBufferBuilder builder = new(1024);
            PresentationPropertiesModel.StartPresentationPropertiesModel(builder);
            PresentationPropertiesModel.AddIsInMemory(builder, powerPointProperties.IsInMemory);
            Offset<PresentationPropertiesModel> powerPointPropertiesModel = PresentationPropertiesModel.EndPresentationPropertiesModel(builder);
            builder.Finish(powerPointPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = power_point_create(fileName, bufferIntPtr, bufferSize, out ffiPowerPoint, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }

        /// <summary>
        /// Even on edit file OpenXML-Office Will clone the source and work on top of it to protect the integrity of source file.
        /// You can save the document at the end of lifecycle targeting the edit file to update or new file.
        /// This is supported for both file path and data stream
        /// </summary>
        public void SaveAs(string filePath)
        {
            sbyte statusCode = power_point_save_as(ffiPowerPoint, filePath, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
        }

    }
}
