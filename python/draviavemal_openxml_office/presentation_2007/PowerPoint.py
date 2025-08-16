from openxml_office_fbs.presentation_2007 import PresentationPropertiesModel;
from flatbuffers import Builder;

class PowerPoint:
    def __init__(self):
        builder = Builder(1024)
        PresentationPropertiesModel.PresentationPropertiesModelStart(builder)
        PresentationPropertiesModel.AddIsInMemory(builder,True)
        PresentationPropertiesModel.PresentationPropertiesModelEnd(builder)   
        pass
    def SaveAs(self):
        pass