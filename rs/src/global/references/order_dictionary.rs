use phf::{phf_map, Map};

pub(crate) static EXCEL_ORDER_COLLECTION: Map<&'static str, &'static [&'static str]> = phf_map! {
    "workbook" => &[
        "fileVersion", "fileSharing", "workbookPr",
        "mc:AlternateContent", "xr:revisionPtr", "workbookProtection",
        "bookViews", "sheets", "functionGroups", "externalReferences",
        "definedNames", "calcPr", "oleSize", "customWorkbookViews",
        "pivotCaches", "smartTagPr", "smartTagTypes", "webPublishing",
        "fileRecoveryPr", "webPublishObjects", "extLst",
    ],
    "worksheet"=>&["sheetPr","dimension","sheetViews",
        "sheetFormatPr","cols","sheetData",
        "sheetCalcPr","protectedRanges","scenarios",
        "autoFilter","sortState","dataConsolidate",
        "customSheetViews","mergeCells","phoneticPr",
        "conditionalFormatting",
        "dataValidations","hyperlinks","printOptions",
        "pageMargins","pageSetup","headerFooter",
        "rowBreaks","colBreaks","customProperties",
        "cellWatches","ignoredErrors","smartTags",
        "drawing","drawingHF","picture",
        "oleObjects","controls","webPublishItems",
        "tableParts","extLst",
    ],
    // Drawing component 
    "absoluteAnchor"=>&["pos","ext","sp","grpSp","graphicFrame","cxnSp","pic","clientData"],
    "oneCellAnchor"=>&["from","ext","sp","grpSp","graphicFrame","cxnSp","pic","clientData"],
    "twoCellAnchor"=>&["from","to","sp","grpSp","graphicFrame","cxnSp","pic","clientData"]
};
