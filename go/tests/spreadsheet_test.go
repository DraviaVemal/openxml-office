package spreadsheet_2007

import (
	"testing"

	"draviavemal_openxml_office/spreadsheet/spreadsheet_2007"
)

func TestNewExcelBlank(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_blank.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestNewExcelFromFile(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel("test_files/basic_test.xlsx")
	if err != nil {
		t.Fatalf("NewExcel with file: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_from_file.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestAddSheetAndListSheetNames(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("TestSheet")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	names, err := excel.ListSheetNames()
	if err != nil {
		t.Fatalf("ListSheetNames: %v", err)
	}
	if len(names) == 0 {
		t.Error("expected at least one sheet name")
	}
	if err := excel.SaveAs("/tmp/go_test_add_sheet.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestSetActiveSheet(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("ActiveSheet")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SetActiveSheet("ActiveSheet"); err != nil {
		t.Fatalf("SetActiveSheet: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_active_sheet.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestWorkbookVisibilityOptions(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	if err := excel.SetVisibility(true); err != nil {
		t.Fatalf("SetVisibility: %v", err)
	}
	if err := excel.MinimizeWorkbook(true); err != nil {
		t.Fatalf("MinimizeWorkbook: %v", err)
	}
	if err := excel.HideSheetTabs(true); err != nil {
		t.Fatalf("HideSheetTabs: %v", err)
	}
	if err := excel.HideVerticalScroll(true); err != nil {
		t.Fatalf("HideVerticalScroll: %v", err)
	}
	if err := excel.HideHorizontalScroll(true); err != nil {
		t.Fatalf("HideHorizontalScroll: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_visibility.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestHideSheet(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("HiddenSheet")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.HideSheet("HiddenSheet"); err != nil {
		t.Fatalf("HideSheet: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_hide_sheet.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestSetAndListMergeCell(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("MergeTest")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	rng := spreadsheet_2007.ReferenceRange{ColumnStart: 1, ColumnEnd: 3, RowStart: 1, RowEnd: 2}
	if err := ws.SetMergeCell(rng); err != nil {
		t.Fatalf("SetMergeCell: %v", err)
	}
	merges, err := ws.ListMergeCell()
	if err != nil {
		t.Fatalf("ListMergeCell: %v", err)
	}
	if len(merges) == 0 {
		t.Error("expected at least one merge range")
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_merge_cell.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestRemoveMergeCell(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("RemoveMerge")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	rng := spreadsheet_2007.ReferenceRange{ColumnStart: 1, ColumnEnd: 3, RowStart: 1, RowEnd: 2}
	if err := ws.SetMergeCell(rng); err != nil {
		t.Fatalf("SetMergeCell: %v", err)
	}
	if err := ws.RemoveMergeCell(rng); err != nil {
		t.Fatalf("RemoveMergeCell: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_remove_merge.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestSetAndListHyperlinks(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("HyperlinkTest")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	rng := spreadsheet_2007.ReferenceRange{ColumnStart: 1, ColumnEnd: 1, RowStart: 1, RowEnd: 1}
	if err := ws.SetHyperlink("https://example.com", rng, "Example"); err != nil {
		t.Fatalf("SetHyperlink: %v", err)
	}
	links, err := ws.ListHyperlinks()
	if err != nil {
		t.Fatalf("ListHyperlinks: %v", err)
	}
	if len(links) == 0 {
		t.Error("expected at least one hyperlink")
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_hyperlinks.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestRemoveHyperlink(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("RemoveLink")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	rng := spreadsheet_2007.ReferenceRange{ColumnStart: 1, ColumnEnd: 1, RowStart: 1, RowEnd: 1}
	if err := ws.SetHyperlink("https://example.com", rng, ""); err != nil {
		t.Fatalf("SetHyperlink: %v", err)
	}
	if err := ws.RemoveHyperlink(rng); err != nil {
		t.Fatalf("RemoveHyperlink: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_remove_link.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestGetRangeCellProperties(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("CellProps")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	cells := []spreadsheet_2007.CellProperty{
		{Value: "Hello", DataType: spreadsheet_2007.CellDataTypeString},
		{Value: "42", DataType: spreadsheet_2007.CellDataTypeNumber},
	}
	if err := ws.SetCellRefValues("A1", cells); err != nil {
		t.Fatalf("SetCellRefValues: %v", err)
	}
	rng := spreadsheet_2007.ReferenceRange{ColumnStart: 1, ColumnEnd: 2, RowStart: 1, RowEnd: 1}
	props, err := ws.GetRangeCellProperties(rng)
	if err != nil {
		t.Fatalf("GetRangeCellProperties: %v", err)
	}
	if len(props) == 0 {
		t.Error("expected cell properties")
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_cell_props.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestAddPicture(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("PictureTest")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	setting := spreadsheet_2007.ExcelPictureSetting{
		ImageType: spreadsheet_2007.ImageTypeJPEG,
		From:      spreadsheet_2007.AnchorPosition{Column: 1, ColumnOffset: 0, Row: 1, RowOffset: 0},
		To:        spreadsheet_2007.AnchorPosition{Column: 5, ColumnOffset: 0, Row: 10, RowOffset: 0},
	}
	if err := ws.AddPicture("test_files/tom_and_jerry.jpg", setting); err != nil {
		t.Fatalf("AddPicture: %v", err)
	}
	if err := ws.Flush(); err != nil {
		t.Fatalf("Flush: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_picture.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

func TestDeleteSheet(t *testing.T) {
	excel, err := spreadsheet_2007.NewExcel()
	if err != nil {
		t.Fatalf("NewExcel: %v", err)
	}
	ws, err := excel.AddSheet("ToDelete")
	if err != nil {
		t.Fatalf("AddSheet: %v", err)
	}
	if err := ws.DeleteSheet(); err != nil {
		t.Fatalf("DeleteSheet: %v", err)
	}
	if err := excel.SaveAs("/tmp/go_test_delete_sheet.xlsx"); err != nil {
		t.Fatalf("SaveAs: %v", err)
	}
}

