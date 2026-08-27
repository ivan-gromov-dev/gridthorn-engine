use anyhow::{Result, ensure};

pub(crate) fn validate(name: &str) -> Result<()> {
    ensure!(!name.is_empty(), "project name cannot be empty");
    ensure!(
        !name.starts_with(|character: char| character.is_ascii_digit()),
        "project name cannot start with a digit: {name}"
    );
    ensure!(
        name.chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_')),
        "project name may contain only ASCII letters, digits, `-`, and `_`: {name}"
    );
    Ok(())
}

#[cfg(test)]
mod test;
