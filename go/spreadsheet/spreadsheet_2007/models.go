package spreadsheet_2007

type CellDataType int8

const (
	CellDataTypeAuto         CellDataType = 0
	CellDataTypeNumber       CellDataType = 1
	CellDataTypeBoolean      CellDataType = 2
	CellDataTypeString       CellDataType = 3
	CellDataTypeShareString  CellDataType = 4
	CellDataTypeInlineString CellDataType = 5
	CellDataTypeError        CellDataType = 6
)

type NumberFormatValues int32

const (
	NumberFormatGeneral                                    NumberFormatValues = 0
	NumberFormatInteger                                    NumberFormatValues = 1
	NumberFormatDecimalTwoPlaces                           NumberFormatValues = 2
	NumberFormatThousandsSeparator                         NumberFormatValues = 3
	NumberFormatThousandsSeparatorTwoDecimals              NumberFormatValues = 4
	NumberFormatCurrencyNoDecimals                         NumberFormatValues = 5
	NumberFormatCurrencyNoDecimalsRed                      NumberFormatValues = 6
	NumberFormatCurrencyTwoDecimals                        NumberFormatValues = 7
	NumberFormatCurrencyTwoDecimalsRed                     NumberFormatValues = 8
	NumberFormatPercentage                                 NumberFormatValues = 9
	NumberFormatPercentageTwoDecimals                      NumberFormatValues = 10
	NumberFormatScientific                                 NumberFormatValues = 11
	NumberFormatFractionOneDigit                           NumberFormatValues = 12
	NumberFormatFractionTwoDigits                          NumberFormatValues = 13
	NumberFormatDateMMDDYY                                 NumberFormatValues = 14
	NumberFormatDateDMmmYY                                 NumberFormatValues = 15
	NumberFormatDateDMmm                                   NumberFormatValues = 16
	NumberFormatDateMmmYY                                  NumberFormatValues = 17
	NumberFormatTime12Hour                                 NumberFormatValues = 18
	NumberFormatTime12HourWithSeconds                      NumberFormatValues = 19
	NumberFormatTime24Hour                                 NumberFormatValues = 20
	NumberFormatTime24HourWithSeconds                      NumberFormatValues = 21
	NumberFormatDateTimeMMDDYY                             NumberFormatValues = 22
	NumberFormatAccountingNoDecimals                       NumberFormatValues = 23
	NumberFormatAccountingNoDecimalsRed                    NumberFormatValues = 24
	NumberFormatAccountingTwoDecimals                      NumberFormatValues = 25
	NumberFormatAccountingTwoDecimalsRed                   NumberFormatValues = 26
	NumberFormatAccountingNegativeInParentheses            NumberFormatValues = 27
	NumberFormatAccountingTwoDecimalsNegativeInParentheses NumberFormatValues = 28
	NumberFormatAccountingAlignedSymbols                   NumberFormatValues = 29
	NumberFormatAccountingAlignedSymbolsTwoDecimals        NumberFormatValues = 30
	NumberFormatTimeMinutesSeconds                         NumberFormatValues = 31
	NumberFormatTimeHoursMinutesSeconds                    NumberFormatValues = 32
	NumberFormatElapsedTimeWithFractions                   NumberFormatValues = 33
	NumberFormatScientificOneDecimal                       NumberFormatValues = 34
	NumberFormatTextFormat                                 NumberFormatValues = 35
	NumberFormatCustom                                     NumberFormatValues = 36
)

type BorderStyleValues int32

const (
	BorderStyleNone           BorderStyleValues = 0
	BorderStyleThin           BorderStyleValues = 1
	BorderStyleThick          BorderStyleValues = 2
	BorderStyleDotted         BorderStyleValues = 3
	BorderStyleDouble         BorderStyleValues = 4
	BorderStyleDashed         BorderStyleValues = 5
	BorderStyleDashDot        BorderStyleValues = 6
	BorderStyleDashDotDot     BorderStyleValues = 7
	BorderStyleMedium         BorderStyleValues = 8
	BorderStyleMediumDashed   BorderStyleValues = 9
	BorderStyleMediumDashDot  BorderStyleValues = 10
	BorderStyleMediumDashDotDot BorderStyleValues = 11
	BorderStyleSlantDashDot   BorderStyleValues = 12
)

