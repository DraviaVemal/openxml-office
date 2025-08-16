from openxml_office_fbs.document_2007 import DocumentPropertiesModel;
from flatbuffers import Builder;

class Word:
    def __init__(self):
        builder = Builder(1024)
        DocumentPropertiesModel.DocumentPropertiesModelStart(builder)
        DocumentPropertiesModel.AddIsInMemory(builder,True)
        DocumentPropertiesModel.DocumentPropertiesModelEnd(builder)   
        pass
    def SaveAs(self):
        pass