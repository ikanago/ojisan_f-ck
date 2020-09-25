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
    instruction_pointer: usize,
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
            instruction_pointer: 0,
            input_buffer,
            output_buffer: Vec::new(),
            loop_stack: Vec::new(),
        }
    }

    /// Evaluate parsed code.
    pub fn eval(&mut self) -> Result<(), EvalError> {
        while self.instruction_pointer < self.instructions.len() {
            match self.instructions[self.instruction_pointer] {
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
            self.instruction_pointer += 1;
        }

        if self.loop_stack.len() != 0 {
            return Err(EvalError::UnbalancedBracket);
        }
        Ok(())
    }

    fn pointer_increment(&mut self) -> Result<(), EvalError> {
        match self.memory_pointer.checked_add(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => return Err(EvalError::MemoryOutOfRange),
        }
        Ok(())
    }

    fn pointer_decrement(&mut self) -> Result<(), EvalError> {
        match self.memory_pointer.checked_sub(1) {
            Some(memory_pointer) => self.memory_pointer = memory_pointer,
            None => return Err(EvalError::MemoryOutOfRange),
        }
        Ok(())
    }

    fn value_increment(&mut self) -> Result<(), EvalError> {
        let value = self.memory[self.memory_pointer].checked_add(1).unwrap_or(0);
        self.memory[self.memory_pointer] = value;
        Ok(())
    }

    fn value_decrement(&mut self) -> Result<(), EvalError> {
        let value = self.memory[self.memory_pointer]
            .checked_sub(1)
            .unwrap_or(std::u8::MAX);
        self.memory[self.memory_pointer] = value;
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
        self.loop_stack.push(self.instruction_pointer);
        if self.memory[self.memory_pointer] == 0 {
            self.instruction_pointer = self.corresponding_loop_end()?;
        }
        Ok(())
    }

    // Returns index of corresponding loop end.
    fn corresponding_loop_end(&mut self) -> Result<usize, EvalError> {
        let mut nest_level = 0;
        let mut instruction_pointer = self.instruction_pointer + 1;
        while nest_level > 0 || self.instructions[instruction_pointer] != Instructions::EndLoop {
            if self.instructions[instruction_pointer] == Instructions::BeginLoop {
                nest_level += 1;
            } else if self.instructions[instruction_pointer] == Instructions::EndLoop {
                nest_level -= 1;
            }
            instruction_pointer += 1;
            if instruction_pointer == self.instructions.len() {
                return Err(EvalError::UnbalancedBracket);
            }
        }
        Ok(instruction_pointer)
    }

    // By using stack to memorize index of `[`, jump to corresponding loop origin without scanning code again.
    fn end_loop(&mut self) -> Result<(), EvalError> {
        let loop_begin = match self.loop_stack.pop() {
            Some(pointer) => pointer,
            None => return Err(EvalError::UnbalancedBracket),
        };
        if self.memory[self.memory_pointer] != 0 {
            self.instruction_pointer = loop_begin;
            self.loop_stack.push(loop_begin);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::interpreter::Interpreter;

    #[test]
    fn overflowing_addtion() {
        // `+`
        let source = "😘";
        let mut interpreter = Interpreter::new(source, String::new());
        interpreter.memory[0] = 255;
        interpreter.eval().unwrap();
        assert_eq!(0, interpreter.memory[0]);
    }

    #[test]
    fn overflowing_subtraction() {
        // `-`
        let source = "😚";
        let mut interpreter = Interpreter::new(source, String::new());
        interpreter.eval().unwrap();
        assert_eq!(255, interpreter.memory[0]);
    }

    #[test]
    fn test_jump_to_loop_end() {
        // `[+]+
        let source = "✋😘🤟😘";
        let mut interpreter = Interpreter::new(source, String::new());
        assert_eq!(2, interpreter.corresponding_loop_end().unwrap());
    }

    #[test]
    fn test_jump_to_nested_loop_end() {
        // `[+++[>++<-]-]+`
        let source = "✋😘😘😘✋😅😘😘😭😚🤟😚🤟😘";
        let mut interpreter = Interpreter::new(source, String::new());
        assert_eq!(
            12,
            interpreter.corresponding_loop_end().unwrap(),
            "Matching outer loop"
        );
        interpreter.instruction_pointer = 4;
        assert_eq!(
            10,
            interpreter.corresponding_loop_end().unwrap(),
            "Matching inner loop"
        );
    }
    
    #[test]
    #[should_panic]
    fn test_lacking_end_of_loop() {
        // `[+++[>++<-]-`
        let source = "✋😘😘😘✋😅😘😘😭😚🤟😚";
        let mut interpreter = Interpreter::new(source, String::new());
        interpreter.eval().unwrap();
    }

    #[test]
    #[should_panic]
    fn test_lacking_begining_of_loop() {
        // `+[-]-]+`
        let source = "😘✋😚🤟😚🤟😘";
        let mut interpreter = Interpreter::new(source, String::new());
        interpreter.eval().unwrap();
    }

}
