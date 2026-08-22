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

	fbs "draviavemal_openxml_office/internal/openxml_office_fbs/spreadsheet"

	flatbuffers "github.com/google/flatbuffers/go"
)

type Excel struct {
	excelPtr uint64
}

// NewExcel creates a new blank workbook or opens an existing file when fileName is provided.
func NewExcel(fileName ...string) (Excel, error) {
	builder := flatbuffers.NewBuilder(256)

	var fileNameOffset flatbuffers.UOffsetT
	hasFile := len(fileName) > 0 && fileName[0] != ""
	if hasFile {
		fileNameOffset = builder.CreateString(fileName[0])
	}

	builder.StartObject(1)
	builder.PrependBoolSlot(0, true, false)
	settingsOffset := builder.EndObject()

	builder.StartObject(2)
	if hasFile {
		builder.PrependUOffsetTSlot(0, fileNameOffset, 0)
	}
	builder.PrependUOffsetTSlot(1, settingsOffset, 0)
	createOffset := builder.EndObject()
	builder.Finish(createOffset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_create(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return Excel{}, ffiError(outError)
	}
	return Excel{excelPtr: readU64Response(outBuffer, outBufferSize)}, nil
}

// AddSheet adds a new worksheet with an optional name.
func (excel *Excel) AddSheet(sheetName ...string) (*Worksheet, error) {
	if excel == nil {
		return nil, errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(128)

	var nameOffset flatbuffers.UOffsetT
	hasName := len(sheetName) > 0 && sheetName[0] != ""
	if hasName {
		nameOffset = builder.CreateString(sheetName[0])
	}

	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	if hasName {
		builder.PrependUOffsetTSlot(1, nameOffset, 0)
	}
	offset := builder.EndObject()
	builder.Finish(offset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_add_sheet(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return nil, ffiError(outError)
	}
	return &Worksheet{worksheetPtr: readU64Response(outBuffer, outBufferSize)}, nil
}

// GetWorksheet retrieves an existing worksheet by name.
func (excel *Excel) GetWorksheet(sheetName string) (*Worksheet, error) {
	if excel == nil {
		return nil, errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(128)
	nameOffset := builder.CreateString(sheetName)

	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, nameOffset, 0)
	offset := builder.EndObject()
	builder.Finish(offset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_get_sheet(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return nil, ffiError(outError)
	}
	return &Worksheet{worksheetPtr: readU64Response(outBuffer, outBufferSize)}, nil
}

// RenameSheet renames an existing worksheet.
func (excel *Excel) RenameSheet(oldName, newName string) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(128)
	oldOffset := builder.CreateString(oldName)
	newOffset := builder.CreateString(newName)

	builder.StartObject(3)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, oldOffset, 0)
	builder.PrependUOffsetTSlot(2, newOffset, 0)
	offset := builder.EndObject()
	builder.Finish(offset)

	buffer := builder.FinishedBytes()
	var outError *C.char

	code := C.Excel_rename_sheet(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outError,
	)
	if code != 0 {
		return ffiError(outError)
	}
	return nil
}

// ListSheetNames returns the names of all worksheets in the workbook.
func (excel *Excel) ListSheetNames() ([]string, error) {
	if excel == nil {
		return nil, errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)

	builder.StartObject(1)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	offset := builder.EndObject()
	builder.Finish(offset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_list_sheet_name(
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
	resp := fbs.GetRootAsExcel_list_sheet_return(data, 0)
	names := make([]string, resp.SheetNamesLength())
	for i := range names {
		names[i] = string(resp.SheetNames(i))
	}
	return names, nil
}

// GetStyleId creates or retrieves a style ID for the given cell style settings.
func (excel *Excel) GetStyleId(setting CellStyleSetting) (StyleId, error) {
	if excel == nil {
		return StyleId{}, errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(512)

	buildColorSetting := func(c ColorSetting) flatbuffers.UOffsetT {
		valOffset := builder.CreateString(c.Value)
		builder.StartObject(2)
		builder.PrependInt32Slot(0, int32(c.Type), 0)
		builder.PrependUOffsetTSlot(1, valOffset, 0)
		return builder.EndObject()
	}

	buildBorder := func(b BorderSetting) flatbuffers.UOffsetT {
		var colorOffset flatbuffers.UOffsetT
		if b.Color != nil {
			colorOffset = buildColorSetting(*b.Color)
		} else {
			colorOffset = buildColorSetting(ColorSetting{})
		}
		builder.StartObject(2)
		builder.PrependUOffsetTSlot(0, colorOffset, 0)
		builder.PrependInt32Slot(1, int32(b.Style), 0)
		return builder.EndObject()
	}

	borderLeft := buildBorder(setting.BorderLeft)
	borderTop := buildBorder(setting.BorderTop)
	borderRight := buildBorder(setting.BorderRight)
	borderBottom := buildBorder(setting.BorderBottom)
	borderDiagonal := buildBorder(setting.BorderDiagonal)
	textColor := buildColorSetting(setting.TextColor)

	fontFamilyOffset := builder.CreateString(setting.FontFamily)
	var customFmtOffset, bgColorOffset, fgColorOffset flatbuffers.UOffsetT
	if setting.CustomNumberFormat != "" {
		customFmtOffset = builder.CreateString(setting.CustomNumberFormat)
	}
	if setting.BackgroundColor != "" {
		bgColorOffset = builder.CreateString(setting.BackgroundColor)
	}
	if setting.ForegroundColor != "" {
		fgColorOffset = builder.CreateString(setting.ForegroundColor)
	}

	builder.StartObject(19)
	builder.PrependInt32Slot(0, int32(setting.NumberFormat), 0)
	builder.PrependUOffsetTSlot(1, customFmtOffset, 0)
	builder.PrependUOffsetTSlot(2, borderLeft, 0)
	builder.PrependUOffsetTSlot(3, borderTop, 0)
	builder.PrependUOffsetTSlot(4, borderRight, 0)
	builder.PrependUOffsetTSlot(5, borderBottom, 0)
	builder.PrependUOffsetTSlot(6, borderDiagonal, 0)
	builder.PrependUOffsetTSlot(7, fontFamilyOffset, 0)
	builder.PrependUint8Slot(8, setting.FontSize, 0)
	builder.PrependUOffsetTSlot(9, textColor, 0)
	builder.PrependBoolSlot(10, setting.IsBold, false)
	builder.PrependBoolSlot(11, setting.IsItalic, false)
	builder.PrependBoolSlot(12, setting.IsUnderline, false)
	builder.PrependBoolSlot(13, setting.IsDoubleUnderline, false)
	builder.PrependBoolSlot(14, setting.IsWrapText, false)
	builder.PrependUOffsetTSlot(15, bgColorOffset, 0)
	builder.PrependUOffsetTSlot(16, fgColorOffset, 0)
	builder.PrependInt32Slot(17, int32(setting.HorizontalAlign), 0)
	builder.PrependInt32Slot(18, int32(setting.VerticalAlign), 0)
	styleSettingOffset := builder.EndObject()

	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, styleSettingOffset, 0)
	offset := builder.EndObject()
	builder.Finish(offset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_get_style_id(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return StyleId{}, ffiError(outError)
	}
	return StyleId{ptr: readU64Response(outBuffer, outBufferSize)}, nil
}

// SaveAs saves the workbook to the specified file path.
func (excel *Excel) SaveAs(fileName string) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(256)
	fileNameOffset := builder.CreateString(fileName)

	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, fileNameOffset, 0)
	saveOffset := builder.EndObject()
	builder.Finish(saveOffset)

	buffer := builder.FinishedBytes()
	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.Excel_save_as(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return ffiError(outError)
	}
	if outBuffer != nil {
		C.free_buffer(outBuffer, outBufferSize)
	}
	return nil
}

// SetActiveSheet sets the sheet that is active when the workbook opens.
func (excel *Excel) SetActiveSheet(sheetName string) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(128)
	v := builder.CreateString(sheetName)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, v, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_set_active_sheet((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// SetVisibility shows or hides the workbook window.
func (excel *Excel) SetVisibility(isVisible bool) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependBoolSlot(1, isVisible, false)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_set_visibility((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// MinimizeWorkbook minimizes or restores the workbook window.
func (excel *Excel) MinimizeWorkbook(isMinimized bool) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependBoolSlot(1, isMinimized, false)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_minimize_workbook((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// HideSheetTabs shows or hides the sheet tab bar.
func (excel *Excel) HideSheetTabs(hide bool) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependBoolSlot(1, hide, false)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_hide_sheet_tabs((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// HideVerticalScroll shows or hides the vertical scroll bar.
func (excel *Excel) HideVerticalScroll(hide bool) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependBoolSlot(1, hide, false)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_hide_vertical_scroll((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// HideHorizontalScroll shows or hides the horizontal scroll bar.
func (excel *Excel) HideHorizontalScroll(hide bool) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(64)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependBoolSlot(1, hide, false)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_hide_horizontal_scroll((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

// HideSheet hides the named worksheet.
func (excel *Excel) HideSheet(sheetName string) error {
	if excel == nil {
		return errors.New("nil Excel")
	}
	builder := flatbuffers.NewBuilder(128)
	v := builder.CreateString(sheetName)
	builder.StartObject(2)
	builder.PrependUint64Slot(0, excel.excelPtr, 0)
	builder.PrependUOffsetTSlot(1, v, 0)
	builder.Finish(builder.EndObject())
	buf := builder.FinishedBytes()
	var outError *C.char
	if code := C.Excel_hide_sheet((*C.uint8_t)(unsafe.Pointer(&buf[0])), C.uintptr_t(len(buf)), &outError); code != 0 {
		return ffiError(outError)
	}
	return nil
}

func readU64Response(outBuffer *C.uint8_t, outBufferSize C.uintptr_t) uint64 {
	if outBuffer == nil {
		return 0
	}
	data := consumeBuffer(outBuffer, outBufferSize)
	rootOffset := flatbuffers.GetUOffsetT(data)
	table := &flatbuffers.Table{Bytes: data, Pos: rootOffset}
	if fieldOffset := flatbuffers.UOffsetT(table.Offset(4)); fieldOffset != 0 {
		return table.GetUint64(fieldOffset + table.Pos)
	}
	return 0
}

func consumeBuffer(outBuffer *C.uint8_t, outBufferSize C.uintptr_t) []byte {
	data := C.GoBytes(unsafe.Pointer(outBuffer), C.int(outBufferSize))
	C.free_buffer(outBuffer, outBufferSize)
	return data
}

func ffiError(outError *C.char) error {
	if outError != nil {
		return errors.New(C.GoString(outError))
	}
	return errors.New("openxml-office ffi: unknown error")
}
