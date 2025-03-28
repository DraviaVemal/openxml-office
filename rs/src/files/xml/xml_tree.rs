use anyhow::{anyhow, Context, Error as AnyError, Result as AnyResult};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub(crate) struct XmlDocument {
    running_id: usize,
    /// XML Namespace collection
    namespace_collection: Rc<RefCell<HashMap<String, String>>>,
    /// XML Element Collection
    xml_element_collection: HashMap<usize, XmlElement>,
}

/// ####################### Im-Mut Access Functions ####################
impl XmlDocument {
    pub(crate) fn new() -> Self {
        Self {
            running_id: 0,
            namespace_collection: Rc::new(RefCell::new(HashMap::new())),
            xml_element_collection: HashMap::new(),
        }
    }

    pub(crate) fn get_element_count(&self) -> usize {
        self.xml_element_collection.len()
    }

    pub(crate) fn get_root(&self) -> Option<&XmlElement> {
        self.xml_element_collection.get(&0)
    }

    pub(crate) fn get_element_ids_by_attribute(
        &self,
        attribute_key: &str,
        attribute_value: &str,
        parent_id: Option<&usize>,
    ) -> Option<Vec<usize>> {
        let parent_id = parent_id.unwrap_or(&0);
        let mut element_ids: Option<Vec<usize>> = None;
        if let Some(parent_element) = self.xml_element_collection.get(&parent_id) {
            let matching_element_ids = parent_element
                .children
                .borrow()
                .iter()
                .filter(|item| {
                    if let Some(current) = self.xml_element_collection.get(&item.id) {
                        if let Some(attribute) = current.get_attribute() {
                            if let Some(value) = attribute.get(attribute_key) {
                                return value == attribute_value;
                            }
                        }
                    }
                    false
                })
                .map(|item| item.id)
                .collect::<Vec<usize>>();
            if matching_element_ids.len() > 0 {
                element_ids = Some(matching_element_ids);
            }
        }
        element_ids
    }

    fn get_element_ids_by_tag(
        &self,
        filter_tag: &str,
        parent_id: Option<&usize>,
    ) -> Option<Vec<usize>> {
        let parent_id = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get(parent_id) {
            let element_id_list = parent_element
                .children
                .borrow()
                .iter()
                .filter(|item| item.tag == filter_tag)
                .map(|item| item.id)
                .collect::<Vec<usize>>();
            if element_id_list.len() > 0 {
                return Some(element_id_list);
            }
        }
        None
    }

    pub(crate) fn get_element(&self, element_id: &usize) -> Option<&XmlElement> {
        self.xml_element_collection.get(element_id)
    }
}

/// ####################### Mut Access Functions ####################
impl XmlDocument {
    pub(crate) fn get_root_mut(&mut self) -> Option<&mut XmlElement> {
        self.xml_element_collection.get_mut(&0)
    }

    pub(crate) fn create_root_mut(&mut self, tag: &str) -> AnyResult<&mut XmlElement, AnyError> {
        let element = XmlElement::new(Rc::downgrade(&self.namespace_collection), tag)
            .context("Create Root XML Element Failed")?;
        self.xml_element_collection.insert(0, element);
        Ok(self.xml_element_collection.get_mut(&0).unwrap())
    }

