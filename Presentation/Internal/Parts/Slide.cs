using DocumentFormat.OpenXml.Packaging;
using A = DocumentFormat.OpenXml.Drawing;
using P = DocumentFormat.OpenXml.Presentation;

namespace OpenXMLOffice.Presentation
{
    public class Slide
    {
        #region Private Fields

        private readonly P.Slide OpenXMLSlide = new();

        #endregion Private Fields

        #region Internal Constructors

        internal Slide(P.Slide? OpenXMLSlide = null)
        {
            if (OpenXMLSlide != null)
            {
                this.OpenXMLSlide = OpenXMLSlide;
            }
            else
            {
                CommonSlideData commonSlideData = new(PresentationConstants.CommonSlideDataType.SLIDE, PresentationConstants.SlideLayoutType.BLANK);
                this.OpenXMLSlide.CommonSlideData = commonSlideData.GetCommonSlideData();
                this.OpenXMLSlide.ColorMapOverride = new P.ColorMapOverride()
                {
                    MasterColorMapping = new A.MasterColorMapping()
                };
                this.OpenXMLSlide.AddNamespaceDeclaration("a", "http://schemas.openxmlformats.org/drawingml/2006/main");
                this.OpenXMLSlide.AddNamespaceDeclaration("r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships");
            }
        }

        #endregion Internal Constructors

        #region Public Methods

        public Chart AddChart()
        {
            return new Chart(this);
        }

        public IEnumerable<Shape> FindShapeByText(string searchText)
        {
            IEnumerable<P.Shape> searchResults = GetCommonSlideData().ShapeTree!.Elements<P.Shape>().Where(shape =>
            {
                return shape.InnerText == searchText;
            });
            return searchResults.Select(shape =>
            {
                return new Shape(shape);
            });
        }

        #endregion Public Methods

        #region Internal Methods

        internal string GetNextSlideRelationId()
        {
            return string.Format("rId{0}", GetSlidePart().Parts.Count() + 1);
        }

        internal P.Slide GetSlide()
        {
            return OpenXMLSlide;
        }

        internal SlidePart GetSlidePart()
        {
            return OpenXMLSlide.SlidePart!;
        }

        #endregion Internal Methods

        #region Private Methods

        private P.CommonSlideData GetCommonSlideData()
        {
            return OpenXMLSlide.CommonSlideData!;
        }

        #endregion Private Methods
    }
}