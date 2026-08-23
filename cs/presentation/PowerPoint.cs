using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.presentation;

namespace draviavemal.openxml_office.presentation_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Power Point presentation
    /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
    /// </summary>
    public class PowerPoint : PrivacyProperties
    {
        private readonly ulong ffiPowerPointPtr;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Presentation_create", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_presentation_create(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Presentation_save_as", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_presentation_save_as(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
        /// </summary>
        public PowerPoint(PowerPointProperties powerPointProperties = null)
        {
            ffiPowerPointPtr = CreatePowerPoint(null, powerPointProperties);
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
		/// </summary>
        public PowerPoint(string fileName, PowerPointProperties powerPointProperties = null)
        {
            ffiPowerPointPtr = CreatePowerPoint(fileName, powerPointProperties);
        }

        private static ulong CreatePowerPoint(string fileName, PowerPointProperties powerPointProperties)
        {
            if (powerPointProperties == null)
            {
                powerPointProperties = new PowerPointProperties();
            }
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsFileName = fileName != null ? builder.CreateString(fileName) : default;
            Offset<Power_point_settings> fbsPowerPointSettings = Power_point_settings.CreatePower_point_settings(builder, true);
            Offset<Power_point_create> fbsPowerPointCreate = Power_point_create.CreatePower_point_create(builder, fbsFileName, fbsPowerPointSettings);
            builder.Finish(fbsPowerPointCreate.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_presentation_create, builder);
            Power_point_create_return response = Power_point_create_return.GetRootAsPower_point_create_return(new ByteBuffer(responseBuffer));
            return response.PowerPointPtr;
        }

        /// <summary>
        /// Even on edit file OpenXML-Office Will clone the source and work on top of it to protect the integrity of source file.
        /// You can save the document at the end of lifecycle targeting the edit file to update or new file.
        /// This is supported for both file path and data stream
        /// </summary>
        public void SaveAs(string filePath)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset filePathOffset = builder.CreateString(filePath);
            Offset<Power_point_save_as> saveAsOffset = Power_point_save_as.CreatePower_point_save_as(builder, ffiPowerPointPtr, filePathOffset);
            builder.Finish(saveAsOffset.Value);
            FfiInterop.InvokeBufferFfi(ffi_presentation_save_as, builder);
        }

    }
}
