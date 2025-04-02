
#[derive(Debug, Clone)]
pub enum HyperlinkPropertyTypeValues {
    EXISTING_FILE,
    WEB_URL,
    TARGET_SHEET,
    TARGET_SLIDE,
    NEXT_SLIDE,
    PREVIOUS_SLIDE,
    FIRST_SLIDE,
    LAST_SLIDE,
}

#[derive(Debug, Clone)]
pub struct HyperlinkProperties {
    pub link_type: HyperlinkPropertyTypeValues,
    pub link: String,
    pub display: Option<String>,
}
