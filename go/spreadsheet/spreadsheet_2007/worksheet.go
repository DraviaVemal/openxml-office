package spreadsheet_2007

/*
#cgo CFLAGS: -I${SRCDIR}/../../lib
#cgo linux LDFLAGS: -L${SRCDIR}/../../lib -ldraviavemal_openxml_office_ffi -llzma -lbz2 -lgcc_s -lutil -lrt -lpthread -lm -ldl -lc
#cgo darwin LDFLAGS: -L${SRCDIR}/../../lib -ldraviavemal_openxml_office_ffi -llzma -lbz2 -lpthread -lm -ldl -framework CoreFoundation -framework Security
#cgo windows LDFLAGS: -L${SRCDIR}/../../lib -ldraviavemal_openxml_office_ffi -llzma -lbz2 -lws2_32 -luserenv -lbcrypt -lntdll
#include <headers.h>
*/
import "C"

import (
	"errors"
	"unsafe"

	flatbuffers "github.com/google/flatbuffers/go"
)

type Worksheet struct {
	worksheetPtr uint64
}

// Flush persists changes and frees the native worksheet object.
func (ws *Worksheet) Flush() error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(1)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	offset := builder.EndObject()
	builder.Finish(offset)
	buffer := builder.FinishedBytes()
	var outError *C.char
	code := C.worksheet_flush(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outError,
	)
	if code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetColumnRefProperties sets column properties identified by a cell reference (e.g. "A1").
