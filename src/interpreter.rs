use crate::instractions::Instractions;
use std::convert::From;

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
            memory: vec![0; 256],
            instructions,
            memory_pointer: 0,
            insruction_pointer: 0,
        }
    }

    pub fn eval(&mut self) {
        while self.insruction_pointer < self.instructions.len() {
            match self.instructions[self.insruction_pointer] {
                Instractions::PtrIncr => self.pointer_increment(),
                Instractions::PtrDecr => self.pointer_decrement(),
                Instractions::ValIncr => self.value_increment(),
                Instractions::ValDecr => self.value_decrement(),
                Instractions::PutChar => self.put_char(),
                Instractions::GetChar => self.get_char(),
                Instractions::JmpLeft => self.jump_forward(),
                Instractions::JmpRight => self.jump_backward(),
                Instractions::Nop => unreachable!(),
            };
            self.insruction_pointer += 1;
        }
    }

    fn pointer_increment(&mut self) {
        match self.memory_pointer.checked_add(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => unimplemented!(),
        }
    }

    fn pointer_decrement(&mut self) {
        match self.memory_pointer.checked_sub(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => unimplemented!(),
        }
    }

    fn value_increment(&mut self) {
        match self.memory[self.memory_pointer].checked_add(1) {
            Some(value) => self.memory[self.memory_pointer] = value,
            None => unimplemented!(),
        }
    }

    fn value_decrement(&mut self) {
        match self.memory[self.memory_pointer].checked_sub(1) {
            Some(value) => self.memory[self.memory_pointer] = value,
            None => unimplemented!(),
        }
    }

    fn put_char(&mut self) {}
    fn get_char(&mut self) {}
    fn jump_forward(&mut self) {}
    fn jump_backward(&mut self) {}
}
