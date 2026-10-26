use super::relationships::{
    base_dir_of, collect_id_usage, parse_relationships, rels_name_for, rels_owner,
    replace_last_segment, resolve_target, rewrite_targets,
};
use anyhow::{Context, Result as AnyResult};
use std::collections::{HashMap, HashSet, VecDeque};

/// Part-family definitions used during renumbering.
const GROUPS: &[(&str, &str, &str)] = &[
    ("xl/worksheets", "sheet", "xml"),
    ("xl/drawings", "drawing", "xml"),
    ("xl/tables", "table", "xml"),
    ("xl/charts", "chart", "xml"),
];

/// Package write order and name changes.
#[derive(Debug, Default)]
pub(crate) struct LayoutPlan {
    /// Names in write order.
    pub(crate) order: Vec<String>,
    /// Old name to new name.
    pub(crate) rename: HashMap<String, String>,
    /// Rewritten relationship parts.
    pub(crate) rewritten_rels: HashMap<String, Vec<u8>>,
}

fn match_group(name: &str) -> Option<(usize, usize)> {
    for (index, (dir, base, extension)) in GROUPS.iter().enumerate() {
        let Some(rest) = name
            .strip_prefix(dir)
            .and_then(|rest| rest.strip_prefix('/'))
            .and_then(|rest| rest.strip_prefix(base))
        else {
            continue;
        };
        let Some(number) = rest
            .strip_suffix(extension)
            .and_then(|rest| rest.strip_suffix('.'))
        else {
            continue;
        };
        if number.is_empty()
            || number.starts_with('0')
            || !number.bytes().all(|byte| byte.is_ascii_digit())
        {
            continue;
        }
        if let Ok(number) = number.parse::<usize>() {
            return Some((index, number));
        }
    }
    None
}

fn group_part_name(group: usize, number: usize) -> String {
    let (dir, base, extension) = GROUPS[group];
    format!("{}/{}{}.{}", dir, base, number, extension)
}

