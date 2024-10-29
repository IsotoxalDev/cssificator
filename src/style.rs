use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

/// Style struct with all the declarations and the selector
pub struct Style {
    selector: String,
    declarations: HashMap<String, String>,
}

impl Style {
    /// Creates an returns an empty style property for the selector
    pub fn new(selector: &str) -> Self {
        Style {
            selector: selector.to_string(),
            declarations: HashMap::new(),
        }
    }

    /// Creates a style property with the given data
    pub fn from(selector: &str, declarations: HashMap<String, String>) -> Self {
        Style {
            selector: selector.to_string(),
            declarations,
        }
    }

    /// Adds a declaration to the style property
    pub fn add_declaration(&mut self, property: &str, value: &str) {
        self.declarations
            .insert(property.to_string(), value.to_string());
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let decl = self
            .declarations
            .iter()
            .map(|(property, value)| format!("\t{}: {};", property, value))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{} {{\n{}\n}}", self.selector, decl)
    }
}
