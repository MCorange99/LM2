use std::collections::HashMap;

use super::Token;


#[derive(Debug, Clone)]
pub struct Program {
    pub tokens: Vec<Token>,
    pub origin: u8,
    pub data_labels: HashMap<String, Vec<i16>>,
    pub labels: HashMap<String, u8>
}

impl Program {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            origin: 0,
            data_labels: HashMap::new(),
            labels: HashMap::new()
        }
    }
}