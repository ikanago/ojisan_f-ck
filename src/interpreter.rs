use crate::instructions::Instructions;
use crate::EvalError;
use std::collections::VecDeque;
use std::convert::From;

/// Executes given ojisanf-ck code.
#[derive(Debug)]
pub struct Interpreter {
    // Memory cells.
    memory: Vec<u8>,
    // Array of instructions.
    instructions: Vec<Instructions>,
    // Current memory index.
    memory_pointer: usize,
    // Current instruction index.
    insruction_pointer: usize,
    // Buffer of user input. This is not interactive.
    input_buffer: VecDeque<u8>,
    // Buffer of output.
    pub output_buffer: Vec<char>,
    // Holds index of matching `[` to which `]` jumps back.
    loop_stack: Vec<usize>,
}

impl Interpreter {
    /// Parse source code into intermediate representation of instruction ignoreing
    /// unexpected characters and return new `Interpreter`.
    pub fn new(source: &str, buffer: String) -> Self {
        let instructions = source
            .chars()
            .filter_map(|c| {
                let instruction = Instructions::from(c);
                if instruction != Instructions::Nop {
                    Some(instruction)
                } else {
                    None
                }
            })
            .collect();
        let input_buffer: VecDeque<u8> = buffer.chars().map(|c| c as u8).collect();

        Self {
            memory: vec![0; 30_000],
            instructions,
            memory_pointer: 0,
            insruction_pointer: 0,
            input_buffer,
            output_buffer: Vec::new(),
            loop_stack: Vec::new(),
        }
    }

    /// Evaluate parsed code.
    pub fn eval(&mut self) -> Result<(), EvalError> {
        while self.insruction_pointer < self.instructions.len() {
            match self.instructions[self.insruction_pointer] {
                Instructions::PtrIncr => self.pointer_increment()?,
                Instructions::PtrDecr => self.pointer_decrement()?,
                Instructions::ValIncr => self.value_increment()?,
                Instructions::ValDecr => self.value_decrement()?,
                Instructions::PutChar => self.put_char()?,
                Instructions::GetChar => self.get_char()?,
                Instructions::BeginLoop => self.begin_loop()?,
                Instructions::EndLoop => self.end_loop()?,
                Instructions::Nop => unreachable!(),
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

    fn put_char(&mut self) -> Result<(), EvalError> {
        let c = self.memory[self.memory_pointer];
        self.output_buffer.push(c as char);
        Ok(())
    }

    fn get_char(&mut self) -> Result<(), EvalError> {
        self.memory[self.memory_pointer] = match self.input_buffer.pop_front() {
            Some(input) => input,
            None => return Err(EvalError::InputExhausted),
        };
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
        let mut instruction_pointer = self.insruction_pointer;
        while self.loop_stack.len() > 0 {
            if instruction_pointer >= self.instructions.len() {
                return Err(EvalError::UnbalancedBracket);
            }
            match self.instructions[instruction_pointer] {
                Instructions::BeginLoop => self.loop_stack.push(instruction_pointer),
                Instructions::EndLoop => instruction_pointer = self.loop_stack.pop().unwrap(),
                _ => continue,
            }
        }
        self.insruction_pointer = instruction_pointer;
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
