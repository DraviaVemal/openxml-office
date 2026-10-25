use crate::{
    log_elapsed,
    spreadsheet_2007::{
        models::{CellStyleSetting, StyleId},
        WorkSheet, WorkbookPart, WorkbookSettings, WorkbookView,
    },
};
use anyhow::{Context, Error as AnyError, Result as AnyResult};
use std::rc::Rc;

impl WorkbookPart {
    pub(crate) fn set_workbook_settings_mut(
        &mut self,
        settings: WorkbookSettings,
    ) -> AnyResult<(), AnyError> {
        if settings.active_sheet.is_none()
            && settings.is_visible.is_none()
            && settings.is_minimized.is_none()
            && settings.hide_tab.is_none()
            && settings.hide_vertical_scroll.is_none()
            && settings.hide_horizontal_scroll.is_none()
            && settings.hide_sheet.is_none()
        {
            return Ok(());
        }

        let active_tab = if let Some(active_sheet) = settings.active_sheet.as_deref() {
            self.sheet_collection
                .try_borrow()
                .context("draviavemal-openxml_office::Failed to pull Sheet Collection Handle")?
                .iter()
                .position(|(name, _, _)| name == active_sheet)
                .map(|index| index.to_string())
        } else {
            None
        };

        if let Some(hidden_sheet) = settings.hide_sheet.as_deref() {
            for (sheet_name, _, is_hidden) in self
                .sheet_collection
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull Sheet Collection Handle")?
                .iter_mut()
            {
                *is_hidden = sheet_name == hidden_sheet;
            }
        }

        let view = self.workbook_view.get_or_insert_with(WorkbookView::default);
        if let Some(active_tab) = active_tab {
            view.active_tab = Some(active_tab);
        }
        if let Some(is_visible) = settings.is_visible {
            view.visibility = Some(if is_visible { "visible" } else { "hidden" }.to_string());
        }
        if let Some(is_minimized) = settings.is_minimized {
            view.minimize = is_minimized;
        }
        if let Some(hide_tab) = settings.hide_tab {
            view.hide_sheet_tab = hide_tab;
        }
        if let Some(hide_vertical_scroll) = settings.hide_vertical_scroll {
            view.hide_vertical_scroll = hide_vertical_scroll;
        }
        if let Some(hide_horizontal_scroll) = settings.hide_horizontal_scroll {
            view.hide_horizontal_scroll = hide_horizontal_scroll;
        }
        Ok(())
    }
}

impl WorkbookPart {
    pub(crate) fn add_sheet_mut(
        &mut self,
        sheet_name: Option<String>,
    ) -> AnyResult<WorkSheet, AnyError> {
        Ok(WorkSheet::new(
            self.office_document.clone(),
            Rc::downgrade(&self.sheet_collection),
            Rc::downgrade(&self.workbook_relationship_part),
            Rc::downgrade(&self.common_service),
            sheet_name,
        )
        .context("draviavemal-openxml_office::Worksheet Creation Failed")?)
    }

    pub(crate) fn get_worksheet_mut(&mut self, sheet_name: &str) -> AnyResult<WorkSheet, AnyError> {
        if self
            .sheet_collection
            .borrow()
            .iter()
            .any(|(ref_sheet_name, _, _)| ref_sheet_name == sheet_name)
        {
            log_elapsed!(
                || {
                    WorkSheet::new(
                        self.office_document.clone(),
                        Rc::downgrade(&self.sheet_collection),
                        Rc::downgrade(&self.workbook_relationship_part),
                        Rc::downgrade(&self.common_service),
                        Some(sheet_name.to_string()),
                    )
                    .context("draviavemal-openxml_office::Failed to Find Worksheet")
                },
                "Get Exiting Workbook"
            )
        } else {
            Err(AnyError::msg(
                "draviavemal-openxml_office::Sheet Not Found!",
            ))
        }
    }

