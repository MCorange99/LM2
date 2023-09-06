use std::{collections::HashMap, hash::Hash};

use crate::common::{Program, assembler_loc_error, Token, assembler_error};



pub struct Generator {
    buf: Vec<u8>,
    prog: Program,
    data_labels: HashMap<String, u8>,
    pub size: u8,
    ptr: usize
}

impl Generator {
    pub fn new(prog: Program) -> Self {
        Self {
            buf: vec![0; 240],
            prog,
            data_labels: HashMap::new(),
            size: 0,
            ptr: 0
        }
    }

    #[allow(overflowing_literals)]    
    fn generate_data_labels(&mut self) -> usize {
        let mut byte_counter = 240;

        for (k, mut v) in self.prog.data_labels.clone() {

            v.reverse();

            for i in v {
                let bytes = i.to_be_bytes();
                self.buf[byte_counter-2] = bytes[0];
                self.buf[byte_counter-1] = bytes[1];
                byte_counter -= 2;
            }

            self.data_labels.insert(k, byte_counter as u8);
        }
        240 - byte_counter
    }

    pub fn generate(&mut self) {

        self.size = self.generate_data_labels() as u8;



        for token in self.prog.tokens.clone() {

            match token.typ.clone() {
                crate::common::TokenType::Operation(op) => {
                    println!("{:?}", op);
                    self.size += 1;
                    match op {
                        crate::common::OpType::NOP => {
                            self.push(0x00);
                            self.push(0);
                        },
                        crate::common::OpType::LDA => {
                            self.push(0x01);
                            self.push_value(token);
                        },
                        crate::common::OpType::STA => {
                            self.push(0x02);
                            self.push_value(token);
                        },
                        crate::common::OpType::ADD => {
                            self.push(0x03);
                            self.push_value(token);
                        },
                        crate::common::OpType::SUB => {
                            self.push(0x04);
                            self.push_value(token);
                        },
                        crate::common::OpType::OUT => {
                            self.push(0x05);
                            self.push(0);
                        },
                        crate::common::OpType::JMP => {
                            self.push(0x06);
                            self.push_value(token);
                        },
                        crate::common::OpType::JN => {
                            self.push(0x07);
                            self.push_value(token);
                        },
                        crate::common::OpType::JZ => {
                            self.push(0x08);
                            self.push_value(token);
                        },
                        crate::common::OpType::JC => {
                            self.push(0x09);
                            self.push_value(token);
                        },
                        crate::common::OpType::INC => {
                            self.push(0x0A);
                            self.push_value(token);
                        },
                        crate::common::OpType::DEC => {
                            self.push(0x0B);
                            self.push_value(token);
                        },
                        crate::common::OpType::ADC => {
                            self.push(0x0C);
                            self.push_value(token);
                        },
                        crate::common::OpType::SBC => {
                            self.push(0x0D);
                            self.push_value(token);
                        },
                        crate::common::OpType::LDI => {
                            self.push(0x0E);
                            self.push_value(token);
                        },
                        crate::common::OpType::HLT => {
                            self.push(0x0F);
                            self.push(0);
                        },
                        crate::common::OpType::JO => {
                            self.push(0x10);
                            self.push_value(token);
                        },
                        crate::common::OpType::JSR => {
                            self.push(0x11);
                            self.push_value(token);
                        },
                        crate::common::OpType::RTS => {
                            self.push(0x12);
                            self.push(0);
                        },
                        crate::common::OpType::PHA => {
                            self.push(0x13);
                            self.push(0);
                        },
                        crate::common::OpType::PLA => {
                            self.push(0x14);
                            self.push(0);
                        },
                        crate::common::OpType::PRT => {
                            self.push(0x15);
                            self.push(0);
                        },
                        crate::common::OpType::AND => {
                            self.push(0x16);
                            self.push_value(token);
                        },
                        crate::common::OpType::OR => {
                            self.push(0x17);
                            self.push_value(token);
                        },
                        crate::common::OpType::XOR => {
                            self.push(0x18);
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
            crate::common::ValueType::Int(i) => self.push(i),
            crate::common::ValueType::String(s) => {
                let i = self.prog.labels.get(&s);
                let i = match i {
                    Some(i) => i,
                    None => {
                        match self.data_labels.get(&s){
                            Some(i) => i,
                            None => {
                                assembler_loc_error(&token.loc, "Unable to find label");
                                return;
                            }
                        }
                    },
                };
                self.push(*i)

            },
            crate::common::ValueType::None => self.push(0),
        }
    }

    pub fn get_bin(&mut self) -> Vec<u8> {
        self.buf.clone()
    }

    fn push(&mut self, b: u8) {
        self.buf[self.ptr] = b;
        self.ptr += 1;
    }
}