type ColorSettingType int32

const (
	ColorSettingIndexed ColorSettingType = 0
	ColorSettingTheme   ColorSettingType = 1
	ColorSettingRgb     ColorSettingType = 2
)

type HorizontalAlignment int32

const (
	HorizontalAlignmentNone    HorizontalAlignment = 0
	HorizontalAlignmentLeft    HorizontalAlignment = 1
	HorizontalAlignmentCenter  HorizontalAlignment = 2
	HorizontalAlignmentRight   HorizontalAlignment = 3
	HorizontalAlignmentJustify HorizontalAlignment = 4
)

type VerticalAlignment int32

const (
	VerticalAlignmentNone   VerticalAlignment = 0
	VerticalAlignmentTop    VerticalAlignment = 1
	VerticalAlignmentMiddle VerticalAlignment = 2
	VerticalAlignmentBottom VerticalAlignment = 3
)

type ImageType int8

const (
	ImageTypeJPEG ImageType = 0
	ImageTypePNG  ImageType = 1
	ImageTypeGIF  ImageType = 2
	ImageTypeBMP  ImageType = 3
	ImageTypeTIFF ImageType = 4
)

type ExcelHyperlinkType int8

const (
	ExcelHyperlinkExistingFile ExcelHyperlinkType = 0
	ExcelHyperlinkWebURL       ExcelHyperlinkType = 1
	ExcelHyperlinkTargetSheet  ExcelHyperlinkType = 2
)

type ColorSetting struct {
	Type  ColorSettingType
	Value string
}

type BorderSetting struct {
	Color *ColorSetting
	Style BorderStyleValues
}

type CellStyleSetting struct {
	NumberFormat       NumberFormatValues
	CustomNumberFormat string
	BorderLeft         BorderSetting
	BorderTop          BorderSetting
	BorderRight        BorderSetting
	BorderBottom       BorderSetting
	BorderDiagonal     BorderSetting
	FontFamily         string
	FontSize           uint8
	TextColor          ColorSetting
	IsBold             bool
	IsItalic           bool
	IsUnderline        bool
	IsDoubleUnderline  bool
	IsWrapText         bool
	BackgroundColor    string
	ForegroundColor    string
	HorizontalAlign    HorizontalAlignment
	VerticalAlign      VerticalAlignment
}

type StyleId struct {
	ptr uint64
}

type ColumnProperties struct {
	Min     uint32
	Max     uint32
	Width   float32
	Hidden  bool
	BestFit bool
	StyleId *StyleId
}

type RowProperties struct {
	Height      float32
	Hidden      bool
	ThickTop    bool
	ThickBottom bool
	StyleId     *StyleId
}

type CellProperty struct {
	Value    string
	Formula  string
	DataType CellDataType
	StyleId  *StyleId
}

type ReferenceRange struct {
	ColumnStart uint16
	ColumnEnd   uint16
	RowStart    uint32
	RowEnd      uint32
}

type CellPackage struct {
	CellRef     string
	RowIndex    uint32
	ColumnIndex uint16
	Property    *CellProperty
}

type HyperlinkInfo struct {
	Display string
	Link    string
	Range   ReferenceRange
}

type AnchorPosition struct {
	Column       uint16
	ColumnOffset uint16
	Row          uint32
	RowOffset    uint32
}

type ExcelHyperlinkProperties struct {
	Display  string
	LinkType ExcelHyperlinkType
	Link     string
}

type ExcelPictureSetting struct {
	ImageType  ImageType
	From       AnchorPosition
	To         AnchorPosition
	Hyperlink  *ExcelHyperlinkProperties
}
