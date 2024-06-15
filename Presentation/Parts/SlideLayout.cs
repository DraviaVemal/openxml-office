// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using DocumentFormat.OpenXml.Packaging;
using A = DocumentFormat.OpenXml.Drawing;
using P = DocumentFormat.OpenXml.Presentation;
namespace OpenXMLOffice.Presentation_2007
{
	internal class SlideLayout
	{
		private readonly CommonSlideData commonSlideData = new CommonSlideData(PresentationConstants.CommonSlideDataType.SLIDE_LAYOUT, PresentationConstants.SlideLayoutType.BLANK);
		private readonly P.SlideLayout openXMLSlideLayout = new P.SlideLayout()
		{
			Type = P.SlideLayoutValues.Text
		}; public SlideLayout()
		{
			CreateSlideLayout();
		}
		public P.SlideLayout GetSlideLayout()
		{
			return openXMLSlideLayout;
		}
		public string UpdateRelationship(OpenXmlPart openXmlPart, string RelationshipId)
		{
			return openXMLSlideLayout.SlideLayoutPart.CreateRelationshipToPart(openXmlPart, RelationshipId);
		}
		private void CreateSlideLayout()
		{
			var unused1 = openXMLSlideLayout.AppendChild(commonSlideData.GetCommonSlideData());
			openXMLSlideLayout.AddNamespaceDeclaration("a", "http://schemas.openxmlformats.org/drawingml/2006/main");
			openXMLSlideLayout.AddNamespaceDeclaration("r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships");
			openXMLSlideLayout.AddNamespaceDeclaration("p", "http://schemas.openxmlformats.org/presentationml/2006/main");
			var unused = openXMLSlideLayout.AppendChild(new P.ColorMapOverride()
			{
				MasterColorMapping = new A.MasterColorMapping()
			});
		}
	}
}
