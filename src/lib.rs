pub mod instructions;
pub mod interpreter;

/// This function transpiles brainfuck code to ojisanfuck code.
/// This is useful to generate ojisanfuck code.
pub fn transpile_from(source: &str) -> String {
    let mut transpiled_code = Vec::new();
    for c in source.chars() {
        match c {
            '>' => transpiled_code.push('😅'),
            '<' => transpiled_code.push('😭'),
            '+' => transpiled_code.push('😘'),
            '-' => transpiled_code.push('😚'),
            '.' => transpiled_code.push('💦'),
            ',' => transpiled_code.push('⁉'),
            '[' => transpiled_code.push('✋'),
            ']' => transpiled_code.push('🤟'),
            _ => continue,
        }
    }
    transpiled_code.into_iter().collect::<String>()
}

#[derive(Debug)]
pub enum EvalError {
    MemoryOverflow,
    MemoryUnderflow,
    MemoryOutOfRange,
    UnbalancedBracket,
    InputExhausted,
}
