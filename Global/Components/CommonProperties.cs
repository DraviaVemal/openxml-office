// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using A = DocumentFormat.OpenXml.Drawing;

namespace OpenXMLOffice.Global {
    /// <summary>
    /// Common Properties organised in one place to get inherited by child classes
    /// </summary>
    public class CommonProperties {
        #region Protected Constructors

        /// <summary>
        /// Class is only for inheritance purposes.
        /// </summary>
        protected CommonProperties() { }

        #endregion Protected Constructors

        #region Protected Methods

        /// <summary>
        /// Create Soild Fill XML Property
        /// </summary>
        /// <param name="FillColors">
        /// </param>
        /// <param name="index">
        /// </param>
        /// <returns>
        /// </returns>
        protected A.SolidFill CreateSolidFill(List<string> FillColors,int index) {
            if(FillColors.Count > 0) {
                return new A.SolidFill(new A.RgbColorModelHex() { Val = FillColors[index % FillColors.Count] });
            }
            return new A.SolidFill(new A.SchemeColor { Val = new A.SchemeColorValues($"accent{(index % 6) + 1}") });
        }

        #endregion Protected Methods
    }
}