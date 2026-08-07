use draviavemal_xml_rs::XmlElement;

/// Namespace declaration pairing a URI with its canonical prefix.
/// Mirrors element_dictionary's (schemas_namespace, default_alias) pattern,
/// keyed by URI rather than component name, and adds scope-aware resolution helpers.
pub(crate) struct NamespaceDeclaration {
    /// W3C namespace URI, e.g. "http://schemas.openxmlformats.org/drawingml/2006/main"
    pub(crate) schemas_namespace: &'static str,
    /// Canonical alias used when writing new documents, e.g. "a"
    pub(crate) default_alias: &'static str,
}

impl NamespaceDeclaration {
    /// Returns the alias in scope for this URI at `element`, falling back to `default_alias`.
    pub(crate) fn resolve(&self, element: &XmlElement) -> String {
        element
            .get_alias_for_uri(self.schemas_namespace)
            .unwrap_or_else(|| self.default_alias.to_string())
    }

    /// Builds a prefixed element tag using the in-scope alias, e.g. `"a:blip"`.
    pub(crate) fn tag(&self, element: &XmlElement, local_name: &str) -> String {
        format!("{}:{}", self.resolve(element), local_name)
    }

    /// Builds a prefixed attribute name using the in-scope alias, e.g. `"r:embed"`.
    pub(crate) fn attr(&self, element: &XmlElement, local_name: &str) -> String {
        format!("{}:{}", self.resolve(element), local_name)
    }
}

/// Office document relationships namespace
pub(crate) static RELATIONSHIPS_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
    default_alias: "r",
};
/// DrawingML main namespace (shapes, images, styles)
pub(crate) static DRAWINGML_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/drawingml/2006/main",
    default_alias: "a",
};
/// SpreadsheetDrawing namespace (anchors, drawings embedded in sheets)
pub(crate) static SPREADSHEET_DRAWING_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
    default_alias: "xdr",
};
/// DrawingML chart namespace
pub(crate) static CHART_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/drawingml/2006/chart",
    default_alias: "c",
};
/// SpreadsheetML main namespace (workbook, worksheet, styles)
pub(crate) static SPREADSHEET_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
    default_alias: "x",
};
/// OPC package relationships namespace
pub(crate) static RELS_PKG_NS: NamespaceDeclaration = NamespaceDeclaration {
    schemas_namespace: "http://schemas.openxmlformats.org/package/2006/relationships",
    default_alias: "r",
};