/// Build deterministic write order and a validated rename plan.
pub(crate) fn plan_layout(
    names: &[String],
    renumber: bool,
    read: &mut dyn FnMut(&str) -> AnyResult<Vec<u8>>,
) -> AnyResult<LayoutPlan> {
    let name_set: HashSet<&str> = names.iter().map(String::as_str).collect();
    let lower_lookup: HashMap<String, &str> = names
        .iter()
        .map(|name| (name.to_ascii_lowercase(), name.as_str()))
        .collect();
    let find = |candidate: &str| -> Option<String> {
        if name_set.contains(candidate) {
            Some(candidate.to_string())
        } else {
            lower_lookup
                .get(&candidate.to_ascii_lowercase())
                .map(|name| name.to_string())
        }
    };

    let mut order: Vec<String> = Vec::with_capacity(names.len());
    let mut seen: HashSet<String> = HashSet::new();
    let mut parsed: HashMap<String, String> = HashMap::new();
    let mut assigned: HashMap<String, String> = HashMap::new();
    let mut counters = vec![0usize; GROUPS.len()];
    let mut queue: VecDeque<Option<String>> = VecDeque::new();

    let root_rels = rels_name_for(None);
    if name_set.contains(root_rels.as_str()) {
        seen.insert(root_rels.clone());
        order.push(root_rels);
    }
    queue.push_back(None);

    while let Some(source) = queue.pop_front() {
        let rels_name = rels_name_for(source.as_deref());
        if !name_set.contains(rels_name.as_str()) {
            continue;
        }
        let base_dir = base_dir_of(source.as_deref());
        let mut records = parse_relationships(&read(&rels_name)?)
            .context("draviavemal-openxml_office::Relationship graph read failed")?;

        let mut relationship_counts: HashMap<usize, usize> = HashMap::new();
        for record in records.iter().filter(|record| !record.is_external) {
            if let Some(part) = resolve_target(&base_dir, &record.target)
                .and_then(|resolved_target| find(&resolved_target))
            {
                if let Some((group, _)) = match_group(&part) {
                    *relationship_counts.entry(group).or_insert(0) += 1;
                }
            }
        }
        if renumber && relationship_counts.values().any(|count| *count >= 2) {
            if let Some(source_part) = source.as_deref() {
                if let Ok(content) = read(source_part) {
                    let relationship_ids: HashSet<&str> =
                        records.iter().map(|record| record.id.as_str()).collect();
                    if let Ok(usage) = collect_id_usage(&content, &relationship_ids) {
                        records.sort_by_key(|record| {
                            usage.get(record.id.as_str()).copied().unwrap_or(usize::MAX)
                        });
                    }
                }
            }
        }

        for record in records.iter().filter(|record| !record.is_external) {
            let Some(part) = resolve_target(&base_dir, &record.target)
                .and_then(|resolved_target| find(&resolved_target))
            else {
                continue;
            };
            if rels_owner(&part).is_some() || !seen.insert(part.clone()) {
                continue;
            }
            order.push(part.clone());
            let part_rels = rels_name_for(Some(&part));
            if name_set.contains(part_rels.as_str()) && seen.insert(part_rels.clone()) {
                order.push(part_rels);
            }
            if let Some((group, _)) = match_group(&part) {
                counters[group] += 1;
                assigned.insert(part.clone(), group_part_name(group, counters[group]));
            }
            queue.push_back(Some(part));
        }
        parsed.insert(rels_name, base_dir);
    }

    for name in names {
        if !seen.contains(name) {
            order.push(name.clone());
        }
    }

    let mut plan = LayoutPlan {
        order,
        ..LayoutPlan::default()
    };
    if !renumber {
        return Ok(plan);
    }

    for name in names {
        if parsed.contains_key(name) {
            continue;
        }
        let Some(owner) = rels_owner(name) else {
            continue;
        };
        parse_relationships(&read(name)?)
            .context("draviavemal-openxml_office::Relationship graph read failed")?;
        parsed.insert(name.clone(), base_dir_of(owner.as_deref()));
    }

    // Keep unreachable members after graph-ordered parts.
    let mut orphans: Vec<(usize, usize, &String)> = names
        .iter()
        .filter(|name| !assigned.contains_key(*name))
        .filter_map(|name| match_group(name).map(|(group, number)| (group, number, name)))
        .collect();
    orphans.sort();
    for (group, _, name) in orphans {
        counters[group] += 1;
        assigned.insert(name.clone(), group_part_name(group, counters[group]));
    }

    let mut rename: HashMap<String, String> = HashMap::new();
    for (old, new) in &assigned {
        if old == new {
            continue;
        }
        rename.insert(old.clone(), new.clone());
        let old_rels = rels_name_for(Some(old));
        if name_set.contains(old_rels.as_str()) {
            rename.insert(old_rels, rels_name_for(Some(new)));
        }
    }
    if rename.is_empty() {
        return Ok(plan);
    }

    let mut final_names: HashSet<String> = HashSet::with_capacity(names.len());
    for name in names {
        let final_name = rename.get(name).unwrap_or(name);
        if !final_names.insert(final_name.to_ascii_lowercase()) {
            return Ok(plan);
        }
    }

    let mut rewritten: HashMap<String, Vec<u8>> = HashMap::new();
    let mut consistent = true;
    for (rels_name, base_dir) in &parsed {
        let content = read(rels_name)?;
        let result = rewrite_targets(&content, |record| {
            if record.is_external {
                return None;
            }
            let part = resolve_target(base_dir, &record.target)
                .and_then(|resolved_target| find(&resolved_target))?;
            let new_part = rename.get(&part)?;
            let new_target = replace_last_segment(&record.target, new_part.rsplit('/').next()?);
            let verified = resolve_target(base_dir, &new_target)
                .map(|resolved| resolved.eq_ignore_ascii_case(new_part))
                .unwrap_or(false);
            if verified {
                Some(new_target)
            } else {
                consistent = false;
                None
            }
        })?;
        if let Some(bytes) = result {
            rewritten.insert(rels_name.clone(), bytes);
        }
    }
    if !consistent {
        return Ok(plan);
    }

    plan.rename = rename;
    plan.rewritten_rels = rewritten;
    Ok(plan)
}
