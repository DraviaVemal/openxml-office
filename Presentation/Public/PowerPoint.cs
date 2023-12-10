﻿using DocumentFormat.OpenXml;

namespace OpenXMLOffice.Presentation;
public class PowerPoint
{
    private readonly Presentation presentation;
    /// <summary>
    /// Open and work with existing file
    /// </summary>
    /// <param name="filePath"></param>
    /// <param name="isEditable"></param>
    /// <param name="powerPointProperties"></param>
    public PowerPoint(string filePath, bool isEditable, PresentationProperties? powerPointProperties = null)
    {
        presentation = new(filePath, isEditable, powerPointProperties);
    }
    /// <summary>
    /// Create New file in the system
    /// </summary>
    /// <param name="filePath"></param>
    /// <param name="powerPointProperties"></param>
    public PowerPoint(string filePath, PresentationProperties? powerPointProperties = null)
    {
        presentation = new(filePath, powerPointProperties);
    }
    /// <summary>
    /// Create New file in the system
    /// </summary>
    /// <param name="filePath"></param>
    /// <param name="powerPointProperties"></param>
    public PowerPoint(string filePath, PresentationProperties? powerPointProperties = null, PresentationDocumentType presentationDocumentType = PresentationDocumentType.Presentation)
    {
        presentation = new(filePath, powerPointProperties, presentationDocumentType);
    }

    /// <summary>
    /// Works with in memory object can be saved to file at later point
    /// </summary>
    /// <param name="filePath"></param>
    /// <param name="powerPointProperties"></param>
    public PowerPoint(Stream stream, PresentationDocumentType presentationDocumentType, PresentationProperties? powerPointProperties = null)
    {
        presentation = new(stream, powerPointProperties, presentationDocumentType);
    }

    public void AddSlide(PresentationConstants.SlideLayoutType slideLayoutType)
    {
        presentation.AddSlide(slideLayoutType);
    }

    public void Save()
    {
        presentation.Save();
    }
    public void SaveAs(string filePath)
    {
        presentation.SaveAs(filePath);
    }
}
