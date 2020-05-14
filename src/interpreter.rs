use crate::instractions::Instractions;

#[derive(Debug)]
pub struct Interpreter {
    pub source: Vec<char>,
    pub memory: Vec<i64>,
    pub instructions: Vec<Instractions>,
    pub memory_pointer: usize,
    pub insruction_pointer: usize,
}

impl Interpreter {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect::<Vec<char>>(),
            memory: Vec::new(),
            instructions: Vec::new(),
            memory_pointer: 0,
            insruction_pointer: 0,
        }
    }
}