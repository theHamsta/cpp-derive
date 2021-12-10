use once_cell::unsync::Lazy;
use regex::Regex;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

use tree_sitter::{Query, QueryCursor};

#[derive(Debug, Serialize)]
pub struct Field<'field> {
    pub field_name: &'field str,
    pub field_type: &'field str,
    pub default_value: Option<&'field str>,
}

#[derive(Debug, Serialize)]
pub struct Class<'class> {
    pub name: &'class str,
    pub attributes: HashSet<String>,
    pub fields: HashMap<&'class str, Field<'class>>,
}

pub fn parse_classes<'a>(
    parser: &mut tree_sitter::Parser,
    source_code: &'a Vec<u8>,
    query: &Query,
) -> anyhow::Result<HashMap<usize, Class<'a>>> {
    let mut classes = HashMap::new();
    let tree = parser.parse(&source_code, None).unwrap();

    let mut query_cursor = QueryCursor::new();

    for (m, _) in query_cursor.captures(&query, tree.root_node(), source_code.as_slice()) {
        let mut class_id = None;
        let mut class_name = None;
        let mut attributes = HashSet::new();

        let mut field_name = None;
        let mut field_type = None;
        let mut default_value = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            match capture_name.as_str() {
                "name" => {
                    class_id = Some(capture.node.id());
                    class_name = Some(
                        capture
                            .node
                            .utf8_text(&source_code)
                            .expect("Failed to get node text"),
                    );
                }
                "attribute" => {
                    let attribute_text = capture
                        .node
                        .utf8_text(&source_code)
                        .expect("Failed to get node text");
                    let regex = Lazy::new(|| Regex::new(r"derive\((.*)\)").unwrap());
                    for cap in regex.captures_iter(attribute_text) {
                        for s in cap[1].split(',') {
                            attributes.insert(s.into());
                        }
                    }
                }
                "decl" => {
                    field_name = Some(
                        capture
                            .node
                            .utf8_text(&source_code)
                            .expect("Failed to get node text"),
                    )
                }
                "type" => {
                    field_type = Some(
                        capture
                            .node
                            .utf8_text(&source_code)
                            .expect("Failed to get node text"),
                    )
                }
                "default" => {
                    default_value = Some(
                        capture
                            .node
                            .utf8_text(&source_code)
                            .expect("Failed to get node text"),
                    )
                }
                _ => (),
            }
        }
        let field_name = field_name.unwrap();
        let field_type = field_type.unwrap();
        classes
            .entry(class_id.unwrap())
            .or_insert(Class {
                attributes,
                name: class_name.unwrap(),
                fields: HashMap::new(),
            })
            .fields
            .insert(
                field_name,
                Field {
                    field_name,
                    field_type,
                    default_value,
                },
            );
    }
    Ok(classes)
}
