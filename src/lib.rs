pub mod instructions;
pub mod interpreter;

/// This function transpiles brainfuck code to ojisanfuck code.
/// This is useful to generate ojisanfuck code.
pub fn transpile_from(source: &str) -> String {
    source
        .chars()
        .filter_map(|c| match c {
            '>' => Some('😅'),
            '<' => Some('😭'),
            '+' => Some('😘'),
            '-' => Some('😚'),
            '.' => Some('💦'),
            ',' => Some('⁉'),
            '[' => Some('✋'),
            ']' => Some('🤟'),
            _ => None,
        })
        .collect::<String>()
}

#[derive(Debug)]
pub enum EvalError {
    MemoryOverflow,
    MemoryUnderflow,
    MemoryOutOfRange,
    UnbalancedBracket,
    InputExhausted,
}

#[cfg(test)]
mod tests {
    use crate::transpile_from;

    #[test]
    fn test_transpile() {
        let source = ">v<+a-.,[x]";
        let expected = "😅😭😘😚💦⁉✋🤟";
        assert_eq!(expected, transpile_from(source));
    }
}
