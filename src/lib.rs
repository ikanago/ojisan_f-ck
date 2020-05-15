pub mod instractions;
pub mod interpreter;
pub mod transpile;

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
