use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub enum Instractions {
    PtrIncr,
    PtrDecr,
    ValIncr,
    ValDecr,
    PutChar,
    GetChar,
    BeginLoop,
    EndLoop,
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
            '✋' => Instractions::BeginLoop,
            '🤟' => Instractions::EndLoop,
            _ => Instractions::Nop,
        }
    }
}
