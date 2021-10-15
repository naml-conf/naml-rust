use std::collections::HashMap;

/// The naml value enum represents all possible types of values that may be defined inside the naml format.
pub enum NamlValue {
    /// The string value representation inside the naml format may hold any form of character set parsable as a string.
    /// Strings in the naml format are always surrounded by double quotes.
    String(String),

    /// The integer value representation inside the naml format may hold any signed integer, covering the entire i64 spectrum.
    /// Any number that does not include a decimal dot will be interpreted as an integer.
    Integer(i64),

    /// The double value representation inside the naml format may hold any floating point value using 64 bits.
    /// Any number that includes a decimal dot will be interpreted as a double.
    Double(f64),

    /// The boolean value representation inside the naml format maybe hold either true or false, denoted by the single unquoted chars `y` and `n`.
    Boolean(bool),

    /// The node representation inside the naml format defines a new unique block/node in the element tree.
    NamlNode(NamlNodeElement),
}

/// The naml node element struct defines the in-memory representation of a single naml node inside the naml document.
#[derive(Default)]
pub struct NamlNodeElement {
    children: HashMap<String, NamlValue>,
}

impl NamlNodeElement {
    /// Looks up a child of the naml node element by the passed key and returns it.
    /// # Arguments
    ///
    /// * `key` - the key of the child that should be retrieved.
    pub fn child<K: AsRef<str>>(&self, key: K) -> Option<&NamlValue> {
        self.children.get(key.as_ref())
    }

    /// Returns a list of all children this node has.
    pub fn children(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }

    /// Inserts a new child node into the element.
    /// # Arguments
    ///
    /// * `key` - the key the child value should be registered under.
    /// * `value` - the actual value to insert into the node.
    pub fn insert<K: AsRef<str>>(&mut self, key: K, value: NamlValue) {
        self.children.insert(key.as_ref().parse().unwrap(), value);
    }
}
