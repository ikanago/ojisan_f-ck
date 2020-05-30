use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub enum Instructions {
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

impl From<char> for Instructions {
    fn from(c: char) -> Self {
        match c {
            '😅' => Instructions::PtrIncr,
            '😭' => Instructions::PtrDecr,
            '😘' => Instructions::ValIncr,
            '😚' => Instructions::ValDecr,
            '💦' => Instructions::PutChar,
            '⁉' => Instructions::GetChar,
            '✋' => Instructions::BeginLoop,
            '🤟' => Instructions::EndLoop,
            _ => Instructions::Nop,
        }
    }
}
