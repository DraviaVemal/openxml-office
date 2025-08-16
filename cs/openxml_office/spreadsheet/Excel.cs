using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.spreadsheet_2007;

namespace draviavemal.openxml_office.spreadsheet_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Excel spreadsheets.
    /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
    /// </summary>
    public class Excel : PrivacyProperties
    {
        private readonly IntPtr ffiExcel;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte excel_create(
            [MarshalAs(UnmanagedType.LPStr)] string optional_file_path,
            IntPtr buffer,
            int length,
            out IntPtr excel,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte excel_add_sheet(
            IntPtr excelPtr,
            [MarshalAs(UnmanagedType.LPStr)] string optional_sheet_name,
            out IntPtr worksheet,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte excel_get_sheet(
            IntPtr excelPtr,
            [MarshalAs(UnmanagedType.LPStr)] string sheet_name,
            out IntPtr worksheet,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte excel_rename_sheet(
            IntPtr excelPtr,
            [MarshalAs(UnmanagedType.LPStr)] string old_sheet_name,
            [MarshalAs(UnmanagedType.LPStr)] string new_sheet_name,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", CallingConvention = CallingConvention.Cdecl)]
        public static extern sbyte excel_save_as(
            IntPtr excelPtr,
            [MarshalAs(UnmanagedType.LPStr)] string file_name,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
        /// </summary>
        public Excel(ExcelProperties excelProperties = null)
        {
            if (excelProperties == null)
            {
                excelProperties = new ExcelProperties();
            }
            FlatBufferBuilder builder = new(1024);
            ExcelPropertiesModel.StartExcelPropertiesModel(builder);
            ExcelPropertiesModel.AddIsInMemory(builder, excelProperties.IsInMemory);
            ExcelPropertiesModel.AddIsEditable(builder, true);
            Offset<ExcelPropertiesModel> excelPropertiesModel = ExcelPropertiesModel.EndExcelPropertiesModel(builder);
            builder.Finish(excelPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = excel_create(null, bufferIntPtr, bufferSize, out ffiExcel, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://openxml-office.draviavemal.com/privacy-policy
		/// </summary>
        public Excel(string fileName, ExcelProperties excelProperties = null)
        {
            if (excelProperties == null)
            {
                excelProperties = new ExcelProperties();
            }
            FlatBufferBuilder builder = new(1024);
            ExcelPropertiesModel.StartExcelPropertiesModel(builder);
            ExcelPropertiesModel.AddIsInMemory(builder, excelProperties.IsInMemory);
            ExcelPropertiesModel.AddIsEditable(builder, true);
            Offset<ExcelPropertiesModel> excelPropertiesModel = ExcelPropertiesModel.EndExcelPropertiesModel(builder);
            builder.Finish(excelPropertiesModel.Value);
            byte[] buffer = builder.SizedByteArray();
            int bufferSize = buffer.Length;
            unsafe
            {
                fixed (byte* bufferPtr = buffer)
                {
                    IntPtr bufferIntPtr = (IntPtr)bufferPtr;
                    sbyte statusCode = excel_create(fileName, bufferIntPtr, bufferSize, out ffiExcel, out IntPtr errorMsg);
                    StatusCode.ProcessStatusCode(statusCode, errorMsg);
                }
            }
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
            sbyte statusCode = excel_add_sheet(ffiExcel, sheetName, out IntPtr ffiWorksheet, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
            return new Worksheet(ffiWorksheet);
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="sheetName"></param>
        /// <returns></returns>
        public Worksheet GetWorksheet(string sheetName)
        {
            sbyte statusCode = excel_get_sheet(ffiExcel, sheetName, out IntPtr ffiWorksheet, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
            return new Worksheet(ffiWorksheet);
        }

        /// <summary>
        /// 
        /// </summary>
        /// <param name="oldSheetName"></param>
        /// <param name="newSheetName"></param>
        public bool RenameSheet(string oldSheetName, string newSheetName)
        {
            sbyte statusCode = excel_rename_sheet(ffiExcel, oldSheetName, newSheetName, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
            return true;
        }

        /// <summary>
        /// Even on edit file OpenXML-Office Will clone the source and work on top of it to protect the integrity of source file.
        /// You can save the document at the end of lifecycle targeting the edit file to update or new file.
        /// This is supported for both file path and data stream
        /// </summary>
        public void SaveAs(string filePath)
        {
            sbyte statusCode = excel_save_as(ffiExcel, filePath, out IntPtr errorMsg);
            StatusCode.ProcessStatusCode(statusCode, errorMsg);
        }

    }
}