    /// Removes the child reference from parent and remove the master element itself from collection
    pub(crate) fn pop_elements_by_tag_mut(
        &mut self,
        filter_tag: &str,
        parent_id: Option<&usize>,
    ) -> Option<Vec<XmlElement>> {
        let parent_id_def = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get(parent_id_def) {
            if let Some(element_id_list) = self.get_element_ids_by_tag(filter_tag, parent_id) {
                parent_element
                    .children
                    .borrow_mut()
                    .retain(|item| item.tag != filter_tag);
                let mut elements: Vec<XmlElement> = Vec::new();
                for element_id in element_id_list {
                    if let Some(item) = self.xml_element_collection.remove(&element_id) {
                        elements.push(item);
                    }
                }
                if elements.len() > 0 {
                    Some(elements)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn pop_element_mut(&mut self, element_id: &usize) -> Option<XmlElement> {
        self.xml_element_collection.remove(element_id)
    }
    /// Insert Children Before specific Tag
    pub(crate) fn insert_children_before_tag_mut(
        &mut self,
        tag: &str,
        find_tag: &str,
        parent_id: Option<&usize>,
    ) -> AnyResult<&mut XmlElement, AnyError> {
        let parent_id = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get_mut(&parent_id) {
            let mut element = XmlElement::new(Rc::downgrade(&self.namespace_collection), tag)
                .context("Create XML Element Failed in insert before")?;
            element.set_parent_id_mut(parent_id.to_owned());
            self.running_id += 1;
            element.set_id_mut(self.running_id);
            parent_element.insert_children_before_tag_mut(self.running_id, tag, find_tag);
            self.xml_element_collection.insert(self.running_id, element);
            Ok(self
                .xml_element_collection
                .get_mut(&self.running_id)
                .unwrap())
        } else {
            Err(anyhow!("Parent Element Not Found"))
        }
    }
    /// Insert Children After specific Tag
    pub(crate) fn insert_children_after_tag_mut(
        &mut self,
        tag: &str,
        find_tag: &str,
        parent_id: Option<&usize>,
    ) -> AnyResult<&mut XmlElement, AnyError> {
        let parent_id = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get_mut(&parent_id) {
            let mut element = XmlElement::new(Rc::downgrade(&self.namespace_collection), tag)
                .context("Create XML Element Failed in insert before")?;
            element.set_parent_id_mut(parent_id.to_owned());
            self.running_id += 1;
            element.set_id_mut(self.running_id);
            parent_element.insert_children_after_tag_mut(self.running_id, tag, find_tag);
            self.xml_element_collection.insert(self.running_id, element);
            Ok(self
                .xml_element_collection
                .get_mut(&self.running_id)
                .unwrap())
        } else {
            Err(anyhow!("Parent Element Not Found"))
        }
    }
    /// Insert Child At Specific Position
    pub(crate) fn insert_child_at_mut(
        &mut self,
        tag: &str,
        position: &usize,
        parent_id: Option<&usize>,
    ) -> AnyResult<&mut XmlElement, AnyError> {
        let parent_id = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get_mut(&parent_id) {
            let mut element = XmlElement::new(Rc::downgrade(&self.namespace_collection), tag)
                .context("Create XML Element Failed insert child")?;
            element.set_parent_id_mut(*parent_id);
            self.running_id += 1;
            element.set_id_mut(self.running_id);
            parent_element.insert_children_at_mut(self.running_id, position.to_owned(), tag);
            self.xml_element_collection.insert(self.running_id, element);
            Ok(self
                .xml_element_collection
                .get_mut(&self.running_id)
                .unwrap())
        } else {
            Err(anyhow!("Parent Element Not Found"))
        }
    }

    pub(crate) fn append_child_mut(
        &mut self,
        tag: &str,
        parent_id: Option<&usize>,
    ) -> AnyResult<&mut XmlElement, AnyError> {
        let id = parent_id.unwrap_or(&0);
        if let Some(parent_element) = self.xml_element_collection.get_mut(id) {
            let mut element = XmlElement::new(Rc::downgrade(&self.namespace_collection), tag)
                .context("Create XML Element Failed append")?;
            element.set_parent_id_mut(id.to_owned());
            self.running_id += 1;
            element.set_id_mut(self.running_id);
            parent_element
                .append_children_mut(self.running_id, tag)
                .context("Failed to add Child Relation")?;
            self.xml_element_collection.insert(self.running_id, element);
            Ok(self
                .xml_element_collection
                .get_mut(&self.running_id)
                .unwrap())
        } else {
            Err(anyhow!("Parent Element Not Found"))
        }
    }

    pub(crate) fn get_first_element_id(
        &self,
        mut element_tree: Vec<&str>,
        start_element: Option<&usize>,
    ) -> AnyResult<Option<usize>, AnyError> {
        element_tree.reverse();
        let mut current_id = *start_element.unwrap_or(&0);
        let mut is_found = false;
        loop {
            if let Some(find_tag) = element_tree.pop() {
                let element = self.xml_element_collection.get(&current_id).unwrap();
                if element.get_tag() == find_tag && current_id == 0 {
                    // Skip the root element
                    if element_tree.len() == 0 {
                        is_found = true;
                        break;
                    }
                    continue;
                }
                let element_child = element
                    .children
                    .try_borrow()
                    .context("Children Borrow Failed")?;
                if let Some(found_child) = element_child.iter().find(|item| item.tag == find_tag) {
                    current_id = found_child.id;
                    if element_tree.len() == 0 {
                        is_found = true;
                        break;
                    }
                } else {
                    // Not Able to find any one Child
                    break;
                }
            } else {
                // If Vec has No Value to start
                break;
            }
        }
        if is_found {
            Ok(Some(current_id))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_first_element_mut(
        &mut self,
        element_tree: Vec<&str>,
        start_element: Option<&usize>,
    ) -> AnyResult<Option<&mut XmlElement>, AnyError> {
        if let Some(element_id) = self
            .get_first_element_id(element_tree, start_element)
            .context("Find Element By Id Failed")?
        {
            Ok(self.xml_element_collection.get_mut(&element_id))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn get_first_element_by_attribute_mut(
        &mut self,
        attribute_key: &str,
        attribute_value: &str,
        parent_id: Option<&usize>,
    ) -> Option<&mut XmlElement> {
        if let Some(ids) =
            self.get_element_ids_by_attribute(attribute_key, attribute_value, parent_id)
        {
            self.xml_element_collection.get_mut(&ids[0])
        } else {
            None
        }
    }

    pub(crate) fn get_element_mut(&mut self, element_id: &usize) -> Option<&mut XmlElement> {
        self.xml_element_collection.get_mut(element_id)
    }
}

#[derive(Debug)]
pub(crate) struct XmlElementChild {
    id: usize,
    tag: String,
}
/// Normalized XML representation
#[derive(Debug)]
pub(crate) struct XmlElement {
    /// Current node Id
    id: usize,
    /// Parent id if applicable else -1
    parent_id: usize,
    /// Element Tag name with Namespace
    tag: String,
    /// Attributes of the element if applicable with namespace
    attributes: Option<HashMap<String, String>>,
    /// Internal Value of the
    value: Option<String>,
    /// Child Element Names to pull up nodes quickly
    children: Rc<RefCell<Vec<XmlElementChild>>>,
    // ######################## Document Parts ##########################
    /// XML Namespace collection
    namespace_collection_ref: Weak<RefCell<HashMap<String, String>>>,
}

impl XmlElement {
    /// Create element with tree document reference
    fn new(
        namespace_collection: Weak<RefCell<HashMap<String, String>>>,
        tag: &str,
    ) -> AnyResult<Self, AnyError> {
        if Self::is_valid_xml_name(tag) {
            Ok(Self {
                id: 0,
                parent_id: 0,
                tag: tag.to_string(),
                attributes: None,
                value: None,
                children: Rc::new(RefCell::new(Vec::new())),
                namespace_collection_ref: namespace_collection,
            })
        } else {
            Err(anyhow!("Provided Tag Name \"{}\" is not valid.", tag))
        }
    }

    fn is_valid_xml_name(name: &str) -> bool {
        fn is_name_start_char(c: char) -> bool {
            c.is_ascii_alphabetic() || c == '_' || c.is_alphabetic()
        }
        fn is_name_char(c: char) -> bool {
            is_name_start_char(c) || c.is_ascii_digit() || c == '-' || c == '.' || c == ':'
        }
        if name.is_empty() {
            return false;
        }
        let mut chars = name.chars();
        if let Some(first_char) = chars.next() {
            if !is_name_start_char(first_char) {
                return false;
            }
        }
        let mut colon_count = 0;
        for c in chars {
            if c == ':' {
                colon_count += 1;
                if colon_count > 1 {
                    return false;
                }
            } else if !is_name_char(c) {
                return false;
            }
        }
        true
    }

    // ######################## Data Read Only Methods ###########################

    pub(crate) fn get_child_count(&self) -> usize {
        self.children.borrow().len()
    }

    pub(crate) fn get_tag(&self) -> &str {
        &self.tag
    }

    pub(crate) fn has_value(&self) -> bool {
        self.get_value().is_some()
    }

    pub(crate) fn is_empty_tag(&self) -> bool {
        self.children.borrow().len() == 0 && self.value.is_none()
    }

    pub(crate) fn get_attribute(&self) -> Option<&HashMap<String, String>> {
        self.attributes.as_ref()
    }

    pub(crate) fn get_namespace(&self) -> Option<HashMap<String, String>> {
        if let Some(namespace_collection) = self.namespace_collection_ref.upgrade() {
            Some(namespace_collection.borrow().clone())
        } else {
            None
        }
    }

    pub(crate) fn get_value(&self) -> &Option<String> {
        &self.value
    }

    pub(crate) fn get_id(&self) -> usize {
        self.id
    }
    /// Use with caution
    pub(crate) fn get_first_child_id(&self) -> Option<usize> {
        if let Some(child_element) = self.children.borrow().get(0) {
            Some(child_element.id)
        } else {
            None
        }
    }

    pub(crate) fn get_parent_id(&self) -> usize {
        self.parent_id
    }
}

// ########################## Data Write Methods ###########################
impl XmlElement {
    /// Remove the child reference irreversible
    pub(crate) fn pop_child_mut(&self) -> Option<(usize, String)> {
        if self.children.borrow_mut().len() > 0 {
            let child_element = self.children.borrow_mut().remove(0);
            Some((child_element.id, child_element.tag))
        } else {
            None
        }
    }

    pub(crate) fn get_attribute_mut(&mut self) -> Option<&mut HashMap<String, String>> {
        self.attributes.as_mut()
    }

    pub(crate) fn order_child_mut(&mut self, order_skeleton: &[&str]) -> AnyResult<(), AnyError> {
        if self.children.borrow_mut().len() > 0 && order_skeleton.len() > 0 {
            let order_map: std::collections::HashMap<&str, usize> = order_skeleton
                .iter()
                .enumerate()
                .map(|(index, &tag)| (tag, index))
                .collect();
            self.children.borrow_mut().sort_by_key(|item| {
                order_map
                    .get(item.tag.as_str())
                    .cloned()
                    .unwrap_or(usize::MAX)
            });
        }
        Ok(())
    }

    pub(crate) fn set_attribute_mut(
        &mut self,
        mut attributes: HashMap<String, String>,
    ) -> AnyResult<&mut Self, AnyError> {
        let keys = attributes
            .keys()
            .clone()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        if !keys
            .iter()
            .map(|item| item.to_string())
            .all(|item| Self::is_valid_xml_name(&item))
        {
            return Err(anyhow!("Not All the attribute satisfy naming standards"));
        }
        let ns_keys = keys
            .iter()
            .map(|item| item.to_string())
            .filter(|item| item.starts_with("xmlns"))
            .clone()
            .collect::<Vec<String>>();
        if !ns_keys.is_empty() {
            if let Some(ns_collection) = self.namespace_collection_ref.upgrade() {
                let mut namespace_collection = ns_collection
                    .try_borrow_mut()
                    .context("Namespace Collection Borrow Failed")?;
                for ns_key in ns_keys.clone() {
                    let ns_alias = ns_key.split(':').collect::<Vec<&str>>();
                    if ns_alias.len() > 1 {
                        if !namespace_collection.contains_key(&ns_alias[1].to_string()) {
                            namespace_collection.insert(
                                ns_alias[1].to_string(),
                                attributes.get(&ns_key).unwrap().to_string(),
                            );
                        }
                    } else {
                        namespace_collection.insert(
                            "<Default>".to_string(),
                            attributes.get(&ns_key).unwrap().to_string(),
                        );
                    }
                }
            }
        }
        attributes.retain(|key, _| !ns_keys.contains(key));
        self.validate_namespace(&attributes)
            .context("Attribute NameSpace Validation Failed")?;
        self.attributes = Some(attributes);
        Ok(self)
    }

    pub(crate) fn set_value_mut(&mut self, text: String) -> &mut Self {
        self.value = Some(text);
        self
    }

    fn append_children_mut(&mut self, child_id: usize, tag: &str) -> AnyResult<(), AnyError> {
        self.children
            .try_borrow_mut()
            .context("Failed to Pull children handle")?
            .push(XmlElementChild {
                id: child_id,
                tag: tag.to_string(),
            });
        Ok(())
    }

    fn insert_children_before_tag_mut(&mut self, child_id: usize, tag: &str, find_tag: &str) {
        let mut children = self.children.borrow_mut();
        if let Some(index) = children.iter().position(|item| item.tag == find_tag) {
            children.insert(
                index,
                XmlElementChild {
                    id: child_id,
                    tag: tag.to_string(),
                },
            );
        } else {
            children.insert(
                0,
                XmlElementChild {
                    id: child_id,
                    tag: tag.to_string(),
                },
            );
        }
    }

    fn insert_children_after_tag_mut(&mut self, child_id: usize, tag: &str, find_tag: &str) {
        let mut children = self.children.borrow_mut();
        if let Some(index) = children.iter().rposition(|item| item.tag == find_tag) {
            children.insert(
                index + 1,
                XmlElementChild {
                    id: child_id,
                    tag: tag.to_string(),
                },
            );
        } else {
            children.push(XmlElementChild {
                id: child_id,
                tag: tag.to_string(),
            });
        }
    }

    fn insert_children_at_mut(&mut self, child_id: usize, position: usize, tag: &str) {
        self.children.borrow_mut().insert(
            position,
            XmlElementChild {
                id: child_id,
                tag: tag.to_string(),
            },
        );
    }

    fn set_id_mut(&mut self, id: usize) {
        self.id = id;
    }

    fn set_parent_id_mut(&mut self, parent_id: usize) {
        self.parent_id = parent_id;
    }

    fn validate_namespace(
        &mut self,
        attributes: &HashMap<String, String>,
    ) -> AnyResult<(), AnyError> {
        Ok(())
    }
}
