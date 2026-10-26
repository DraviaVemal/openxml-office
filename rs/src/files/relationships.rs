use anyhow::{Context, Result as AnyResult};
use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};
use std::collections::{HashMap, HashSet};

/// Relationship data used by the package planner.
#[derive(Debug, Clone)]
pub(crate) struct RelationshipRecord {
    pub(crate) id: String,
    pub(crate) target: String,
    pub(crate) is_external: bool,
}

/// Resolve the part that owns a relationship file.
pub(crate) fn rels_owner(name: &str) -> Option<Option<String>> {
    let stem = name.strip_suffix(".rels")?;
    let (dir, file) = if let Some(file) = stem.strip_prefix("_rels/") {
        ("", file)
    } else {
        let index = stem.rfind("/_rels/")?;
        (&stem[..index], &stem[index + "/_rels/".len()..])
    };
    if file.is_empty() {
        return Some(None);
    }
    Some(Some(if dir.is_empty() {
        file.to_string()
    } else {
        format!("{}/{}", dir, file)
    }))
}

/// Return the relationship part name for an owner.
pub(crate) fn rels_name_for(part: Option<&str>) -> String {
    match part {
        None => "_rels/.rels".to_string(),
        Some(part) => match part.rfind('/') {
            Some(index) => format!("{}/_rels/{}.rels", &part[..index], &part[index + 1..]),
            None => format!("_rels/{}.rels", part),
        },
    }
}

/// Return the URI base directory for an owner.
pub(crate) fn base_dir_of(part: Option<&str>) -> String {
    part.and_then(|part| part.rfind('/').map(|index| part[..index].to_string()))
        .unwrap_or_default()
}

fn percent_decode(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let hex = std::str::from_utf8(&bytes[index + 1..index + 3])
                .ok()
                .and_then(|hex| u8::from_str_radix(hex, 16).ok());
            if let Some(value) = hex {
                decoded.push(value);
                index += 3;
                continue;
            }
        }
        decoded.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

/// Resolve an internal target to a package part name.
pub(crate) fn resolve_target(base_dir: &str, target: &str) -> Option<String> {
    let path_end = target.find(['#', '?']).unwrap_or(target.len());
    let raw = &target[..path_end];
    if raw.is_empty() {
        return None;
    }
    if let Some(colon) = raw.find(':') {
        if !raw[..colon].contains('/') {
            return None;
        }
    }
    let decoded = percent_decode(raw);
    let full = if let Some(absolute) = decoded.strip_prefix('/') {
        absolute.to_string()
    } else if base_dir.is_empty() {
        decoded
    } else {
        format!("{}/{}", base_dir, decoded)
    };
    let mut segments: Vec<&str> = Vec::new();
    for segment in full.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop()?;
            }
            other => segments.push(other),
        }
    }
    if segments.is_empty() {
        None
    } else {
        Some(segments.join("/"))
    }
}

/// Replace a target's file name while retaining its suffix.
pub(crate) fn replace_last_segment(target: &str, new_file_name: &str) -> String {
    let path_end = target.find(['#', '?']).unwrap_or(target.len());
    let (path, suffix) = target.split_at(path_end);
    let cut = path.rfind('/').map(|index| index + 1).unwrap_or(0);
    format!("{}{}{}", &path[..cut], new_file_name, suffix)
}

fn read_record(element: &BytesStart) -> AnyResult<RelationshipRecord> {
    let mut record = RelationshipRecord {
        id: String::new(),
        target: String::new(),
        is_external: false,
    };
    for attribute in element.attributes() {
        let attribute =
            attribute.context("draviavemal-openxml_office::Relationship attribute invalid")?;
        match attribute.key.as_ref() {
            b"Id" => record.id = attribute.unescape_value()?.into_owned(),
            b"Target" => record.target = attribute.unescape_value()?.into_owned(),
            b"TargetMode" => {
                record.is_external = attribute.unescape_value()?.eq_ignore_ascii_case("External")
            }
            _ => {}
        }
    }
    Ok(record)
}

fn is_relationship(element: &BytesStart) -> bool {
    element.local_name().as_ref() == b"Relationship"
}

pub(crate) fn parse_relationships(content: &[u8]) -> AnyResult<Vec<RelationshipRecord>> {
    let mut reader = Reader::from_reader(content);
    let mut records = Vec::new();
    loop {
        match reader
            .read_event()
            .context("draviavemal-openxml_office::Relationship part xml read failed")?
        {
            Event::Start(element) | Event::Empty(element) => {
                if is_relationship(&element) {
                    records.push(read_record(&element)?);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(records)
}

/// Rewrite targets and return `None` when unchanged.
pub(crate) fn rewrite_targets<F>(content: &[u8], mut new_target: F) -> AnyResult<Option<Vec<u8>>>
where
    F: FnMut(&RelationshipRecord) -> Option<String>,
{
    let mut reader = Reader::from_reader(content);
    let mut writer = Writer::new(Vec::with_capacity(content.len() + 64));
    let mut changed = false;
    loop {
        let event = reader
            .read_event()
            .context("draviavemal-openxml_office::Relationship part xml read failed")?;
        if matches!(event, Event::Eof) {
            break;
        }
        let replacement = match &event {
            Event::Start(element) | Event::Empty(element) if is_relationship(element) => {
                match new_target(&read_record(element)?) {
                    Some(target) => {
                        let mut rebuilt = BytesStart::new(
                            String::from_utf8_lossy(element.name().as_ref()).into_owned(),
                        );
                        for attribute in element.attributes() {
                            let attribute = attribute.context(
                                "draviavemal-openxml_office::Relationship attribute invalid",
                            )?;
                            if attribute.key.as_ref() == b"Target" {
                                rebuilt.push_attribute(("Target", target.as_str()));
                            } else {
                                rebuilt.push_attribute(attribute);
                            }
                        }
                        Some(rebuilt)
                    }
                    None => None,
                }
            }
            _ => None,
        };
        match replacement {
            Some(rebuilt) => {
                changed = true;
                let rebuilt_event = if matches!(event, Event::Empty(_)) {
                    Event::Empty(rebuilt)
                } else {
                    Event::Start(rebuilt)
                };
                writer
                    .write_event(rebuilt_event)
                    .context("draviavemal-openxml_office::Relationship part xml write failed")?;
            }
            None => writer
                .write_event(event)
                .context("draviavemal-openxml_office::Relationship part xml write failed")?,
        }
    }
    Ok(if changed {
        Some(writer.into_inner())
    } else {
        None
    })
}

/// Return first-use order for referenced relationship IDs.
pub(crate) fn collect_id_usage(
    content: &[u8],
    ids: &HashSet<&str>,
) -> AnyResult<HashMap<String, usize>> {
    let mut reader = Reader::from_reader(content);
    let mut usage: HashMap<String, usize> = HashMap::new();
    loop {
        match reader
            .read_event()
            .context("draviavemal-openxml_office::Part xml read failed")?
        {
            Event::Start(element) | Event::Empty(element) => {
                for attribute in element.attributes().flatten() {
                    if !attribute.key.as_ref().contains(&b':') {
                        continue;
                    }
                    let value = String::from_utf8_lossy(&attribute.value);
                    if ids.contains(value.as_ref()) {
                        let next = usage.len();
                        usage.entry(value.into_owned()).or_insert(next);
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(usage)
}
