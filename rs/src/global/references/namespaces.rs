pub(crate) use draviavemal_xml_rs::NamespaceDeclaration;

pub(crate) static DRAWINGML_NS: NamespaceDeclaration =
    NamespaceDeclaration::new("http://schemas.openxmlformats.org/drawingml/2006/main", "a");
pub(crate) static SPREADSHEET_DRAWING_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
    "xdr",
);
pub(crate) static CHART_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/drawingml/2006/chart",
    "c",
);
pub(crate) static SPREADSHEET_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    "x",
);
pub(crate) static CONTENT_TYPES_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/package/2006/content-types",
    "r",
);
pub(crate) static RELATIONSHIP_OFFICE_DOC_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    "r",
);
pub(crate) static RELATIONSHIP_PKG_NS: NamespaceDeclaration = NamespaceDeclaration::new(
    "http://schemas.openxmlformats.org/package/2006/relationships",
    "r",
);
pub(crate) static DCTERMS_NS: NamespaceDeclaration =
    NamespaceDeclaration::new("http://purl.org/dc/terms/", "dcterms");
