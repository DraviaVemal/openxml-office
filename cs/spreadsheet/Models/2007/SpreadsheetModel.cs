// Copyright (c) DraviaVemal. This project is dual-licensed. See License in the project root.

namespace draviavemal.openxml_office.spreadsheet_2007
{
	/// <summary>
	/// Represents the properties of a column in a worksheet.
	/// </summary>
	public class ExcelProperties
	{
		public bool isEditable = true;
	}

	public class ColumnProperties
	{
		public uint Min { get; set; }
		public uint Max { get; set; }
		public float Width { get; set; }
		public bool Hidden { get; set; }
		public bool BestFit { get; set; }
	}

	public class RowProperties
	{
		public float Height { get; set; }
		public bool Hidden { get; set; }
		public bool TickTop { get; set; }
		public bool ThickBottom { get; set; }
	}

	public class CellProperty
	{
		public string Value { get; set; }
		public string Formula { get; set; }
		public string DataType { get; set; }
		public StyleId StyleId { get; set; }
	}

	public class StyleId
	{
		private readonly ulong ffiStyleIdPtr;

		internal StyleId(ulong ffiStyleIdPtr)
		{
			this.ffiStyleIdPtr = ffiStyleIdPtr;
		}
	}
}