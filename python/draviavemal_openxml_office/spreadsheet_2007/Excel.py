import draviavemal_openxml_office_ffi;
from cffi import FFI
from openxml_office_fbs.spreadsheet_2007 import ExcelPropertiesModel;
from flatbuffers import Builder;

class Excel:
    def __init__(self):
        builder = Builder(1024)
        ExcelPropertiesModel.ExcelPropertiesModelStart(builder)
        ExcelPropertiesModel.AddIsInMemory(builder,True)
        ExcelPropertiesModel.ExcelPropertiesModelEnd(builder)
        ffi = FFI()
        # Create a new pointer for the Excel struct and error message
        file_name = ffi.new("const char[]", b"example.xlsx")
        buffer = ffi.new("const uint8_t[]", b"some binary data")
        buffer_size = len(buffer)
        out_excel = ffi.new("void**")  # Pointer to the Excel struct (void**)
        out_error = ffi.new("const char**")  # Pointer to error message (const char**)

        # Call the function
        result = draviavemal_openxml_office_ffi.excel_create(file_name, buffer, buffer_size, out_excel, out_error)
        pass
    
    def SaveAs(self):
        pass