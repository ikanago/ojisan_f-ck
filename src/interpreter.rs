use std::convert::From;
use crate::instractions::Instractions;

#[derive(Debug)]
pub struct Interpreter {
    pub memory: Vec<i64>,
    pub instructions: Vec<Instractions>,
    pub memory_pointer: usize,
    pub insruction_pointer: usize,
}

impl Interpreter {
    pub fn new(source: &str) -> Self {
        let mut instructions = Vec::new();
        for c in source.chars() {
            let instruction = Instractions::from(c);
            if instruction != Instractions::Nop {
                instructions.push(instruction);
            }
        }

        Self {
            instructions,
            memory: Vec::new(),
            memory_pointer: 0,
            insruction_pointer: 0,
        }
    }
}