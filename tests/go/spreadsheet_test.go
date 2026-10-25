package openxml_office_go_test

import (
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/DraviaVemal/openxml-office/go/spreadsheet/spreadsheet_2007"
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

func TestWorkbookSettingsBatch(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("failed to create excel: %v", err)
	}
	worksheet, err := excel.AddSheet("Settings")
	if err != nil {
		t.Fatalf("failed to add worksheet: %v", err)
	}
	if err := worksheet.Flush(); err != nil {
		t.Fatalf("failed to flush worksheet: %v", err)
	}
	visible := false
	minimized := true
	hideTab := false
	if err := excel.SetWorkbookSettings(spreadsheet_2007.WorkbookSettings{
		ActiveSheet:          "Settings",
		IsVisible:            &visible,
		IsMinimized:          &minimized,
		HideTab:              &hideTab,
		HideVerticalScroll:   &hideTab,
		HideHorizontalScroll: &hideTab,
		HideSheet:            "Settings",
	}); err != nil {
		t.Fatalf("failed to set workbook settings: %v", err)
	}
	out := filepath.Join(resultPath, "WorkbookSettings-"+timestamp()+".xlsx")
	if err := excel.SaveAs(out); err != nil {
		t.Fatalf("failed to save workbook: %v", err)
	}
}

func TestStyleIdIsWorkbookScoped(t *testing.T) {
	firstExcel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("failed to create first excel: %v", err)
	}
	styleID, err := firstExcel.GetStyleId(spreadsheet_2007.CellStyleSetting{})
	if err != nil {
		t.Fatalf("failed to get style ID: %v", err)
	}
	firstWorksheet, err := firstExcel.AddSheet("First")
	if err != nil {
		t.Fatalf("failed to add first worksheet: %v", err)
	}
	if err := firstWorksheet.SetCellIndexValues(1, 1, []spreadsheet_2007.CellProperty{{
		Value:   "styled",
		StyleId: &styleID,
	}}); err != nil {
		t.Fatalf("failed to write a cell with its workbook style: %v", err)
	}
	packages, err := firstWorksheet.GetRangeCellProperties(spreadsheet_2007.ReferenceRange{
		ColumnStart: 1,
		ColumnEnd:   1,
		RowStart:    1,
		RowEnd:      1,
	})
	if err != nil {
		t.Fatalf("failed to read styled cell: %v", err)
	}
	if len(packages) != 1 || packages[0].Property == nil || packages[0].Property.StyleId == nil {
		t.Fatal("expected the styled cell to return its opaque style handle")
	}
	secondExcel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("failed to create second excel: %v", err)
	}
	worksheet, err := secondExcel.AddSheet("Other")
	if err != nil {
		t.Fatalf("failed to add worksheet: %v", err)
	}
	err = worksheet.SetCellIndexValues(1, 1, []spreadsheet_2007.CellProperty{{
		Value:   "value",
		StyleId: &styleID,
	}})
	if err == nil {
		t.Fatal("expected a StyleId from another workbook to be rejected")
	}
}

func timestamp() string {
	return time.Now().Format("2006-01-02-15-04-05")
}
