package spreadsheet_2007

/*
#cgo LDFLAGS: -L../../lib -ldraviavemal_openxml_office_ffi
#include <../../lib/headers.h>
*/
import "C"

import (
	ExcelPropertiesModel "draviavemal_openxml_office/openxml_office_fbs/spreadsheet_2007"
	"errors"
	"unsafe"

	flatbuffers "github.com/google/flatbuffers/go"
)

type Excel struct {
	excel_ptr unsafe.Pointer
}

func NewExcel(fileName string) (Excel, error) {
	builder := flatbuffers.NewBuilder(0)
	ExcelPropertiesModel.ExcelPropertiesModelStart(builder)
	ExcelPropertiesModel.ExcelPropertiesModelAddIsInMemory(builder, true)
	ExcelPropertiesModel.ExcelPropertiesModelAddIsEditable(builder, true)
	excelPropertiesModel := ExcelPropertiesModel.ExcelPropertiesModelEnd(builder)
	builder.Finish(excelPropertiesModel)
	buffer := builder.Bytes[builder.Head():]
	bufferPtr := (*C.uint8_t)(unsafe.Pointer(&buffer[0]))
	bufferSize := C.uintptr_t(len(buffer))
	var outExcel unsafe.Pointer
	var outError *C.char
	code := C.excel_create(C.CString(fileName), bufferPtr, bufferSize, &outExcel, &outError)
	if code != 0 {
		if outError != nil {
			return Excel{}, errors.New(C.GoString(outError))
		} else {
			return Excel{}, errors.New(C.GoString(outError))
		}
	}
	return Excel{
		excel_ptr: outExcel,
	}, nil
}

func (excel *Excel) SaveAs(fileName string) {
	if excel == nil {
		panic("attempted to use a nil Excel object")
	}
}
