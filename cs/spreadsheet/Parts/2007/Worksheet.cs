
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using draviavemal.openxml_office.global_2007;
using Google.FlatBuffers;
using openxml_office_fbs.global;
using openxml_office_fbs.spreadsheet;

namespace draviavemal.openxml_office.spreadsheet_2007
{

    public class Worksheet : IDisposable
    {
        private readonly ulong ffiWorksheet;

        internal Worksheet(ulong ffiWorksheet)
        {
            this.ffiWorksheet = ffiWorksheet;
        }

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Set_column_ref_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_column_ref_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Set_column_index_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_column_index_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Set_row_index_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_row_index_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Set_cell_ref_value", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_cell_ref_value(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Set_cell_index_value", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_cell_index_value(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_flush", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_Worksheet_flush(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_set_merge_cell", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_set_merge_cell(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_remove_merge_cell", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_remove_merge_cell(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_set_hyperlink", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_set_hyperlink(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_remove_hyperlink", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_remove_hyperlink(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_delete_sheet", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_delete_sheet(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_get_range_cell_properties", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_Worksheet_get_range_cell_properties(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_list_merge_cell", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_list_merge_cell(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_list_hyperlinks", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_list_hyperlinks(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr out_buffer,
            out UIntPtr out_buffer_size,
            out IntPtr error_msg
        );

        [DllImport("draviavemal_openxml_office_ffi", EntryPoint = "Worksheet_add_picture", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_add_picture(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// Destroys the worksheet and releases any unmanaged resources.
        /// </summary>
        public void Dispose()
        {
            FlatBufferBuilder builder = new(1024);
            Offset<Worksheet_flush> fbsDestroyWorksheet = Worksheet_flush.CreateWorksheet_flush(builder, ffiWorksheet);
            builder.Finish(fbsDestroyWorksheet.Value);
            FfiInterop.InvokeVoidFfi(ffi_Worksheet_flush, builder);
        }

        public void SetColumnRefProperties(string cellRef, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsCellRef = builder.CreateString(cellRef);
            Offset<Worksheet_column_properties> fbsColumnRefProperties = Worksheet_column_properties.CreateWorksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<Worksheet_set_column_ref_properties> fbsSetColumnRefProperties = Worksheet_set_column_ref_properties.CreateWorksheet_set_column_ref_properties(builder, ffiWorksheet, fbsCellRef, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnRefProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_column_ref_properties, builder);
        }

        public void SetColumnindexProperties(ushort cellIndex, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<Worksheet_column_properties> fbsColumnRefProperties = Worksheet_column_properties.CreateWorksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<Worksheet_set_column_index_properties> fbsSetColumnIndexProperties = Worksheet_set_column_index_properties.CreateWorksheet_set_column_index_properties(builder, ffiWorksheet, cellIndex, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnIndexProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_column_index_properties, builder);
        }

        public void SetRowindexProperties(ushort cellIndex, RowProperties rowRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<Worksheet_row_properties> fbsRowRefProperties = Worksheet_row_properties.CreateWorksheet_row_properties(builder, rowRefProperties.Height, rowRefProperties.Hidden, rowRefProperties.TickTop, rowRefProperties.ThickBottom);
            Offset<Worksheet_set_row_index_properties> fbsSetRowIndexProperties = Worksheet_set_row_index_properties.CreateWorksheet_set_row_index_properties(builder, ffiWorksheet, cellIndex, fbsRowRefProperties);
            builder.Finish(fbsSetRowIndexProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_row_index_properties, builder);
        }

        public void SetCellRefValues(string cellRef, params CellProperty[] cellProperties)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsCellRef = builder.CreateString(cellRef);
            VectorOffset fbsColumnCells = Worksheet_set_cell_ref_value.CreateColumnCellsVector(builder, BuildCellProperties(builder, cellProperties));
            Offset<Worksheet_set_cell_ref_value> fbsSetCellRefValue = Worksheet_set_cell_ref_value.CreateWorksheet_set_cell_ref_value(builder, ffiWorksheet, fbsCellRef, fbsColumnCells);
            builder.Finish(fbsSetCellRefValue.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_cell_ref_value, builder);
        }

        public void SetCellIndexValues(uint rowIndex, ushort columnIndex, params CellProperty[] cellProperties)
        {
            FlatBufferBuilder builder = new(1024);
            VectorOffset fbsColumnCells = Worksheet_set_cell_index_value.CreateColumnCellsVector(builder, BuildCellProperties(builder, cellProperties));
            Offset<Worksheet_set_cell_index_value> fbsSetCellIndexValue = Worksheet_set_cell_index_value.CreateWorksheet_set_cell_index_value(builder, ffiWorksheet, rowIndex, columnIndex, fbsColumnCells);
            builder.Finish(fbsSetCellIndexValue.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_cell_index_value, builder);
        }

        private static Offset<Worksheet_cell_property>[] BuildCellProperties(FlatBufferBuilder builder, CellProperty[] cellProperties)
        {
            Offset<Worksheet_cell_property>[] cellOffsets = new Offset<Worksheet_cell_property>[cellProperties.Length];
            for (int index = 0; index < cellProperties.Length; index++)
            {
                CellProperty cellProperty = cellProperties[index];
                StringOffset fbsValue = cellProperty.Value != null ? builder.CreateString(cellProperty.Value) : default;
                StringOffset fbsFormula = cellProperty.Formula != null ? builder.CreateString(cellProperty.Formula) : default;
                cellOffsets[index] = Worksheet_cell_property.CreateWorksheet_cell_property(builder, fbsValue, fbsFormula, MapCellDataType(cellProperty.DataType));
            }
            return cellOffsets;
        }

        private static Worksheet_cell_data_type MapCellDataType(CellDataType dataType)
        {
            return (Worksheet_cell_data_type)(sbyte)dataType;
        }

        private static Offset<Worksheet_reference_range> BuildReferenceRange(FlatBufferBuilder builder, ReferenceRange range)
        {
            return Worksheet_reference_range.CreateWorksheet_reference_range(builder,
                range.ColumnStart, range.ColumnEnd, range.RowStart, range.RowEnd);
        }

        /// <summary>
        /// Merge a range of cells.
        /// </summary>
        public void SetMergeCell(ReferenceRange range)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Worksheet_reference_range> rangeOffset = BuildReferenceRange(builder, range);
            Offset<Worksheet_set_merge_cell> offset = Worksheet_set_merge_cell.CreateWorksheet_set_merge_cell(builder, ffiWorksheet, rangeOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_set_merge_cell, builder);
        }

        /// <summary>
        /// Remove a merged cell range.
        /// </summary>
        public void RemoveMergeCell(ReferenceRange range)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Worksheet_reference_range> rangeOffset = BuildReferenceRange(builder, range);
            Offset<Worksheet_remove_merge_cell> offset = Worksheet_remove_merge_cell.CreateWorksheet_remove_merge_cell(builder, ffiWorksheet, rangeOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_remove_merge_cell, builder);
        }

        /// <summary>
        /// Add a hyperlink to a cell range.
        /// </summary>
        public void SetHyperlink(string link, ReferenceRange range, string display = null)
        {
            FlatBufferBuilder builder = new(512);
            StringOffset linkOffset = builder.CreateString(link);
            StringOffset displayOffset = display != null ? builder.CreateString(display) : default;
            Offset<Worksheet_reference_range> rangeOffset = BuildReferenceRange(builder, range);
            Offset<Worksheet_set_hyperlink> offset = Worksheet_set_hyperlink.CreateWorksheet_set_hyperlink(builder, ffiWorksheet, displayOffset, linkOffset, rangeOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_set_hyperlink, builder);
        }

        /// <summary>
        /// Remove a hyperlink from a cell range.
        /// </summary>
        public void RemoveHyperlink(ReferenceRange range)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Worksheet_reference_range> rangeOffset = BuildReferenceRange(builder, range);
            Offset<Worksheet_remove_hyperlink> offset = Worksheet_remove_hyperlink.CreateWorksheet_remove_hyperlink(builder, ffiWorksheet, rangeOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_remove_hyperlink, builder);
        }

        /// <summary>
        /// Delete the worksheet and all its components.
        /// </summary>
        public void DeleteSheet()
        {
            FlatBufferBuilder builder = new(128);
            Offset<Worksheet_delete_sheet> offset = Worksheet_delete_sheet.CreateWorksheet_delete_sheet(builder, ffiWorksheet);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_delete_sheet, builder);
        }

        /// <summary>
        /// Get cell properties for all cells within the specified range.
        /// </summary>
        public CellPackage[] GetRangeCellProperties(ReferenceRange range)
        {
            FlatBufferBuilder builder = new(256);
            Offset<Worksheet_reference_range> rangeOffset = BuildReferenceRange(builder, range);
            Offset<Worksheet_get_range_cell_properties> offset = Worksheet_get_range_cell_properties.CreateWorksheet_get_range_cell_properties(builder, ffiWorksheet, rangeOffset);
            builder.Finish(offset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_Worksheet_get_range_cell_properties, builder);
            Worksheet_get_range_cell_properties_return response = Worksheet_get_range_cell_properties_return.GetRootAsWorksheet_get_range_cell_properties_return(new ByteBuffer(responseBuffer));
            List<CellPackage> result = new();
            for (int i = 0; i < response.CellPackagesLength; i++)
            {
                Worksheet_cell_package pkg = response.CellPackages(i).Value;
                Worksheet_cell_property? prop = pkg.CellProperty;
                result.Add(new CellPackage
                {
                    CellRef = pkg.CellRef,
                    RowIndex = pkg.RowIndex,
                    ColumnIndex = pkg.ColumnIndex,
                    CellProperty = prop.HasValue ? new CellProperty
                    {
                        Value = prop.Value.Value,
                        Formula = prop.Value.Formula,
                        DataType = (CellDataType)(sbyte)prop.Value.DataType,
                    } : null,
                });
            }
            return result.ToArray();
        }

        /// <summary>
        /// List all merged cell ranges in the worksheet.
        /// </summary>
        public ReferenceRange[] ListMergeCell()
        {
            FlatBufferBuilder builder = new(128);
            Offset<Worksheet_list_merge_cell> offset = Worksheet_list_merge_cell.CreateWorksheet_list_merge_cell(builder, ffiWorksheet);
            builder.Finish(offset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_worksheet_list_merge_cell, builder);
            Worksheet_list_merge_cell_return response = Worksheet_list_merge_cell_return.GetRootAsWorksheet_list_merge_cell_return(new ByteBuffer(responseBuffer));
            List<ReferenceRange> result = new();
            for (int i = 0; i < response.RangesLength; i++)
            {
                Worksheet_reference_range r = response.Ranges(i).Value;
                result.Add(new ReferenceRange
                {
                    ColumnStart = r.ColumnStart,
                    ColumnEnd = r.ColumnEnd,
                    RowStart = r.RowStart,
                    RowEnd = r.RowEnd,
                });
            }
            return result.ToArray();
        }

        /// <summary>
        /// List all hyperlinks in the worksheet.
        /// </summary>
        public HyperlinkInfo[] ListHyperlinks()
        {
            FlatBufferBuilder builder = new(128);
            Offset<Worksheet_list_hyperlinks> offset = Worksheet_list_hyperlinks.CreateWorksheet_list_hyperlinks(builder, ffiWorksheet);
            builder.Finish(offset.Value);
            byte[] responseBuffer = FfiInterop.InvokeBufferFfi(ffi_worksheet_list_hyperlinks, builder);
            Worksheet_list_hyperlinks_return response = Worksheet_list_hyperlinks_return.GetRootAsWorksheet_list_hyperlinks_return(new ByteBuffer(responseBuffer));
            List<HyperlinkInfo> result = new();
            for (int i = 0; i < response.HyperlinksLength; i++)
            {
                Worksheet_hyperlink h = response.Hyperlinks(i).Value;
                Worksheet_reference_range? r = h.RefRange;
                result.Add(new HyperlinkInfo
                {
                    Display = h.Display,
                    Link = h.Link,
                    Range = r.HasValue ? new ReferenceRange
                    {
                        ColumnStart = r.Value.ColumnStart,
                        ColumnEnd = r.Value.ColumnEnd,
                        RowStart = r.Value.RowStart,
                        RowEnd = r.Value.RowEnd,
                    } : null,
                });
            }
            return result.ToArray();
        }

        /// <summary>
        /// Add a picture to the worksheet at the specified anchor positions.
        /// </summary>
        public void AddPicture(string imagePath, ExcelPictureSetting pictureSetting)
        {
            FlatBufferBuilder builder = new(512);
            StringOffset imagePathOffset = builder.CreateString(imagePath);
            Offset<Worksheet_anchor_position> fromOffset = Worksheet_anchor_position.CreateWorksheet_anchor_position(
                builder,
                pictureSetting.From.Column,
                pictureSetting.From.ColumnOffset,
                pictureSetting.From.Row,
                pictureSetting.From.RowOffset);
            Offset<Worksheet_anchor_position> toOffset = Worksheet_anchor_position.CreateWorksheet_anchor_position(
                builder,
                pictureSetting.To.Column,
                pictureSetting.To.ColumnOffset,
                pictureSetting.To.Row,
                pictureSetting.To.RowOffset);
            Offset<Worksheet_excel_hyperlink> hyperlinkOffset = default;
            if (pictureSetting.HyperlinkProperties != null)
            {
                StringOffset hlDisplay = pictureSetting.HyperlinkProperties.Display != null
                    ? builder.CreateString(pictureSetting.HyperlinkProperties.Display)
                    : default;
                StringOffset hlLink = builder.CreateString(pictureSetting.HyperlinkProperties.Link ?? string.Empty);
                hyperlinkOffset = Worksheet_excel_hyperlink.CreateWorksheet_excel_hyperlink(
                    builder,
                    hlDisplay,
                    (Worksheet_excel_hyperlink_type)pictureSetting.HyperlinkProperties.LinkType,
                    hlLink);
            }
            Offset<Global_picture_settings> pictureSettingOffset = Global_picture_settings.CreateGlobal_picture_settings(
                builder,
                (Global_image_type)pictureSetting.ImageType
            );
            Offset<Worksheet_picture_setting> settingOffset = Worksheet_picture_setting.CreateWorksheet_picture_setting(
                builder,
                pictureSettingOffset,
                fromOffset,
                toOffset,
                hyperlinkOffset);
            Offset<Worksheet_add_picture> offset = Worksheet_add_picture.CreateWorksheet_add_picture(builder, ffiWorksheet, imagePathOffset, settingOffset);
            builder.Finish(offset.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_add_picture, builder);
        }
    }

}