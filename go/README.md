# OpenXML Office Go

[![Go Reference](https://pkg.go.dev/badge/github.com/DraviaVemal/openxml-office/go/v4.svg)](https://pkg.go.dev/github.com/DraviaVemal/openxml-office/go/v4)

Go bindings for OpenXML Office, backed by the shared Rust core through CGO. The module currently exposes spreadsheet and presentation packages.

## Install

Requires Go 1.22.5 or later, CGO, and a C toolchain for the target platform.

```sh
go get github.com/DraviaVemal/openxml-office/go/v4@latest
```

The module is published from the repository's `go/` subdirectory. Go release tags use the `go/` subdirectory prefix, for example `go/v4.0.0-alpha.26`.

## Quick Start

```go
package main

import "github.com/DraviaVemal/openxml-office/go/v4/spreadsheet/spreadsheet_2007"

func main() {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		panic(err)
	}

	worksheet, err := excel.AddSheet("Summary")
	if err != nil {
		panic(err)
	}
	if err := worksheet.SetCellRefValues("A1", []spreadsheet_2007.CellProperty{
		{Value: "Revenue", DataType: spreadsheet_2007.CellDataTypeString},
		{Value: "42", DataType: spreadsheet_2007.CellDataTypeNumber},
	}); err != nil {
		panic(err)
	}
	if err := worksheet.Flush(); err != nil {
		panic(err)
	}
	if err := excel.SaveAs("report.xlsx"); err != nil {
		panic(err)
	}
}
```

## Packages

- [Spreadsheet API](https://pkg.go.dev/github.com/DraviaVemal/openxml-office/go/v4/spreadsheet/spreadsheet_2007)
- [Presentation API](https://pkg.go.dev/github.com/DraviaVemal/openxml-office/go/v4/presentation/presentation_2007)
- [Source and tests](https://github.com/DraviaVemal/openxml-office/tree/release/go)

The archive implementation is shared across OpenXML document formats. Parts are loaded on demand, unchanged ZIP entries are copied on save, and edited parts are compressed in parallel. The Go examples and benchmark workloads currently focus on spreadsheets.

## Platforms

Linux, macOS and Windows. CGO must be enabled, and builds need a C compiler and the native library for the target platform.

## License

Dual-licensed under [AGPL-3.0](../LICENSE) or a [commercial license](../COMMERCIAL-LICENSE.md).
