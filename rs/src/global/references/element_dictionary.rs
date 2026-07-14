use phf::{phf_map, Map};

pub(crate) struct Content {
    pub(crate) schemas_namespace: &'static str,
    pub(crate) schemas_type: &'static str,
    pub(crate) alias: &'static str,
    pub(crate) content_type: &'static str,
    pub(crate) extension: &'static str,
    pub(crate) extension_type: &'static str,
    pub(crate) default_path: &'static str,
    pub(crate) default_name: &'static str,
}
/// Common Content
pub(crate) static COMMON_TYPE_COLLECTION: Map<&'static str, &'static Content> = phf_map! {
    "content_type"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/package/2006/content-types",
        schemas_type:"",
        alias:"",
        content_type:"",
        extension:"xml",
        extension_type:"application/xml",
        default_path:".",
        default_name:""
    },
    "xml"=>&Content{
        schemas_namespace:"",
        schemas_type:"",
        alias:"",
        content_type:"application/xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:".",
        default_name:""
    },
    "rels"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/package/2006/relationships",
        schemas_type:"",
        alias:"r",
        content_type:"application/vnd.openxmlformats-package.relationships+xml",
        extension:"rels",
        extension_type:"application/vnd.openxmlformats-package.relationships+xml",
        default_path:".",
        default_name:""
    },
    "docProps_core"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/package/2006/metadata/core-properties",
        schemas_type:"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
        alias:"cp",
        content_type:"application/vnd.openxmlformats-package.core-properties+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"docProps",
        default_name:"core"
    },
    "hyperlink"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/officeDocument/2006",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
        alias:"",
        content_type:"",
        extension:"",
        extension_type:"",
        default_path:"",
        default_name:""
    },
    "drawing"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/drawingml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
        alias:"a",
        content_type:"application/vnd.openxmlformats-officedocument.drawing+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"drawings",
        default_name:"drawing"
    },
    // TODO Move to Drawing
    "theme"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/drawingml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
        alias:"",
        content_type:"application/vnd.openxmlformats-officedocument.theme+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"theme",
        default_name:"theme"
    },
};
/// Excel Related Content
pub(crate) static EXCEL_TYPE_COLLECTION: Map<&'static str, &'static Content> = phf_map! {
    "style"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"xl",
        default_name:"styles"
    },
    "share_string"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"xl",
        default_name:"sharedStrings"
    },
    "calc_chain"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"xl",
        default_name:"calcChain"
    },
    "workbook"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"xl",
        default_name:"workbook"
    },
    "table"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/table",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"tables",
        default_name:"table"
    },
    "drawing"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
        alias:"xdr",
        content_type:"application/vnd.openxmlformats-officedocument.drawing+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"drawings",
        default_name:"drawing"
    },
    "worksheet"=>&Content{
        schemas_namespace:"http://schemas.openxmlformats.org/spreadsheetml/2006/main",
        schemas_type:"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
        alias:"x",
        content_type:"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
        extension:"xml",
        extension_type:"application/xml",
        default_path:"worksheets",
        default_name:"sheet"
    },
};
/// Power Point Related Content
pub(crate) static POWER_POINT_TYPE_COLLECTION: Map<&'static str, &'static Content> = phf_map! {};
/// Word Related Content
pub(crate) static DOCUMENT_TYPE_COLLECTION: Map<&'static str, &'static Content> = phf_map! {};
