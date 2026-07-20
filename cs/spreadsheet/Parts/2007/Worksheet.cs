
using System;
using System.Runtime.InteropServices;
using draviavemal.openxml_office.global_2007;
using Google.FlatBuffers;
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
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "set_cell_ref_value", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_cell_ref_value(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "set_cell_index_value", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_set_cell_index_value(
            IntPtr in_buffer,
            UIntPtr in_buffer_size,
            out IntPtr error_msg
        );

        /// <summary>
        /// 
        /// </summary>
        [DllImport("lib/draviavemal_openxml_office_ffi", EntryPoint = "worksheet_flush", CallingConvention = CallingConvention.Cdecl)]
        private static extern sbyte ffi_worksheet_flush(
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
            Offset<worksheet_flush> fbsDestroyWorksheet = worksheet_flush.Createworksheet_flush(builder, ffiWorksheet);
            builder.Finish(fbsDestroyWorksheet.Value);
            FfiInterop.InvokeVoidFfi(ffi_worksheet_flush, builder);
        }

        public void SetColumnRefProperties(string cellRef, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsCellRef = builder.CreateString(cellRef);
            Offset<worksheet_column_properties> fbsColumnRefProperties = worksheet_column_properties.Createworksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<worksheet_set_column_ref_properties> fbsSetColumnRefProperties = worksheet_set_column_ref_properties.Createworksheet_set_column_ref_properties(builder, ffiWorksheet, fbsCellRef, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnRefProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_column_ref_properties, builder);
        }

        public void SetColumnindexProperties(ushort cellIndex, ColumnProperties columnRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<worksheet_column_properties> fbsColumnRefProperties = worksheet_column_properties.Createworksheet_column_properties(builder, columnRefProperties.Min, columnRefProperties.Max, columnRefProperties.Width, columnRefProperties.Hidden);
            Offset<worksheet_set_column_index_properties> fbsSetColumnIndexProperties = worksheet_set_column_index_properties.Createworksheet_set_column_index_properties(builder, ffiWorksheet, cellIndex, fbsColumnRefProperties);
            builder.Finish(fbsSetColumnIndexProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_column_index_properties, builder);
        }

        public void SetRowindexProperties(ushort cellIndex, RowProperties rowRefProperties)
        {
            FlatBufferBuilder builder = new(1024);
            Offset<worksheet_row_properties> fbsRowRefProperties = worksheet_row_properties.Createworksheet_row_properties(builder, rowRefProperties.Height, rowRefProperties.Hidden, rowRefProperties.TickTop, rowRefProperties.ThickBottom);
            Offset<worksheet_set_row_index_properties> fbsSetRowIndexProperties = worksheet_set_row_index_properties.Createworksheet_set_row_index_properties(builder, ffiWorksheet, cellIndex, fbsRowRefProperties);
            builder.Finish(fbsSetRowIndexProperties.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_row_index_properties, builder);
        }

        public void SetCellRefValues(string cellRef, params CellProperty[] cellProperties)
        {
            FlatBufferBuilder builder = new(1024);
            StringOffset fbsCellRef = builder.CreateString(cellRef);
            VectorOffset fbsColumnCells = worksheet_set_cell_ref_value.CreateColumnCellsVector(builder, BuildCellProperties(builder, cellProperties));
            Offset<worksheet_set_cell_ref_value> fbsSetCellRefValue = worksheet_set_cell_ref_value.Createworksheet_set_cell_ref_value(builder, ffiWorksheet, fbsCellRef, fbsColumnCells);
            builder.Finish(fbsSetCellRefValue.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_cell_ref_value, builder);
        }

        public void SetCellIndexValues(uint rowIndex, ushort columnIndex, params CellProperty[] cellProperties)
        {
            FlatBufferBuilder builder = new(1024);
            VectorOffset fbsColumnCells = worksheet_set_cell_index_value.CreateColumnCellsVector(builder, BuildCellProperties(builder, cellProperties));
            Offset<worksheet_set_cell_index_value> fbsSetCellIndexValue = worksheet_set_cell_index_value.Createworksheet_set_cell_index_value(builder, ffiWorksheet, rowIndex, columnIndex, fbsColumnCells);
            builder.Finish(fbsSetCellIndexValue.Value);
            FfiInterop.InvokeVoidFfi(ffi_set_cell_index_value, builder);
        }

        private static Offset<worksheet_cell_property>[] BuildCellProperties(FlatBufferBuilder builder, CellProperty[] cellProperties)
        {
            Offset<worksheet_cell_property>[] cellOffsets = new Offset<worksheet_cell_property>[cellProperties.Length];
            for (int index = 0; index < cellProperties.Length; index++)
            {
                CellProperty cellProperty = cellProperties[index];
                StringOffset fbsValue = cellProperty.Value != null ? builder.CreateString(cellProperty.Value) : default;
                StringOffset fbsFormula = cellProperty.Formula != null ? builder.CreateString(cellProperty.Formula) : default;
                cellOffsets[index] = worksheet_cell_property.Createworksheet_cell_property(builder, fbsValue, fbsFormula, MapCellDataType(cellProperty.DataType));
            }
            return cellOffsets;
        }

        private static worksheet_cell_data_type MapCellDataType(CellDataType dataType)
        {
            return (worksheet_cell_data_type)(sbyte)dataType;
        }
    }

}