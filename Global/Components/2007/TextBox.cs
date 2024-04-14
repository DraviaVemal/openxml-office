// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.
using A = DocumentFormat.OpenXml.Drawing;
using P = DocumentFormat.OpenXml.Presentation;
namespace OpenXMLOffice.Global_2007
{
	/// <summary>
	/// Represents Textbox base class to build on
	/// </summary>
	public class TextBoxBase : CommonProperties
	{
		private readonly TextBoxSetting textBoxSetting;
		private P.Shape openXMLShape;
		/// <summary>
		/// Create Textbox with provided settings
		/// </summary>
		public TextBoxBase(TextBoxSetting TextBoxSetting)
		{
			textBoxSetting = TextBoxSetting;
			CreateTextBox();
		}
		/// <summary>
		/// Get Textbox Shape
		/// </summary>
		public P.Shape GetTextBoxBaseShape()
		{
			return openXMLShape;
		}
		/// <summary>
		/// Update Textbox Position
		/// </summary>
		public void UpdatePosition(uint X, uint Y)
		{
			textBoxSetting.x = X;
			textBoxSetting.y = Y;
			if (openXMLShape != null)
			{
				openXMLShape.ShapeProperties.Transform2D = new A.Transform2D
				{
					Offset = new A.Offset { X = textBoxSetting.x, Y = textBoxSetting.y },
					Extents = new A.Extents { Cx = textBoxSetting.width, Cy = textBoxSetting.height }
				};
			}
		}
		/// <summary>
		/// Update Textbox Size
		/// </summary>
		public void UpdateSize(uint Width, uint Height)
		{
			textBoxSetting.width = Width;
			textBoxSetting.height = Height;
			if (openXMLShape != null)
			{
				openXMLShape.ShapeProperties.Transform2D = new A.Transform2D
				{
					Offset = new A.Offset { X = textBoxSetting.x, Y = textBoxSetting.y },
					Extents = new A.Extents { Cx = textBoxSetting.width, Cy = textBoxSetting.height }
				};
			}
		}
		/// <summary>
		///
		/// </summary>
		public void UpdateShapeStyle(P.ShapeStyle shapeStyle)
		{
			GetTextBoxBaseShape().ShapeStyle = shapeStyle;
		}
		private P.Shape CreateTextBox()
		{
			SolidFillModel solidFillModel = new SolidFillModel()
			{
				schemeColorModel = new SchemeColorModel()
				{
					themeColorValues = ThemeColorValues.TEXT_1
				}
			};
			if (textBoxSetting.textColor != null)
			{
				solidFillModel.hexColor = textBoxSetting.textColor;
				solidFillModel.schemeColorModel = null;
			}
			P.ShapeProperties ShapeProperties = new P.ShapeProperties(
							new A.Transform2D(
								new A.Offset { X = textBoxSetting.x, Y = textBoxSetting.y },
								new A.Extents { Cx = textBoxSetting.width, Cy = textBoxSetting.height }),
							new A.PresetGeometry(new A.AdjustValueList()) { Preset = A.ShapeTypeValues.Rectangle });
			if (textBoxSetting.shapeBackground != null)
			{
				ShapeProperties.Append(CreateSolidFill(new SolidFillModel() { hexColor = textBoxSetting.shapeBackground }));
			}
			else
			{
				ShapeProperties.Append(new A.NoFill());
			}
			openXMLShape = new P.Shape()
			{
				NonVisualShapeProperties = new P.NonVisualShapeProperties(
				new P.NonVisualDrawingProperties()
				{
					Id = 10,
					Name = "Text Box"
				},
				new P.NonVisualShapeDrawingProperties(),
				new P.ApplicationNonVisualDrawingProperties()),
				ShapeProperties = ShapeProperties,
				TextBody = new P.TextBody(
						new A.BodyProperties(),
						new A.ListStyle(),
						CreateDrawingParagraph(new DrawingParagraphModel()
						{
							paragraphPropertiesModel = new ParagraphPropertiesModel()
							{
								horizontalAlignment = textBoxSetting.horizontalAlignment
							},
							drawingRun = new DrawingRunModel()
							{
								text = textBoxSetting.text,
								textHightlight = textBoxSetting.textBackground,
								drawingRunProperties = new DrawingRunPropertiesModel()
								{
									solidFill = solidFillModel,
									fontFamily = textBoxSetting.fontFamily,
									fontSize = textBoxSetting.fontSize,
									isBold = textBoxSetting.isBold,
									isItalic = textBoxSetting.isItalic,
									underline = textBoxSetting.isUnderline ? UnderLineValues.SINGLE : UnderLineValues.NONE,
								}
							}
						})),
			};
			return openXMLShape;
		}
	}
}
