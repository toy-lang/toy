use std::collections::HashMap;

use bincode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Bytecode {
    pub children: Vec<Command>
}

impl Bytecode {
    pub fn ser(&self) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
        return bincode::serialize(self);
    }
}

pub fn de(v: &Vec<u8>) -> Result<Bytecode, Box<bincode::ErrorKind>> {
    return bincode::deserialize(v);
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    ImportDyn(String),
    Return(Bytecode),
    Get(String),
    Set(String, Bytecode),
    Add(Bytecode, Bytecode),
    Subtract(Bytecode, Bytecode),
    Modulo(Bytecode, Bytecode),
    Negate(Bytecode),

    Not(Bytecode),
    And(Bytecode),
    Or(Bytecode),
    Greater(Bytecode),
    Lesser(Bytecode),

    Bool(bool),
    Number(i32),
    String(String),
    Array(Vec<Bytecode>),
    Tuple(Vec<Bytecode>),
    Map(HashMap<String, Bytecode>)
}