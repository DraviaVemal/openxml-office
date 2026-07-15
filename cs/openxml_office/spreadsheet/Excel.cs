using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.spreadsheet;

namespace draviavemal.openxml_office.spreadsheet_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Excel spreadsheets.
    /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
    /// </summary>
    public class Excel : PrivacyProperties
    {
        private readonly ulong ffiExcelPtr;

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
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_create", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_create(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_add_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_add_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_get_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_get_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_rename_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_rename_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_save_as", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_save_as(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
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

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
        /// </summary>
        public Excel(ExcelProperties excelProperties = null)
        {
            ffiExcelPtr = CreateExcel(null, excelProperties);
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
		/// </summary>
        public Excel(string fileName, ExcelProperties excelProperties = null)
        {
            ffiExcelPtr = CreateExcel(fileName, excelProperties);
        }

        private static ulong CreateExcel(string fileName, ExcelProperties excelProperties)
        {
            if (excelProperties == null)
            {
                excelProperties = new ExcelProperties();
            }
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsFileName = fileName != null ? builder.CreateString(fileName) : default;
            Offset<excel_settings> fbsExcelSettings = excel_settings.Createexcel_settings(builder, true);
            Offset<excel_create> fbsExcelCreate = excel_create.Createexcel_create(builder, fbsFileName, fbsExcelSettings);
            builder.Finish(fbsExcelCreate.Value);
            byte[] responseBuffer = InvokeBufferFfi(ffi_excel_create, builder);
            excel_create_return response = excel_create_return.GetRootAsexcel_create_return(new ByteBuffer(responseBuffer));
            return response.ExcelPtr;
        }

        /// <summary>
        /// 
        /// </summary>
        /// <returns></returns>
        public Worksheet AddSheet()
        {
            return AddSheet(null);
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="sheetName"></param>
        /// <returns></returns>
        public Worksheet AddSheet(string sheetName)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset sheetNameOffset = sheetName != null ? builder.CreateString(sheetName) : default;
            Offset<excel_add_sheet> addSheetOffset = excel_add_sheet.Createexcel_add_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(addSheetOffset.Value);
            byte[] responseBuffer = InvokeBufferFfi(ffi_excel_add_sheet, builder);
            excel_add_sheet_return response = excel_add_sheet_return.GetRootAsexcel_add_sheet_return(new ByteBuffer(responseBuffer));
            return new Worksheet(response.WorksheetPtr);
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="sheetName"></param>
        /// <returns></returns>
        public Worksheet GetWorksheet(string sheetName)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset sheetNameOffset = builder.CreateString(sheetName);
            Offset<excel_get_sheet> getSheetOffset = excel_get_sheet.Createexcel_get_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(getSheetOffset.Value);
            byte[] responseBuffer = InvokeBufferFfi(ffi_excel_get_sheet, builder);
            excel_get_sheet_return response = excel_get_sheet_return.GetRootAsexcel_get_sheet_return(new ByteBuffer(responseBuffer));
            return new Worksheet(response.WorksheetPtr);
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="oldSheetName"></param>
        /// <param name="newSheetName"></param>
        public bool RenameSheet(string oldSheetName, string newSheetName)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset oldSheetNameOffset = builder.CreateString(oldSheetName);
            StringOffset newSheetNameOffset = builder.CreateString(newSheetName);
            Offset<excel_rename_sheet> renameSheetOffset = excel_rename_sheet.Createexcel_rename_sheet(builder, ffiExcelPtr, oldSheetNameOffset, newSheetNameOffset);
            builder.Finish(renameSheetOffset.Value);
            InvokeVoidFfi(ffi_excel_rename_sheet, builder);
            return true;
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
            Offset<excel_save_as> saveAsOffset = excel_save_as.Createexcel_save_as(builder, ffiExcelPtr, filePathOffset);
            builder.Finish(saveAsOffset.Value);
            InvokeBufferFfi(ffi_excel_save_as, builder);
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
