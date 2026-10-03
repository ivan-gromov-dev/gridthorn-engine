use super::CatalogError;
use crate::LocaleId;

pub(super) fn validate_parser_depth(locale: &LocaleId, source: &str) -> Result<(), CatalogError> {
    let mut depth = 0_usize;
    let mut expressions = Vec::<ExpressionDepth>::new();
    let mut quoted = false;
    let mut escaped = false;
    for line in source.lines() {
        if depth == 0 && line.trim_start().starts_with('#') {
            continue;
        }
        let mut previous = 0;
        for byte in line.bytes() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    quoted = false;
                }
                continue;
            }
            match byte {
                b'"' if expressions
                    .last()
                    .is_some_and(|expression| !expression.pattern) =>
                {
                    quoted = true;
                }
                b'{' => {
                    expressions.push(ExpressionDepth::default());
                    depth += 1;
                }
                b'(' => {
                    if let Some(expression) = expressions
                        .last_mut()
                        .filter(|expression| !expression.pattern)
                    {
                        expression.parentheses += 1;
                        depth += 1;
                    }
                }
                b')' => {
                    if let Some(expression) = expressions
                        .last_mut()
                        .filter(|expression| !expression.pattern && expression.parentheses > 0)
                    {
                        expression.parentheses -= 1;
                        depth -= 1;
                    }
                }
                b'}' => {
                    if let Some(expression) = expressions.pop() {
                        depth -= 1 + expression.parentheses;
                    }
                }
                b'>' if previous == b'-' => {
                    if let Some(expression) = expressions.last_mut() {
                        expression.pattern = true;
                    }
                }
                _ => {}
            }
            previous = byte;
            if depth > 32 {
                return Err(CatalogError::Validation {
                    locale: locale.to_string(),
                    entry: "<catalog>".to_owned(),
                    reason: "parser nesting exceeds 32".to_owned(),
                });
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct ExpressionDepth {
    pattern: bool,
    parentheses: usize,
}
