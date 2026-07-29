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

	public enum CellDataType
	{
		Auto = 0,
		Number = 1,
		Boolean = 2,
		String = 3,
		SharedString = 4,
		InlineString = 5,
		Error = 6,
	}

	public class CellProperty
	{
		public string Value { get; set; }
		public string Formula { get; set; }
		public CellDataType DataType { get; set; } = CellDataType.Auto;
		public StyleId StyleId { get; set; }
	}

	public enum NumberFormatValues
	{
		General,
		Integer,
		DecimalTwoPlaces,
		ThousandsSeparator,
		ThousandsSeparatorTwoDecimals,
		CurrencyNoDecimals,
		CurrencyNoDecimalsRed,
		CurrencyTwoDecimals,
		CurrencyTwoDecimalsRed,
		Percentage,
		PercentageTwoDecimals,
		Scientific,
		FractionOneDigit,
		FractionTwoDigits,
		DateMMDDYY,
		DateDMmmYY,
		DateDMmm,
		DateMmmYY,
		Time12Hour,
		Time12HourWithSeconds,
		Time24Hour,
		Time24HourWithSeconds,
		DateTimeMMDDYY,
		AccountingNoDecimals,
		AccountingNoDecimalsRed,
		AccountingTwoDecimals,
		AccountingTwoDecimalsRed,
		AccountingNegativeInParentheses,
		AccountingTwoDecimalsNegativeInParentheses,
		AccountingAlignedSymbols,
		AccountingAlignedSymbolsTwoDecimals,
		TimeMinutesSeconds,
		TimeHoursMinutesSeconds,
		ElapsedTimeWithFractions,
		ScientificOneDecimal,
		TextFormat,
		Custom,
	}

	public enum BorderStyleValues
	{
		None,
		Thin,
		Thick,
		Dotted,
		Double,
		Dashed,
		DashDot,
		DashDotDot,
		Medium,
		MediumDashed,
		MediumDashDot,
		MediumDashDotDot,
		SlantDashDot,
		Hair,
	}

	public enum ColorSettingTypeValues
	{
		Indexed,
		Theme,
		Rgb,
	}

	public enum HorizontalAlignmentValues
	{
		None,
		Left,
		Center,
		Right,
		Justify,
	}

	public enum VerticalAlignmentValues
	{
		None,
		Top,
		Middle,
		Bottom,
	}

	public class ColorSetting
	{
		public ColorSettingTypeValues ColorSettingType { get; set; } = ColorSettingTypeValues.Indexed;
		public string Value { get; set; } = string.Empty;
	}

	public class BorderSetting
	{
		public ColorSetting BorderColor { get; set; }
		public BorderStyleValues Style { get; set; } = BorderStyleValues.None;
	}

	public class CellStyleSetting
	{
		public NumberFormatValues NumberFormat { get; set; } = NumberFormatValues.General;
		public string CustomNumberFormat { get; set; }
		public BorderSetting BorderLeft { get; set; } = new BorderSetting();
		public BorderSetting BorderTop { get; set; } = new BorderSetting();
		public BorderSetting BorderRight { get; set; } = new BorderSetting();
		public BorderSetting BorderBottom { get; set; } = new BorderSetting();
		public BorderSetting BorderDiagonal { get; set; } = new BorderSetting();
		public string FontFamily { get; set; } = string.Empty;
		public byte FontSize { get; set; } = 0;
		public ColorSetting TextColor { get; set; } = new ColorSetting();
		public bool IsBold { get; set; } = false;
		public bool IsItalic { get; set; } = false;
		public bool IsUnderline { get; set; } = false;
		public bool IsDoubleUnderline { get; set; } = false;
		public bool IsWrapText { get; set; } = false;
		public string BackgroundColor { get; set; }
		public string ForegroundColor { get; set; }
		public HorizontalAlignmentValues HorizontalAlignment { get; set; } = HorizontalAlignmentValues.None;
		public VerticalAlignmentValues VerticalAlignment { get; set; } = VerticalAlignmentValues.None;
	}

	public class StyleId
	{
		private readonly ulong ffiStyleIdPtr;

		internal StyleId(ulong ffiStyleIdPtr)
		{
			this.ffiStyleIdPtr = ffiStyleIdPtr;
		}
	}

	public class ReferenceRange
	{
		public ushort ColumnStart { get; set; } = 1;
		public ushort ColumnEnd { get; set; } = 1;
		public uint RowStart { get; set; } = 1;
		public uint RowEnd { get; set; } = 1;
	}

	public class CellPackage
	{
		public string CellRef { get; set; }
		public uint RowIndex { get; set; }
		public ushort ColumnIndex { get; set; }
		public CellProperty CellProperty { get; set; }
	}

	public class HyperlinkInfo
	{
		public string Display { get; set; }
		public string Link { get; set; }
		public ReferenceRange Range { get; set; }
	}

	public class AnchorPosition
	{
		public ushort Column { get; set; } = 1;
		public ushort ColumnOffset { get; set; } = 0;
		public uint Row { get; set; } = 1;
		public uint RowOffset { get; set; } = 0;
	}

	public enum ImageType
	{
		JPEG = 0,
		PNG = 1,
		GIF = 2,
		BMP = 3,
		TIFF = 4,
	}

	public enum ExcelHyperlinkType
	{
		ExistingFile = 0,
		WebUrl = 1,
		TargetSheet = 2,
	}

	public class ExcelHyperlinkProperties
	{
		public string Display { get; set; }
		public ExcelHyperlinkType LinkType { get; set; } = ExcelHyperlinkType.WebUrl;
		public string Link { get; set; }
	}

	public class ExcelPictureSetting
	{
		public ImageType ImageType { get; set; } = ImageType.JPEG;
		public AnchorPosition From { get; set; } = new AnchorPosition();
		public AnchorPosition To { get; set; } = new AnchorPosition();
		public ExcelHyperlinkProperties HyperlinkProperties { get; set; }
	}
}