func (ws *Worksheet) SetColumnRefProperties(cellRef string, props *ColumnProperties) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	cellRefOffset := builder.CreateString(cellRef)
	propsOffset := buildColumnProperties(builder, props)
	builder.StartObject(3)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, cellRefOffset, 0)
	builder.PrependUOffsetTSlot(2, propsOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.set_column_ref_properties((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetColumnIndexProperties sets column properties by numeric column index (1-based).
func (ws *Worksheet) SetColumnIndexProperties(columnIndex uint16, props *ColumnProperties) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	propsOffset := buildColumnProperties(builder, props)
	builder.StartObject(3)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUint16Slot(1, columnIndex, 0)
	builder.PrependUOffsetTSlot(2, propsOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.set_column_index_properties((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetRowIndexProperties sets row properties by numeric row index (1-based).
func (ws *Worksheet) SetRowIndexProperties(rowIndex uint32, props RowProperties) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)

	var stylePtrVal uint64
	if props.StyleId != nil {
		stylePtrVal = props.StyleId.ptr
	}
	builder.StartObject(5)
	builder.PrependFloat32Slot(0, props.Height, 0)
	builder.PrependBoolSlot(1, props.Hidden, false)
	builder.PrependBoolSlot(2, props.ThickTop, false)
	builder.PrependBoolSlot(3, props.ThickBottom, false)
	if stylePtrVal != 0 {
		builder.PrependUint64Slot(4, stylePtrVal, 0)
	}
	rowPropsOffset := builder.EndObject()

	builder.StartObject(3)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUint32Slot(1, rowIndex, 0)
	builder.PrependUOffsetTSlot(2, rowPropsOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.set_row_index_properties((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetCellRefValues sets cell values starting at the given cell reference.
func (ws *Worksheet) SetCellRefValues(cellRef string, cells []CellProperty) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(256)
	cellRefOffset := builder.CreateString(cellRef)
	cellOffsets := buildCellProperties(builder, cells)
	builder.StartVector(4, len(cellOffsets), 4)
	for i := len(cellOffsets) - 1; i >= 0; i-- {
		builder.PrependUOffsetT(cellOffsets[i])
	}
	cellsVector := builder.EndVector(len(cellOffsets))

	builder.StartObject(3)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, cellRefOffset, 0)
	builder.PrependUOffsetTSlot(2, cellsVector, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.set_cell_ref_value((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetCellIndexValues sets cell values at the given row/column position.
func (ws *Worksheet) SetCellIndexValues(rowIndex uint32, columnIndex uint16, cells []CellProperty) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(256)
	cellOffsets := buildCellProperties(builder, cells)
	builder.StartVector(4, len(cellOffsets), 4)
	for i := len(cellOffsets) - 1; i >= 0; i-- {
		builder.PrependUOffsetT(cellOffsets[i])
	}
	cellsVector := builder.EndVector(len(cellOffsets))

	builder.StartObject(4)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUint32Slot(1, rowIndex, 0)
	builder.PrependUint16Slot(2, columnIndex, 0)
	builder.PrependUOffsetTSlot(3, cellsVector, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.set_cell_index_value((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetMergeCell merges the cells in the given range.
func (ws *Worksheet) SetMergeCell(refRange ReferenceRange) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	rangeOffset := buildReferenceRange(builder, refRange)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, rangeOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_set_merge_cell((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// RemoveMergeCell removes the merge from the given cell range.
func (ws *Worksheet) RemoveMergeCell(refRange ReferenceRange) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	rangeOffset := buildReferenceRange(builder, refRange)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, rangeOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_remove_merge_cell((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetHyperlink adds a hyperlink to the specified cell range.
// display is optional; pass "" to use the link URL as the display text.
func (ws *Worksheet) SetHyperlink(link string, refRange ReferenceRange, display string) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(256)
	var displayOffset flatbuffers.UOffsetT
	if display != "" {
		displayOffset = builder.CreateString(display)
	}
	linkOffset := builder.CreateString(link)
	rangeOffset := buildReferenceRange(builder, refRange)

	builder.StartObject(4)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, displayOffset, 0)
	builder.PrependUOffsetTSlot(2, linkOffset, 0)
	builder.PrependUOffsetTSlot(3, rangeOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_set_hyperlink((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// RemoveHyperlink removes the hyperlink from the given cell range.
func (ws *Worksheet) RemoveHyperlink(refRange ReferenceRange) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	rangeOffset := buildReferenceRange(builder, refRange)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, rangeOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_remove_hyperlink((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// DeleteSheet permanently removes this worksheet from the workbook.
func (ws *Worksheet) DeleteSheet() error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(1)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_delete_sheet((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// GetRangeCellProperties returns the cell values and properties for the given range.
func (ws *Worksheet) GetRangeCellProperties(refRange ReferenceRange) ([]CellPackage, error) {
	if ws == nil {
		return nil, errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(128)
	rangeOffset := buildReferenceRange(builder, refRange)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, rangeOffset, 0)
	builder.Finish(builder.EndObject())
	buffer := builder.FinishedBytes()

	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char
	code := C.worksheet_get_range_cell_properties(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return nil, ffiError(outError)
	}
	data := consumeBuffer(outBuffer, outBufferSize)

	rootPos := flatbuffers.GetUOffsetT(data)
	root := flatbuffers.Table{Bytes: data, Pos: rootPos}
	packages := fbsReadTableVector(root, 4, func(t flatbuffers.Table) CellPackage {
		cp := CellPackage{
			CellRef:     fbsReadString(t, 4),
			RowIndex:    t.GetUint32Slot(6, 0),
			ColumnIndex: t.GetUint16Slot(8, 0),
		}
		if propOffset := flatbuffers.UOffsetT(t.Offset(10)); propOffset != 0 {
			propPos := t.Indirect(propOffset + t.Pos)
			prop := flatbuffers.Table{Bytes: data, Pos: propPos}
			cp.Property = &CellProperty{
				Value:    fbsReadString(prop, 4),
				Formula:  fbsReadString(prop, 6),
				DataType: CellDataType(prop.GetInt8Slot(8, 0)),
			}
		}
		return cp
	})
	return packages, nil
}

// ListMergeCell returns all merged cell ranges in the worksheet.
func (ws *Worksheet) ListMergeCell() ([]ReferenceRange, error) {
	if ws == nil {
		return nil, errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(1)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.Finish(builder.EndObject())
	buffer := builder.FinishedBytes()

	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char
	code := C.worksheet_list_merge_cell(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return nil, ffiError(outError)
	}
	data := consumeBuffer(outBuffer, outBufferSize)

	rootPos := flatbuffers.GetUOffsetT(data)
	root := flatbuffers.Table{Bytes: data, Pos: rootPos}
	ranges := fbsReadTableVector(root, 4, func(t flatbuffers.Table) ReferenceRange {
		return ReferenceRange{
			ColumnStart: t.GetUint16Slot(4, 0),
			ColumnEnd:   t.GetUint16Slot(6, 0),
			RowStart:    t.GetUint32Slot(8, 0),
			RowEnd:      t.GetUint32Slot(10, 0),
		}
	})
	return ranges, nil
}

// ListHyperlinks returns all hyperlinks defined in the worksheet.
func (ws *Worksheet) ListHyperlinks() ([]HyperlinkInfo, error) {
	if ws == nil {
		return nil, errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(1)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.Finish(builder.EndObject())
	buffer := builder.FinishedBytes()

	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char
	code := C.worksheet_list_hyperlinks(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return nil, ffiError(outError)
	}
	data := consumeBuffer(outBuffer, outBufferSize)

	rootPos := flatbuffers.GetUOffsetT(data)
	root := flatbuffers.Table{Bytes: data, Pos: rootPos}
	links := fbsReadTableVector(root, 4, func(t flatbuffers.Table) HyperlinkInfo {
		info := HyperlinkInfo{
			Display: fbsReadString(t, 4),
			Link:    fbsReadString(t, 6),
		}
		if o := flatbuffers.UOffsetT(t.Offset(8)); o != 0 {
			rPos := t.Indirect(o + t.Pos)
			r := flatbuffers.Table{Bytes: data, Pos: rPos}
			info.Range = ReferenceRange{
				ColumnStart: r.GetUint16Slot(4, 0),
				ColumnEnd:   r.GetUint16Slot(6, 0),
				RowStart:    r.GetUint32Slot(8, 0),
				RowEnd:      r.GetUint32Slot(10, 0),
			}
		}
		return info
	})
	return links, nil
}

// AddPicture inserts an image file into the worksheet at the specified anchor positions.
func (ws *Worksheet) AddPicture(imagePath string, setting ExcelPictureSetting) error {
	if ws == nil {
		return errors.New("nil Worksheet")
	}
	builder := flatbuffers.NewBuilder(512)

	imagePathOffset := builder.CreateString(imagePath)

	var hyperlinkOffset flatbuffers.UOffsetT
	if setting.Hyperlink != nil {
		var displayOffset, linkOffset flatbuffers.UOffsetT
		if setting.Hyperlink.Display != "" {
			displayOffset = builder.CreateString(setting.Hyperlink.Display)
		}
		if setting.Hyperlink.Link != "" {
			linkOffset = builder.CreateString(setting.Hyperlink.Link)
		}
		builder.StartObject(3)
		builder.PrependUOffsetTSlot(0, displayOffset, 0)
		builder.PrependInt8Slot(1, int8(setting.Hyperlink.LinkType), 0)
		builder.PrependUOffsetTSlot(2, linkOffset, 0)
		hyperlinkOffset = builder.EndObject()
	}

	fromOffset := buildAnchorPosition(builder, setting.From)
	toOffset := buildAnchorPosition(builder, setting.To)

	builder.StartObject(4)
	builder.PrependInt8Slot(0, int8(setting.ImageType), 0)
	builder.PrependUOffsetTSlot(1, fromOffset, 0)
	builder.PrependUOffsetTSlot(2, toOffset, 0)
	builder.PrependUOffsetTSlot(3, hyperlinkOffset, 0)
	pictureSettingOffset := builder.EndObject()

	builder.StartObject(3)
	builder.PrependUint64Slot(0, ws.worksheetPtr, 0)
	builder.PrependUOffsetTSlot(1, imagePathOffset, 0)
	builder.PrependUOffsetTSlot(2, pictureSettingOffset, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.worksheet_add_picture((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

func buildReferenceRange(builder *flatbuffers.Builder, r ReferenceRange) flatbuffers.UOffsetT {
	builder.StartObject(4)
	builder.PrependUint16Slot(0, r.ColumnStart, 0)
	builder.PrependUint16Slot(1, r.ColumnEnd, 0)
	builder.PrependUint32Slot(2, r.RowStart, 0)
	builder.PrependUint32Slot(3, r.RowEnd, 0)
	return builder.EndObject()
}

func buildAnchorPosition(builder *flatbuffers.Builder, a AnchorPosition) flatbuffers.UOffsetT {
	builder.StartObject(4)
	builder.PrependUint16Slot(0, a.Column, 0)
	builder.PrependUint16Slot(1, a.ColumnOffset, 0)
	builder.PrependUint32Slot(2, a.Row, 0)
	builder.PrependUint32Slot(3, a.RowOffset, 0)
	return builder.EndObject()
}

func buildColumnProperties(builder *flatbuffers.Builder, props *ColumnProperties) flatbuffers.UOffsetT {
	var stylePtrVal uint64
	if props != nil && props.StyleId != nil {
		stylePtrVal = props.StyleId.ptr
	}
	builder.StartObject(6)
	if props != nil {
		builder.PrependUint32Slot(0, props.Min, 0)
		builder.PrependUint32Slot(1, props.Max, 0)
		builder.PrependFloat32Slot(2, props.Width, 0)
		builder.PrependBoolSlot(3, props.Hidden, false)
		builder.PrependBoolSlot(4, props.BestFit, false)
	}
	if stylePtrVal != 0 {
		builder.PrependUint64Slot(5, stylePtrVal, 0)
	}
	return builder.EndObject()
}

func buildCellProperties(builder *flatbuffers.Builder, cells []CellProperty) []flatbuffers.UOffsetT {
	offsets := make([]flatbuffers.UOffsetT, len(cells))
	for i, c := range cells {
		var valueOffset, formulaOffset flatbuffers.UOffsetT
		if c.Value != "" {
			valueOffset = builder.CreateString(c.Value)
		}
		if c.Formula != "" {
			formulaOffset = builder.CreateString(c.Formula)
		}
		var stylePtrVal uint64
		if c.StyleId != nil {
			stylePtrVal = c.StyleId.ptr
		}
		builder.StartObject(4)
		builder.PrependUOffsetTSlot(0, valueOffset, 0)
		builder.PrependUOffsetTSlot(1, formulaOffset, 0)
		builder.PrependInt8Slot(2, int8(c.DataType), 0)
		if stylePtrVal != 0 {
			builder.PrependUint64Slot(3, stylePtrVal, 0)
		}
		offsets[i] = builder.EndObject()
	}
	return offsets
}

func fbsReadTableVector[T any](root flatbuffers.Table, vtableOff flatbuffers.VOffsetT, fn func(flatbuffers.Table) T) []T {
	o := flatbuffers.UOffsetT(root.Offset(vtableOff))
	if o == 0 {
		return nil
	}
	vecStart := root.Vector(o)
	vecLen := root.VectorLen(o)
	result := make([]T, vecLen)
	for i := 0; i < vecLen; i++ {
		elemPos := vecStart + flatbuffers.UOffsetT(i)*4
		tablePos := root.Indirect(elemPos)
		result[i] = fn(flatbuffers.Table{Bytes: root.Bytes, Pos: tablePos})
	}
	return result
}

func fbsReadString(t flatbuffers.Table, vtableOff flatbuffers.VOffsetT) string {
	o := flatbuffers.UOffsetT(t.Offset(vtableOff))
	if o == 0 {
		return ""
	}
	return string(t.ByteVector(o + t.Pos))
}
