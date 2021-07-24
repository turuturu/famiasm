use std::collections::HashMap;
use crate::insts::{RamAddress};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SymbolTable {
    table: HashMap<String, RamAddress>,
}
impl SymbolTable {
    pub fn new() -> SymbolTable{
        SymbolTable {
            table: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: String, val: RamAddress) -> Option<RamAddress>{
        self.table.insert(key, val)
    }
    pub fn get(&self, key:&String) -> Option<&RamAddress>{
        self.table.get(key)
    }
    pub fn contains(&self, key:&String)->bool{
        self.table.contains_key(key)
    }
}