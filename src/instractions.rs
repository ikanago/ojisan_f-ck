use std::str::FromStr;

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
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Instractions {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instraction = match s {
            "😅" => Instractions::PtrIncr,
            "😭" => Instractions::PtrDecr,
            "😘" => Instractions::ValIncr,
            "😚" => Instractions::ValDecr,
            "💦" => Instractions::PutChar,
            "⁉" => Instractions::GetChar,
            "✋" => Instractions::JmpLeft,
            "🤟" => Instractions::JmpRight,
            _ => return Err(ParseError),
        };
        Ok(instraction)
    }
}
