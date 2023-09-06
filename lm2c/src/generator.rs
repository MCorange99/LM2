use crate::common::{Program, assembler_loc_error, Token};



pub struct Generator {
    buf: Vec<u8>,
    prog: Program
}

impl Generator {
    pub fn new(prog: Program) -> Self {
        Self {
            buf: Vec::new(),
            prog
        }
    }

    pub fn generate(&mut self) {
        for token in self.prog.tokens.clone() {

            match token.typ.clone() {
                crate::common::TokenType::Operation(op) => {
                    match op {
                        crate::common::OpType::NOP => {
                            self.buf.push(0x00);
                            self.buf.push(0);
                        },
                        crate::common::OpType::LDA => {
                            self.buf.push(0x01);
                            self.push_value(token);
                        },
                        crate::common::OpType::STA => {
                            self.buf.push(0x02);
                            self.push_value(token);
                        },
                        crate::common::OpType::ADD => {
                            self.buf.push(0x03);
                            self.push_value(token);
                        },
                        crate::common::OpType::SUB => {
                            self.buf.push(0x04);
                            self.push_value(token);
                        },
                        crate::common::OpType::OUT => {
                            self.buf.push(0x05);
                            self.buf.push(0);
                        },
                        crate::common::OpType::JMP => {
                            self.buf.push(0x06);
                            self.push_value(token);
                        },
                        crate::common::OpType::JN => {
                            self.buf.push(0x07);
                            self.push_value(token);
                        },
                        crate::common::OpType::JZ => {
                            self.buf.push(0x08);
                            self.push_value(token);
                        },
                        crate::common::OpType::JC => {
                            self.buf.push(0x09);
                            self.push_value(token);
                        },
                        crate::common::OpType::INC => {
                            self.buf.push(0x0A);
                            self.push_value(token);
                        },
                        crate::common::OpType::DEC => {
                            self.buf.push(0x0B);
                            self.push_value(token);
                        },
                        crate::common::OpType::ADC => {
                            self.buf.push(0x0C);
                            self.push_value(token);
                        },
                        crate::common::OpType::SBC => {
                            self.buf.push(0x0D);
                            self.push_value(token);
                        },
                        crate::common::OpType::LDI => {
                            self.buf.push(0x0E);
                            self.push_value(token);
                        },
                        crate::common::OpType::HLT => {
                            self.buf.push(0x0F);
                            self.buf.push(0);
                        },
                        crate::common::OpType::JO => {
                            self.buf.push(0x10);
                            self.push_value(token);
                        },
                        crate::common::OpType::JSR => {
                            self.buf.push(0x11);
                            self.push_value(token);
                        },
                        crate::common::OpType::RTS => {
                            self.buf.push(0x12);
                            self.buf.push(0);
                        },
                        crate::common::OpType::PHA => {
                            self.buf.push(0x13);
                            self.buf.push(0);
                        },
                        crate::common::OpType::PLA => {
                            self.buf.push(0x14);
                            self.buf.push(0);
                        },
                        crate::common::OpType::PRT => {
                            self.buf.push(0x15);
                            self.buf.push(0);
                        },
                        crate::common::OpType::AND => {
                            self.buf.push(0x16);
                            self.push_value(token);
                        },
                        crate::common::OpType::OR => {
                            self.buf.push(0x17);
                            self.push_value(token);
                        },
                        crate::common::OpType::XOR => {
                            self.buf.push(0x18);
                            self.push_value(token);
                        },
                    }
                },
                crate::common::TokenType::DataLabel(_) => (),
                crate::common::TokenType::TextLabel(_) => (),
            }
            // println!("{:?}", self.buf);
        }
    }

    pub fn push_value(&mut self, token: Token) {
        match token.value {
            crate::common::ValueType::Int(i) => self.buf.push(i),
            crate::common::ValueType::String(s) => {
                let i = self.prog.labels.get(&s);
                let i = match i {
                    Some(i) => i,
                    None => {
                        assembler_loc_error(&token.loc, "Unable to find label");
                        return;
                    },
                };
                self.buf.push(*i)

            },
            crate::common::ValueType::None => self.buf.push(0),
        }
    }

    pub fn get_bin(&mut self) -> Vec<u8> {
        self.buf.clone()
    }
}
