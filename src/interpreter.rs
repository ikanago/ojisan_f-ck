use crate::instractions::Instractions;
use crate::EvalError;
use std::convert::From;

#[derive(Debug)]
pub struct Interpreter {
    pub memory: Vec<u8>,
    pub instructions: Vec<Instractions>,
    pub memory_pointer: usize,
    pub insruction_pointer: usize,
    pub loop_stack: Vec<usize>,
}

impl Interpreter {
    /// Parse source code into intermediate representation of instruction ignoreing
    /// unexpected characters and return new `Interpreter`.
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
            loop_stack: Vec::new(),
        }
    }

    /// Evaluate parsed code.
    pub fn eval(&mut self) -> Result<(), EvalError> {
        while self.insruction_pointer < self.instructions.len() {
            match self.instructions[self.insruction_pointer] {
                Instractions::PtrIncr => self.pointer_increment()?,
                Instractions::PtrDecr => self.pointer_decrement()?,
                Instractions::ValIncr => self.value_increment()?,
                Instractions::ValDecr => self.value_decrement()?,
                Instractions::PutChar => self.put_char()?,
                Instractions::GetChar => self.get_char()?,
                Instractions::BeginLoop => self.begin_loop()?,
                Instractions::EndLoop => self.end_loop()?,
                Instractions::Nop => unreachable!(),
            };
            self.insruction_pointer += 1;
        }

        if self.loop_stack.len() != 0 {
            return Err(EvalError::UnbalancedBracket);
        }
        Ok(())
    }

    fn pointer_increment(&mut self) -> Result<(), EvalError> {
        match self.memory_pointer.checked_add(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => return Err(EvalError::MemoryOverflow),
        }
        Ok(())
    }

    fn pointer_decrement(&mut self) -> Result<(), EvalError> {
        match self.memory_pointer.checked_sub(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => return Err(EvalError::MemoryUnderflow),
        }
        Ok(())
    }

    fn value_increment(&mut self) -> Result<(), EvalError> {
        match self.memory[self.memory_pointer].checked_add(1) {
            Some(value) => self.memory[self.memory_pointer] = value,
            None => return Err(EvalError::MemoryOutOfRange),
        }
        Ok(())
    }

    fn value_decrement(&mut self) -> Result<(), EvalError> {
        match self.memory[self.memory_pointer].checked_sub(1) {
            Some(value) => self.memory[self.memory_pointer] = value,
            None => return Err(EvalError::MemoryOutOfRange),
        }
        Ok(())
    }

    fn put_char(&self) -> Result<(), EvalError> {
        let c = self.memory[self.memory_pointer] as char;
        print!("{}", c);
        Ok(())
    }

    fn get_char(&mut self) -> Result<(), EvalError> {
        Ok(())
    }

    fn begin_loop(&mut self) -> Result<(), EvalError> {
        self.loop_stack.push(self.insruction_pointer);
        if self.memory[self.memory_pointer] == 0 {
            self.jump_to_corresponding_loop_end()?;
        }
        Ok(())
    }

    fn jump_to_corresponding_loop_end(&mut self) -> Result<(), EvalError> {
        let mut instraction_pointer = self.insruction_pointer;
        while self.loop_stack.len() > 0 {
            if instraction_pointer >= self.instructions.len() {
                return Err(EvalError::UnbalancedBracket);
            }
            match self.instructions[instraction_pointer] {
                Instractions::BeginLoop => self.loop_stack.push(instraction_pointer),
                Instractions::EndLoop => instraction_pointer = self.loop_stack.pop().unwrap(),
                _ => continue,
            }
        }
        self.insruction_pointer = instraction_pointer;
        Ok(())
    }

    fn end_loop(&mut self) -> Result<(), EvalError> {
        let loop_begin = match self.loop_stack.pop() {
            Some(pointer) => pointer,
            None => return Err(EvalError::UnbalancedBracket),
        };
        if self.memory[self.memory_pointer] != 0 {
            self.insruction_pointer = loop_begin;
            self.loop_stack.push(loop_begin);
        }
        Ok(())
    }
}
