// We do not have a nested structure, because we are poping the scopes at compile time
// So we are just storing the current scope and the parent scopes

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Scope {
    // Current Base pointer offset
    current_offset: u32,

    locals: HashMap<String, u32>,

    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            current_offset: 0,
            locals: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(self) -> Self {
        Self {
            current_offset: self.current_offset,
            locals: HashMap::new(),
            parent: Some(Box::new(self)),
        }
    }

    pub fn pop_child(self) -> Option<Self> {
        match self.parent {
            Some(parent) => {
                let mut p = *parent;
                p.current_offset = self.current_offset; // Update the offset here
                Some(p)
            }
            None => None,
        }
    }

    pub fn push_variable(&mut self, name: String) -> u32 {
        let offset = self.current_offset;
        self.locals.insert(name, offset);
        self.current_offset += 4; // Advance 32 bits
        offset
    }

    pub fn get(&self, name: &str) -> Option<u32> {
        if let Some(offset) = self.locals.get(name) {
            return Some(*offset);
        }

        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        None
    }
}
