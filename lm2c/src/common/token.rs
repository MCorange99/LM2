use super::Loc;

#[derive(Debug, Clone)]
pub struct Token {
    pub loc: Loc,
    pub typ: TokenType,
    pub value: ValueType
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Int(u8),
    String(String),
    None
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    Operation(OpType),
    DataLabel(Vec<i16>),
    TextLabel(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OpType {
    NOP,
    LDA,
    STA,
    ADD,
    SUB,
    OUT,
    JMP,
    JN,
    JZ,
    JC,
    INC,
    DEC,
    ADC,
    SBC,
    LDI,
    HLT,
    JO,
    JSR,
    RTS,
    PHA,
    PLA,
    PRT,
    AND,
    OR,
    XOR
}

impl OpType {
    
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        let t = match s.to_lowercase().as_str() {
            "nop" => Self::NOP,
            "lda" => Self::LDA,
            "sta" => Self::STA,
            "add" => Self::ADD,
            "sub" => Self::SUB,
            "out" => Self::OUT,
            "jmp" => Self::JMP,
            "jn" => Self::JN,
            "jz" => Self::JZ,
            "jc" => Self::JC,
            "inc" => Self::INC,
            "dec" => Self::DEC,
            "adc" => Self::ADC,
            "sbc" => Self::SBC,
            "ldi" => Self::LDI,
            "hlt" => Self::HLT,
            "jo" => Self::JO,
            "jsr" | "call" => Self::JSR,
            "rts" => Self::RTS,
            "pha" => Self::PHA,
            "pla" => Self::PLA,
            "prt" => Self::PRT,
            "and" => Self::AND,
            "or" => Self::OR,
            "xor" => Self::XOR,
            _ => return None
        };

        Some(t)
    }
}
