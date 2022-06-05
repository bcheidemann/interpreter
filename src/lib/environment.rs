use std::collections::HashMap;

use super::parser::LiteralValue;

pub struct Environment {
    variables: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::from([(
                "VERSION".to_string(),
                LiteralValue::String(env!("CARGO_PKG_VERSION").to_string()),
            )]),
        }
    }

    pub fn resolve(&self, identifier: &String) -> &LiteralValue {
        self.variables.get(identifier).unwrap_or(&LiteralValue::Nil)
    }

    pub fn assign(&mut self, identifier: &String, value: LiteralValue) {
        self.variables.insert(identifier.to_string(), value);
    }
}
