package openxml_office_go_test

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"draviavemal_openxml_office/spreadsheet/spreadsheet_2007"
)

// resultPath mirrors the C# test project layout (tests/cs/test_results).
const resultPath = "./test_results"

func TestMain(m *testing.M) {
	if err := os.MkdirAll(resultPath, 0o755); err != nil {
		panic(err)
	}
	os.Exit(m.Run())
}

// TestBlankFile is the Go parity of the C# `Spreadsheet.BlankFile` test:
// create an empty workbook and persist it.
func TestBlankFile(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("failed to create excel: %v", err)
	}

	out := filepath.Join(resultPath, "Blank-"+timestamp()+".xlsx")
	if err := excel.SaveAs(out); err != nil {
		t.Fatalf("failed to save excel: %v", err)
	}
}

func timestamp() string {
	return time.Now().Format("2006-01-02-15-04-05")
}
