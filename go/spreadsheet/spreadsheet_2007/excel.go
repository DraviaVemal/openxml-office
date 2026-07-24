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

type Excel struct {
	excelPtr uint64
}

func NewExcel() (Excel, error) {
	builder := flatbuffers.NewBuilder(0)

	builder.StartObject(1)
	builder.PrependBoolSlot(0, true, false)
	settingsOffset := builder.EndObject()

	builder.StartObject(2)
	builder.PrependUOffsetTSlot(1, settingsOffset, 0)
	createOffset := builder.EndObject()
	builder.Finish(createOffset)

	buffer := builder.FinishedBytes()

	var outBuffer *C.uint8_t
	var outBufferSize C.uintptr_t
	var outError *C.char

	code := C.excel_create(
		(*C.uint8_t)(unsafe.Pointer(&buffer[0])),
		C.uintptr_t(len(buffer)),
		&outBuffer,
		&outBufferSize,
		&outError,
	)
	if code != 0 {
		return Excel{}, ffiError(outError)
	}

	return Excel{excelPtr: readExcelPtr(outBuffer, outBufferSize)}, nil
}

func (excel *Excel) SaveAs(fileName string) error {
	if excel == nil {
		return errors.New("attempted to use a nil Excel object")
	}

	builder := flatbuffers.NewBuilder(0)
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

	code := C.excel_save_as(
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

func readExcelPtr(outBuffer *C.uint8_t, outBufferSize C.uintptr_t) uint64 {
	if outBuffer == nil {
		return 0
	}
	data := C.GoBytes(unsafe.Pointer(outBuffer), C.int(outBufferSize))
	C.free_buffer(outBuffer, outBufferSize)

	rootOffset := flatbuffers.GetUOffsetT(data)
	table := &flatbuffers.Table{Bytes: data, Pos: rootOffset}
	if fieldOffset := flatbuffers.UOffsetT(table.Offset(4)); fieldOffset != 0 {
		return table.GetUint64(fieldOffset + table.Pos)
	}
	return 0
}

// ffiError converts a C error string returned by the FFI into a Go error.
func ffiError(outError *C.char) error {
	if outError != nil {
		return errors.New(C.GoString(outError))
	}
	return errors.New("openxml-office ffi: unknown error")
}
