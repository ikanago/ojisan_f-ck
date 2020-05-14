use std::convert::From;

#[derive(Debug)]
pub enum Instractions {
    PtrIncr,
    PtrDecr,
    ValIncr,
    ValDecr,
    PutChar,
    GetChar,
    JmpLeft,
    JmpRight,
    Nop,
}

#[derive(Debug)]
pub struct ParseError;

impl From<char> for Instractions {
    fn from(c: char) -> Self {
        match c {
            '😅' => Instractions::PtrIncr,
            '😭' => Instractions::PtrDecr,
            '😘' => Instractions::ValIncr,
            '😚' => Instractions::ValDecr,
            '💦' => Instractions::PutChar,
            '⁉' => Instractions::GetChar,
            '✋' => Instractions::JmpLeft,
            '🤟' => Instractions::JmpRight,
            _ => Instractions::Nop,
        }
    }
}
