// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using OpenXMLOffice.Global_2007;
using XDR = DocumentFormat.OpenXml.Drawing.Spreadsheet;

namespace OpenXMLOffice.Spreadsheet_2007
{
    /// <summary>
    /// Shape Class For Presentation shape manipulation
    /// </summary>
    public class Shape : CommonProperties
    {
        private readonly XDR.Shape openXMLShape = new XDR.Shape();
        internal Shape(XDR.Shape shape = null)
        {
            if (shape != null)
            {
                openXMLShape = shape;
            }
        }

        /// <summary>
        /// Remove Found Shape
        /// </summary>
        public void RemoveShape()
        {
            openXMLShape.Remove();
        }

        internal Shape AddLine<ApplicationSpecificSetting, LineColorOption>(ShapeLineModel<ApplicationSpecificSetting, LineColorOption> lineModel)
        where ApplicationSpecificSetting : class, ISizeAndPosition, new()
        where LineColorOption : class, IColorOptions, new()
        {
            return this;
        }

        internal Shape AddRectangle<ApplicationSpecificSetting, LineColorOption, FillColorOption>(ShapeRectangleModel<ApplicationSpecificSetting, LineColorOption, FillColorOption> rectangleModel)
        where ApplicationSpecificSetting : class, ISizeAndPosition, new()
        where LineColorOption : class, IColorOptions, new()
        where FillColorOption : class, IColorOptions, new()
        {
            return this;
        }

        internal Shape AddArrow<ApplicationSpecificSetting, LineColorOption, FillColorOption>(ShapeArrowModel<ApplicationSpecificSetting, LineColorOption, FillColorOption> arrowModel)
        where ApplicationSpecificSetting : class, ISizeAndPosition, new()
        where LineColorOption : class, IColorOptions, new()
        where FillColorOption : class, IColorOptions, new()
        {
            return this;
        }
    }
}