use std::collections::{BTreeMap, BTreeSet};

use fluent_bundle::FluentResource;
use fluent_syntax::ast::{
    Entry, Expression, InlineExpression, Pattern, PatternElement, VariantKey,
};

use super::CatalogError;
use crate::LocaleId;

pub(super) fn validate(
    locale: &LocaleId,
    resource: &FluentResource,
) -> Result<BTreeMap<String, BTreeSet<String>>, CatalogError> {
    let mut graph = BTreeMap::new();
    let mut parameters = BTreeMap::new();
    for entry in resource.entries() {
        let Entry::Message(message) = entry else {
            if matches!(
                entry,
                Entry::Comment(_) | Entry::GroupComment(_) | Entry::ResourceComment(_)
            ) {
                continue;
            }
            return Err(failure(
                locale,
                "<catalog>",
                "only messages and comments are supported; terms are not supported",
            ));
        };
        let id = message.id.name;
        if graph.contains_key(id) {
            return Err(failure(locale, id, "duplicate message ID"));
        }
        if !message.attributes.is_empty() {
            return Err(failure(
                locale,
                id,
                "attributes are not supported; use separate message IDs",
            ));
        }
        let value = message
            .value
            .as_ref()
            .ok_or_else(|| failure(locale, id, "message must have a value"))?;
        let mut references = BTreeSet::new();
        let mut numbers = BTreeSet::new();
        pattern(value, &mut references, &mut numbers, 0)
            .map_err(|reason| failure(locale, id, &reason))?;
        parameters.insert(id.to_owned(), numbers);
        graph.insert(id.to_owned(), references);
        if graph.len() > 4096 {
            return Err(failure(locale, id, "catalog exceeds 4096 messages"));
        }
    }
    for (id, references) in &graph {
        for reference in references {
            if !graph.contains_key(reference) {
                return Err(failure(
                    locale,
                    id,
                    &format!("missing referenced message '{reference}' in this catalog"),
                ));
            }
        }
    }
    let mut heights = BTreeMap::new();
    for id in graph.keys() {
        visit(id, &graph, &mut BTreeSet::new(), &mut heights, 0)
            .map_err(|reason| failure(locale, id, &reason))?;
    }
    let mut ordered: Vec<_> = heights.into_iter().collect();
    ordered.sort_by_key(|(_, height)| *height);
    for (id, _) in ordered {
        for reference in &graph[&id] {
            let inherited = parameters[reference].clone();
            parameters
                .get_mut(&id)
                .expect("validated message")
                .extend(inherited);
        }
    }
    Ok(parameters)
}

fn failure(locale: &LocaleId, entry: &str, reason: &str) -> CatalogError {
    CatalogError::Validation {
        locale: locale.to_string(),
        entry: entry.to_owned(),
        reason: reason.to_owned(),
    }
}

fn visit(
    id: &str,
    graph: &BTreeMap<String, BTreeSet<String>>,
    path: &mut BTreeSet<String>,
    heights: &mut BTreeMap<String, usize>,
    depth: usize,
) -> Result<usize, String> {
    if depth > 32 {
        return Err("message reference depth exceeds 32".to_owned());
    }
    if let Some(height) = heights.get(id) {
        if depth + height > 32 {
            return Err("message reference depth exceeds 32".to_owned());
        }
        return Ok(*height);
    }
    if !path.insert(id.to_owned()) {
        return Err(format!("cyclic message reference at '{id}'"));
    }
    let mut height = 0;
    for reference in &graph[id] {
        height = height.max(visit(reference, graph, path, heights, depth + 1)? + 1);
    }
    path.remove(id);
    heights.insert(id.to_owned(), height);
    Ok(height)
}

fn pattern(
    value: &Pattern<&str>,
    references: &mut BTreeSet<String>,
    numbers: &mut BTreeSet<String>,
    depth: usize,
) -> Result<(), String> {
    if depth > 16 {
        return Err("expression nesting exceeds 16".to_owned());
    }
    for element in &value.elements {
        if let PatternElement::Placeable { expression } = element {
            match expression {
                Expression::Inline(value) => inline(value, references, numbers, depth + 1)?,
                Expression::Select { selector, variants } => {
                    inline(selector, references, numbers, depth + 1)?;
                    let mut keys = BTreeSet::new();
                    for variant in variants {
                        let key = match &variant.key {
                            VariantKey::Identifier { name } => (*name).to_owned(),
                            VariantKey::NumberLiteral { value } => {
                                numeric(value)?;
                                let number =
                                    value.parse::<f64>().map_err(|error| error.to_string())?;
                                if number == 0.0 {
                                    "0".to_owned()
                                } else {
                                    number.to_string()
                                }
                            }
                        };
                        if !keys.insert(key) {
                            return Err("duplicate select variant".to_owned());
                        }
                        pattern(&variant.value, references, numbers, depth + 1)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn inline(
    value: &InlineExpression<&str>,
    references: &mut BTreeSet<String>,
    numbers: &mut BTreeSet<String>,
    depth: usize,
) -> Result<(), String> {
    if depth > 16 {
        return Err("expression nesting exceeds 16".to_owned());
    }
    match value {
        InlineExpression::StringLiteral { .. } | InlineExpression::VariableReference { .. } => {
            Ok(())
        }
        InlineExpression::NumberLiteral { value } => numeric(value),
        InlineExpression::MessageReference {
            id,
            attribute: None,
        } => {
            references.insert(id.name.to_owned());
            Ok(())
        }
        InlineExpression::FunctionReference { id, arguments } => {
            if id.name != "NUMBER" || arguments.positional.len() != 1 {
                return Err("only NUMBER with one positional argument is supported".to_owned());
            }
            match &arguments.positional[0] {
                InlineExpression::VariableReference { id } => {
                    numbers.insert(id.name.to_owned());
                }
                InlineExpression::NumberLiteral { .. } => {}
                _ => return Err("NUMBER requires a numeric literal or variable".to_owned()),
            }
            inline(&arguments.positional[0], references, numbers, depth + 1)?;
            let mut names = BTreeSet::new();
            for argument in &arguments.named {
                if !names.insert(argument.name.name) {
                    return Err("duplicate NUMBER option".to_owned());
                }
                match (argument.name.name, &argument.value) {
                    (
                        "type",
                        InlineExpression::StringLiteral {
                            value: "cardinal" | "ordinal",
                        },
                    )
                    | (
                        "useGrouping",
                        InlineExpression::StringLiteral {
                            value: "true" | "false",
                        },
                    ) => {}
                    ("minimumFractionDigits", InlineExpression::NumberLiteral { value }) => {
                        if value.parse::<u8>().is_err()
                            || value.parse::<u8>().is_ok_and(|value| value > 6)
                        {
                            return Err(
                                "minimumFractionDigits must be an integer from 0 to 6".to_owned()
                            );
                        }
                    }
                    _ => {
                        return Err(format!(
                            "unsupported NUMBER option '{}'; supported: type, useGrouping, minimumFractionDigits",
                            argument.name.name
                        ));
                    }
                }
            }
            Ok(())
        }
        _ => Err("terms, attributes and nested inline placeables are not supported".to_owned()),
    }
}

pub(crate) fn numeric(value: &str) -> Result<(), String> {
    let number = value.parse::<f64>().map_err(|error| error.to_string())?;
    let fraction = value
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    if !number.is_finite()
        || number.abs() > 1_000_000_000_000.0
        || fraction > 6
        || value.contains(['e', 'E'])
    {
        return Err("numbers must be finite, at most 10^12 in magnitude, with at most six fractional digits".to_owned());
    }
    Ok(())
}
