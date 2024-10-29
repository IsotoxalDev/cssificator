use std::fmt::{Display, Formatter, Result};
pub use style::Style;

mod style;

/// The base CSS struct which hoists a collection of styles
pub struct CSS {
    styles: Vec<Style>,
}

impl CSS {
    /// Creates and returns an empty CSS struct
    pub fn new() -> Self {
        CSS { styles: vec![] }
    }

    /// Adds a style rule to the CSS struct
    pub fn add_style(&mut self, style: Style) {
        self.styles.push(style);
    }
}

impl Display for CSS {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.styles
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<String>>()
                .join("\n\n")
        )
    }
}
