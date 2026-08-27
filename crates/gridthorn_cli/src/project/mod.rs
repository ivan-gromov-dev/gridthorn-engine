mod manifest;
mod name;
mod template;

pub(crate) use manifest::validate;
pub(crate) use name::validate as validate_name;
pub(crate) use template::{GeneratedProject, TemplateKind, render};
