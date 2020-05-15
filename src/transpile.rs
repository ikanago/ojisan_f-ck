use crate::interpreter::Interpreter;

impl Interpreter {
    pub fn transpile_from(source: &str) -> Self {
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
        let transpiled_code = transpiled_code.into_iter().collect::<String>();
        Self::new(&transpiled_code)
    }
}
