#[derive(Debug, Clone)]
pub enum PowerPointHyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SLIDE,
    NEXT_SLIDE,
    PREVIOUS_SLIDE,
    FIRST_SLIDE,
    LAST_SLIDE,
}

#[derive(Debug, Clone)]
pub enum ExcelHyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SHEET,
}

#[derive(Debug, Clone)]
pub struct ExcelHyperlinkProperties {
    pub display: Option<String>,
    pub link_type: ExcelHyperlinkPropertyTypeValues,
    pub link: String,
}
