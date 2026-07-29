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
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_get_style_id", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_get_style_id(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
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

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_set_active_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_set_active_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_set_visibility", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_set_visibility(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_minimize_workbook", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_minimize_workbook(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_hide_sheet_tabs", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_sheet_tabs(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_hide_vertical_scroll", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_vertical_scroll(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_hide_horizontal_scroll", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_horizontal_scroll(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "excel_hide_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
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
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_create, builder);
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
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_add_sheet, builder);
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
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_get_sheet, builder);
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
            FfiInterop.InvokeVoidFfi(ffi_excel_rename_sheet, builder);
            return true;
        }

        /// <summary>
        /// Get or create a style ID for the given cell style settings.
        /// </summary>
        public StyleId GetStyleId(CellStyleSetting cellStyleSetting)
        {
            FlatBufferBuilder builder = new(1024);

            // Build border settings
            Offset<excel_border_settings> BuildBorderSettings(BorderSetting borderSetting)
            {
                Offset<excel_color_setting> colorOffset = default;
                if (borderSetting.BorderColor != null)
                {
                    StringOffset colorValueOffset = builder.CreateString(borderSetting.BorderColor.Value ?? string.Empty);
                    colorOffset = excel_color_setting.Createexcel_color_setting(builder,
                        (color_setting_type_values)borderSetting.BorderColor.ColorSettingType,
                        colorValueOffset);
                }
                return excel_border_settings.Createexcel_border_settings(builder,
                    colorOffset,
                    (border_style_values)borderSetting.Style);
            }

            Offset<excel_border_settings> borderLeftOffset = BuildBorderSettings(cellStyleSetting.BorderLeft ?? new BorderSetting());
            Offset<excel_border_settings> borderTopOffset = BuildBorderSettings(cellStyleSetting.BorderTop ?? new BorderSetting());
            Offset<excel_border_settings> borderRightOffset = BuildBorderSettings(cellStyleSetting.BorderRight ?? new BorderSetting());
            Offset<excel_border_settings> borderBottomOffset = BuildBorderSettings(cellStyleSetting.BorderBottom ?? new BorderSetting());
            Offset<excel_border_settings> borderDiagonalOffset = BuildBorderSettings(cellStyleSetting.BorderDiagonal ?? new BorderSetting());

            StringOffset textColorValueOffset = builder.CreateString(cellStyleSetting.TextColor?.Value ?? string.Empty);
            Offset<excel_color_setting> textColorOffset = excel_color_setting.Createexcel_color_setting(builder,
                (color_setting_type_values)(cellStyleSetting.TextColor?.ColorSettingType ?? ColorSettingTypeValues.Indexed),
                textColorValueOffset);

            StringOffset fontFamilyOffset = builder.CreateString(cellStyleSetting.FontFamily ?? string.Empty);
            StringOffset customNumberFormatOffset = cellStyleSetting.CustomNumberFormat != null
                ? builder.CreateString(cellStyleSetting.CustomNumberFormat)
                : default;
            StringOffset backgroundColorOffset = cellStyleSetting.BackgroundColor != null
                ? builder.CreateString(cellStyleSetting.BackgroundColor)
                : default;
            StringOffset foregroundColorOffset = cellStyleSetting.ForegroundColor != null
                ? builder.CreateString(cellStyleSetting.ForegroundColor)
                : default;

            Offset<excel_style_setting> styleSettingOffset = excel_style_setting.Createexcel_style_setting(builder,
                (number_format_values)cellStyleSetting.NumberFormat,
                customNumberFormatOffset,
                borderLeftOffset,
                borderTopOffset,
                borderRightOffset,
                borderBottomOffset,
                borderDiagonalOffset,
                fontFamilyOffset,
                cellStyleSetting.FontSize,
                textColorOffset,
                cellStyleSetting.IsBold,
                cellStyleSetting.IsItalic,
                cellStyleSetting.IsUnderline,
                cellStyleSetting.IsDoubleUnderline,
                cellStyleSetting.IsWrapText,
                backgroundColorOffset,
                foregroundColorOffset,
                (horizontal_alignment_values)cellStyleSetting.HorizontalAlignment,
                (vertical_alignment_values)cellStyleSetting.VerticalAlignment);

            Offset<excel_get_style_id> getStyleIdOffset = excel_get_style_id.Createexcel_get_style_id(builder, ffiExcelPtr, styleSettingOffset);
            builder.Finish(getStyleIdOffset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_get_style_id, builder);
            excel_get_style_id_return response = excel_get_style_id_return.GetRootAsexcel_get_style_id_return(new ByteBuffer(responseBuffer));
            return new StyleId(response.StyleIdPtr);
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
            FfiInterop.InvokeBufferFfi(ffi_excel_save_as, builder);
        }

        /// <summary>
        /// Set the active sheet that opens by default when the workbook is opened.
        /// </summary>
        public void SetActiveSheet(string sheetName)
        {
            FlatBufferBuilder builder = new(256);
            StringOffset sheetNameOffset = builder.CreateString(sheetName);
            Offset<excel_set_active_sheet> offset = excel_set_active_sheet.Createexcel_set_active_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_set_active_sheet, builder);
        }

        /// <summary>
        /// Set the visibility of the workbook window.
        /// </summary>
        public void SetVisibility(bool isVisible)
        {
            FlatBufferBuilder builder = new(256);
            Offset<excel_set_visibility> offset = excel_set_visibility.Createexcel_set_visibility(builder, ffiExcelPtr, isVisible);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_set_visibility, builder);
        }

        /// <summary>
        /// Minimize or restore the workbook window.
        /// </summary>
        public void MinimizeWorkbook(bool isMinimized)
        {
            FlatBufferBuilder builder = new(256);
            Offset<excel_minimize_workbook> offset = excel_minimize_workbook.Createexcel_minimize_workbook(builder, ffiExcelPtr, isMinimized);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_minimize_workbook, builder);
        }

        /// <summary>
        /// Show or hide the sheet tab bar.
        /// </summary>
        public void HideSheetTabs(bool hideTab)
        {
            FlatBufferBuilder builder = new(256);
            Offset<excel_hide_sheet_tabs> offset = excel_hide_sheet_tabs.Createexcel_hide_sheet_tabs(builder, ffiExcelPtr, hideTab);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_sheet_tabs, builder);
        }

        /// <summary>
        /// Show or hide the vertical scroll bar.
        /// </summary>
        public void HideVerticalScroll(bool hide)
        {
            FlatBufferBuilder builder = new(256);
            Offset<excel_hide_vertical_scroll> offset = excel_hide_vertical_scroll.Createexcel_hide_vertical_scroll(builder, ffiExcelPtr, hide);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_vertical_scroll, builder);
        }

        /// <summary>
        /// Show or hide the horizontal scroll bar.
        /// </summary>
        public void HideHorizontalScroll(bool hide)
        {
            FlatBufferBuilder builder = new(256);
            Offset<excel_hide_horizontal_scroll> offset = excel_hide_horizontal_scroll.Createexcel_hide_horizontal_scroll(builder, ffiExcelPtr, hide);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_horizontal_scroll, builder);
        }

        /// <summary>
        /// Hide a specific sheet in the workbook.
        /// </summary>
        public void HideSheet(string sheetName)
        {
            FlatBufferBuilder builder = new(256);
            StringOffset sheetNameOffset = builder.CreateString(sheetName);
            Offset<excel_hide_sheet> offset = excel_hide_sheet.Createexcel_hide_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_sheet, builder);
        }
    }
}