    /// Set Active sheet on opening the excel
    pub(crate) fn set_active_sheet_mut(&mut self, sheet_name: &str) -> AnyResult<(), AnyError> {
        let mut tab_count: i32 = 0;
        for (current_sheet_name, _, _) in self
            .sheet_collection
            .try_borrow()
            .context("draviavemal-openxml_office::Failed to pull Sheet Collection Handle")?
            .iter()
        {
            if current_sheet_name == sheet_name {
                self.workbook_view
                    .get_or_insert(WorkbookView::default())
                    .active_tab = Some(tab_count.to_string());
                break;
            }
            tab_count += 1;
        }
        Ok(())
    }

    /// Set workbook visibility
    pub(crate) fn set_visibility_mut(&mut self, is_visible: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.visibility = if is_visible {
                Some("visible".to_string())
            } else {
                Some("hidden".to_string())
            }
        } else {
            self.workbook_view = Some(WorkbookView {
                visibility: if is_visible {
                    Some("visible".to_string())
                } else {
                    Some("hidden".to_string())
                },
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook minimized
    pub(crate) fn minimize_workbook_mut(&mut self, is_minimized: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.minimize = is_minimized;
        } else {
            self.workbook_view = Some(WorkbookView {
                minimize: is_minimized,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set visibility of sheet tabs in workbook
    pub(crate) fn hide_sheet_tabs_mut(&mut self, hide_sheet_tab: bool) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_sheet_tab = hide_sheet_tab;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_sheet_tab,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook Vertical Scroll Visibility
    pub(crate) fn hide_vertical_scroll_mut(
        &mut self,
        hide_vertical_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_vertical_scroll = hide_vertical_scroll;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_vertical_scroll,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Set workbook Horizontal Scroll Visibility
    pub(crate) fn hide_horizontal_scroll_mut(
        &mut self,
        hide_horizontal_scroll: bool,
    ) -> AnyResult<(), AnyError> {
        if let Some(workbook_view) = &mut self.workbook_view {
            workbook_view.hide_horizontal_scroll = hide_horizontal_scroll;
        } else {
            self.workbook_view = Some(WorkbookView {
                hide_horizontal_scroll,
                ..WorkbookView::default()
            })
        }
        Ok(())
    }

    /// Hide sheet on opening the excel
    pub(crate) fn hide_sheet_mut(&mut self, sheet_name: &str) -> AnyResult<(), AnyError> {
        for (current_sheet_name, _, hide_sheet) in self
            .sheet_collection
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Failed to pull Sheet Collection Handle")?
            .iter_mut()
        {
            if current_sheet_name == sheet_name {
                *hide_sheet = true
            } else {
                *hide_sheet = false
            }
        }
        Ok(())
    }

    pub(crate) fn rename_sheet_name_mut(
        &mut self,
        old_sheet_name: &str,
        new_sheet_name: &str,
    ) -> AnyResult<(), AnyError> {
        // Check if sheet with same name exist
        if self
            .sheet_collection
            .try_borrow()
            .context("draviavemal-openxml_office::Failed to pull Sheet Name Collection")?
            .iter()
            .any(|item| new_sheet_name == item.0)
        {
            Err(AnyError::msg(
                "draviavemal-openxml_office::New Sheet Name Already exist in the stack",
            ))
        } else {
            if let Some(record) = self
                .sheet_collection
                .try_borrow_mut()
                .context("draviavemal-openxml_office::Failed to pull Sheet Name Collection")?
                .iter_mut()
                .find(|item| item.0 == old_sheet_name)
            {
                record.0 = new_sheet_name.to_string();
                Ok(())
            } else {
                Err(AnyError::msg(
                    "draviavemal-openxml_office::Old Sheet Name not found in the stack",
                ))
            }
        }
    }

    /// Return Style Id for the said combination
    pub(crate) fn get_style_id_mut(
        &mut self,
        style_setting: CellStyleSetting,
    ) -> AnyResult<StyleId, AnyError> {
        self.common_service
            .try_borrow_mut()
            .context("draviavemal-openxml_office::Failed to get Style Handle")?
            .get_style_id_mut(style_setting)
    }
}
