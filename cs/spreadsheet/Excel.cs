using System;
using System.Runtime.InteropServices;
using Google.FlatBuffers;
using draviavemal.openxml_office.global_2007;
using openxml_office_fbs.spreadsheet;

namespace draviavemal.openxml_office.spreadsheet_2007
{
    /// <summary>
    /// This class serves as a versatile tool for working with Excel spreadsheets.
    /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
    /// </summary>
    public class Excel : PrivacyProperties
    {
        private readonly ulong ffiExcelPtr;

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_create", CallingConvention = CallingConvention.Cdecl)]
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
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_add_sheet", CallingConvention = CallingConvention.Cdecl)]
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
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "excel_get_sheet", CallingConvention = CallingConvention.Cdecl)]
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
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_rename_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_rename_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_get_style_id", CallingConvention = CallingConvention.Cdecl)]
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
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_save_as", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_save_as(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_set_active_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_set_active_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_set_visibility", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_set_visibility(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_minimize_workbook", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_minimize_workbook(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_hide_sheet_tabs", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_sheet_tabs(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_hide_vertical_scroll", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_vertical_scroll(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_hide_horizontal_scroll", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_horizontal_scroll(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Excel_hide_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_excel_hide_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// Create New file in the system
        /// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
        /// </summary>
        public Excel(ExcelProperties excelProperties = null)
        {
            ffiExcelPtr = CreateExcel(null, excelProperties);
        }

        /// <summary>
		/// Works with in memory object can be saved to file at later point.
		/// Source file will be cloned and released. hence can be replace by saveAs method if you want to update the same file.
		/// Read Privacy Details document at https://docs.draviavemal.com/openxml-office/privacy-policy
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
            Offset<Excel_settings> fbsExcelSettings = Excel_settings.CreateExcel_settings(builder, true);
            Offset<Excel_create> fbsExcelCreate = Excel_create.CreateExcel_create(builder, fbsFileName, fbsExcelSettings);
            builder.Finish(fbsExcelCreate.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_create, builder);
            Excel_create_return response = Excel_create_return.GetRootAsExcel_create_return(new ByteBuffer(responseBuffer));
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
            Offset<Excel_add_sheet> addSheetOffset = Excel_add_sheet.CreateExcel_add_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(addSheetOffset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_add_sheet, builder);
            Excel_add_sheet_return response = Excel_add_sheet_return.GetRootAsExcel_add_sheet_return(new ByteBuffer(responseBuffer));
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
            Offset<Excel_get_sheet> getSheetOffset = Excel_get_sheet.CreateExcel_get_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(getSheetOffset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_get_sheet, builder);
            Excel_get_sheet_return response = Excel_get_sheet_return.GetRootAsExcel_get_sheet_return(new ByteBuffer(responseBuffer));
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
            Offset<Excel_rename_sheet> renameSheetOffset = Excel_rename_sheet.CreateExcel_rename_sheet(builder, ffiExcelPtr, oldSheetNameOffset, newSheetNameOffset);
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
            Offset<Excel_border_settings> BuildBorderSettings(BorderSetting borderSetting)
            {
                Offset<Excel_color_setting> colorOffset = default;
                if (borderSetting.BorderColor != null)
                {
                    StringOffset colorValueOffset = builder.CreateString(borderSetting.BorderColor.Value ?? string.Empty);
                    colorOffset = Excel_color_setting.CreateExcel_color_setting(builder,
                        (Color_setting_type_values)borderSetting.BorderColor.ColorSettingType,
                        colorValueOffset);
                }
                return Excel_border_settings.CreateExcel_border_settings(builder,
                    colorOffset,
                    (Border_style_values)borderSetting.Style);
            }

            Offset<Excel_border_settings> borderLeftOffset = BuildBorderSettings(cellStyleSetting.BorderLeft ?? new BorderSetting());
            Offset<Excel_border_settings> borderTopOffset = BuildBorderSettings(cellStyleSetting.BorderTop ?? new BorderSetting());
            Offset<Excel_border_settings> borderRightOffset = BuildBorderSettings(cellStyleSetting.BorderRight ?? new BorderSetting());
            Offset<Excel_border_settings> borderBottomOffset = BuildBorderSettings(cellStyleSetting.BorderBottom ?? new BorderSetting());
            Offset<Excel_border_settings> borderDiagonalOffset = BuildBorderSettings(cellStyleSetting.BorderDiagonal ?? new BorderSetting());

            StringOffset textColorValueOffset = builder.CreateString(cellStyleSetting.TextColor?.Value ?? string.Empty);
            Offset<Excel_color_setting> textColorOffset = Excel_color_setting.CreateExcel_color_setting(builder,
                (Color_setting_type_values)(cellStyleSetting.TextColor?.ColorSettingType ?? ColorSettingTypeValues.Indexed),
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

            Offset<Excel_style_setting> styleSettingOffset = Excel_style_setting.CreateExcel_style_setting(builder,
                (Number_format_values)cellStyleSetting.NumberFormat,
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
                (Horizontal_alignment_values)cellStyleSetting.HorizontalAlignment,
                (Vertical_alignment_values)cellStyleSetting.VerticalAlignment);

            Offset<Excel_get_style_id> getStyleIdOffset = Excel_get_style_id.CreateExcel_get_style_id(builder, ffiExcelPtr, styleSettingOffset);
            builder.Finish(getStyleIdOffset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_excel_get_style_id, builder);
            Excel_get_style_id_return response = Excel_get_style_id_return.GetRootAsExcel_get_style_id_return(new ByteBuffer(responseBuffer));
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
            Offset<Excel_save_as> saveAsOffset = Excel_save_as.CreateExcel_save_as(builder, ffiExcelPtr, filePathOffset);
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
            Offset<Excel_set_active_sheet> offset = Excel_set_active_sheet.CreateExcel_set_active_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_set_active_sheet, builder);
        }

        /// <summary>
        /// Set the visibility of the workbook window.
        /// </summary>
        public void SetVisibility(bool isVisible)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Excel_set_visibility> offset = Excel_set_visibility.CreateExcel_set_visibility(builder, ffiExcelPtr, isVisible);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_set_visibility, builder);
        }

        /// <summary>
        /// Minimize or restore the workbook window.
        /// </summary>
        public void MinimizeWorkbook(bool isMinimized)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Excel_minimize_workbook> offset = Excel_minimize_workbook.CreateExcel_minimize_workbook(builder, ffiExcelPtr, isMinimized);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_minimize_workbook, builder);
        }

        /// <summary>
        /// Show or hide the sheet tab bar.
        /// </summary>
        public void HideSheetTabs(bool hideTab)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Excel_hide_sheet_tabs> offset = Excel_hide_sheet_tabs.CreateExcel_hide_sheet_tabs(builder, ffiExcelPtr, hideTab);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_sheet_tabs, builder);
        }

        /// <summary>
        /// Show or hide the vertical scroll bar.
        /// </summary>
        public void HideVerticalScroll(bool hide)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Excel_hide_vertical_scroll> offset = Excel_hide_vertical_scroll.CreateExcel_hide_vertical_scroll(builder, ffiExcelPtr, hide);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_vertical_scroll, builder);
        }

        /// <summary>
        /// Show or hide the horizontal scroll bar.
        /// </summary>
        public void HideHorizontalScroll(bool hide)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Excel_hide_horizontal_scroll> offset = Excel_hide_horizontal_scroll.CreateExcel_hide_horizontal_scroll(builder, ffiExcelPtr, hide);
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
            Offset<Excel_hide_sheet> offset = Excel_hide_sheet.CreateExcel_hide_sheet(builder, ffiExcelPtr, sheetNameOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_excel_hide_sheet, builder);
        }
    }
}
