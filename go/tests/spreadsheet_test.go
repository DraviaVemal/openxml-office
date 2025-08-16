package spreadsheet_2007

import "draviavemal_openxml_office/spreadsheet/spreadsheet_2007"

func BlankExcel() {
	excel, err := spreadsheet_2007.NewExcel("blank.xlsx")
	if err != nil {
		panic(err)
	}
	excel.SaveAs("Test")
}
