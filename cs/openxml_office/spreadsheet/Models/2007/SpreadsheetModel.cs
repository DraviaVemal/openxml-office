// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using draviavemal.openxml_office.global_2007;

namespace draviavemal.openxml_office.spreadsheet_2007
{
	/// <summary>
	/// Represents the properties of a column in a worksheet.
	/// </summary>
	public class ExcelProperties
	{
		/// <summary>
		/// 
		/// </summary>
		public bool IsInMemory = false;
		/// <summary>
		/// Spreadsheet settings
		/// </summary>
		public ExcelSettings settings = new();
		/// <summary>
		/// Spreadsheet theme settings
		/// </summary>
		public ThemePallet theme = new();
		/// <summary>
		/// Add Meta Data Details to File
		/// </summary>
		public CorePropertiesModel coreProperties = new();
	}
	/// <summary>
	/// Represents the settings of a spreadsheet.
	/// </summary>
	public class ExcelSettings
	{
	}
	internal class ExcelInfo
	{
		public bool isEditable = true;
		public bool isExistingFile;
	}
}
