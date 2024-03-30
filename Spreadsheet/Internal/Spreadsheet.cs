﻿// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using DocumentFormat.OpenXml;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Spreadsheet;

namespace OpenXMLOffice.Spreadsheet_2013
{
	internal class Spreadsheet : SpreadsheetCore
	{
		internal Spreadsheet(string filePath, SpreadsheetProperties? spreadsheetProperties) : base(filePath, spreadsheetProperties) { }

		internal Spreadsheet(string filePath, bool isEditable, SpreadsheetProperties? spreadsheetProperties) : base(filePath, isEditable, spreadsheetProperties) { }

		internal Spreadsheet(Stream stream, SpreadsheetProperties? spreadsheetProperties) : base(stream, spreadsheetProperties) { }

		internal Spreadsheet(Stream stream, bool isEditable, SpreadsheetProperties? spreadsheetProperties) : base(stream, isEditable, spreadsheetProperties) { }

		internal Worksheet AddSheet(string? sheetName)
		{
			if (sheetName == null)
			{
				sheetName = string.Format("Sheet{0}", GetMaxSheetId() + 1);
			}
			if (CheckIfSheetNameExist(sheetName))
			{
				throw new ArgumentException("Sheet with name already exist.");
			}
			// Check If Sheet Already exist
			WorksheetPart worksheetPart = GetWorkbookPart().AddNewPart<WorksheetPart>();
			Sheet sheet = new()
			{
				Id = GetWorkbookPart().GetIdOfPart(worksheetPart),
				SheetId = GetMaxSheetId() + 1,
				Name = sheetName
			};
			GetSheets().Append(sheet);
			worksheetPart.Worksheet = new DocumentFormat.OpenXml.Spreadsheet.Worksheet(new SheetData());
			return new Worksheet(worksheetPart.Worksheet, sheet);
		}

		internal int? GetSheetId(string sheetName)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Name == sheetName) as Sheet;
			if (sheet != null)
			{
				return int.Parse(sheet.Id!.Value!);
			}
			return null;
		}

		internal static uint GetStyleId(CellStyleSetting CellStyleSetting)
		{
			return Styles.Instance.GetCellStyleId(CellStyleSetting);
		}

		internal string? GetSheetName(string sheetId)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Id?.Value == sheetId) as Sheet;
			if (sheet != null)
			{
				return sheet.Name;
			}
			return null;
		}

		internal Worksheet? GetWorksheet(string sheetName)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Name == sheetName) as Sheet;
			if (sheet == null) { return null; }
			if (GetWorkbookPart().GetPartById(sheet.Id!) is not WorksheetPart worksheetPart) { return null; }
			return new Worksheet(worksheetPart.Worksheet, sheet);
		}

		internal bool RemoveSheet(string sheetName)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Name == sheetName) as Sheet;
			if (sheet != null)
			{
				if (GetWorkbookPart().GetPartById(sheet.Id!) is WorksheetPart worksheetPart)
				{
					GetWorkbookPart().DeletePart(worksheetPart);
				}
				sheet.Remove();
				return true;
			}
			return false;
		}

		internal bool RemoveSheet(int sheetId)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Id?.Value == sheetId.ToString()) as Sheet;
			if (sheet != null)
			{
				if (GetWorkbookPart().GetPartById(sheet.Id!) is WorksheetPart worksheetPart)
				{
					GetWorkbookPart().DeletePart(worksheetPart);
				}
				sheet.Remove();
				return true;
			}
			return false;
		}

		internal bool RenameSheet(string oldSheetName, string newSheetName)
		{
			if (CheckIfSheetNameExist(newSheetName))
			{
				throw new ArgumentException("New Sheet with name already exist.");
			}
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Name == oldSheetName) as Sheet;
			if (sheet == null)
			{
				return false;
			}
			sheet.Name = newSheetName;
			return true;
		}

		internal bool RenameSheet(int sheetId, string newSheetName)
		{
			if (CheckIfSheetNameExist(newSheetName))
			{
				throw new ArgumentException("New Sheet with name already exist.");
			}
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Id?.Value == sheetId.ToString()) as Sheet;
			if (sheet == null)
			{
				return false;
			}
			sheet.Name = newSheetName;
			return true;
		}

		internal void Save()
		{
			UpdateStyle();
			UpdateSharedString();
			spreadsheetDocument.Save();
			spreadsheetDocument.Dispose();
		}

		internal void SaveAs(string filePath)
		{
			throw new NotImplementedException();
		}

		private bool CheckIfSheetNameExist(string sheetName)
		{
			Sheet? sheet = GetSheets().FirstOrDefault(sheet => (sheet as Sheet)?.Name == sheetName) as Sheet;
			return sheet != null;
		}

		private UInt32Value GetMaxSheetId()
		{
			return GetSheets().Max(sheet => (sheet as Sheet)?.SheetId) ?? 0;
		}


	}
}
