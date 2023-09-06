use std::collections::HashMap;

use crate::common::{Loc, Token, Program, assembler_loc_error, assembler_loc_warn, TokenType, OpType, ValueType};

#[derive(PartialEq)]
enum Section {
    Text,
    RoData
}

pub struct Tokeniser {
    loc: Loc,
    program: Program,
    code: String,
    section: Section,
    pub errors: usize,
}

impl Tokeniser {
    pub fn new(filename: String, code: String) -> Self {
        Self {
            loc: Loc(filename, 1),
            program: Program::new() ,
            code,
            section: Section::Text,
            errors: 0,

        }
    }

    pub fn parse(&mut self) -> anyhow::Result<()> {
        let lines: Vec<String> = self.code.lines().map(|f| f.to_string()).collect();

        for mut line in lines {
            line = self.split_comment(line);
            let words = shell_words::split(&line)?;

            if words.is_empty() {
                self.loc.1 += 1;
                continue;
            }
            // println!("{:?}", words);

            match words[0].as_str() {
                "section" => {
                    if words.len() < 2 {
                        assembler_loc_error(&self.loc, "No Section name provided");
                        self.errors += 1;
                        self.loc.1 += 1;
                        continue;
                    }
                    match words[1].as_str() {
                        ".text" => {
                            self.section = Section::Text;
                        }
                        ".rodata" => {
                            self.section = Section::RoData;
                        }
                        _ => {
                            assembler_loc_error(&self.loc, "Invalid section");
                            self.errors += 1;
                            self.loc.1 += 1;
                            continue;
                        }
                    }
                    self.loc.1 += 1;
                    continue;
                }
                "org" => {
                    if words.len() < 2 {
                        assembler_loc_error(&self.loc, "No origin address specified while defining it");
                        self.errors += 1;
                        self.loc.1 += 1;
                        continue;
                    }

                    let org_addr: u8 = parse_int::parse(&words[1])?;
                    self.program.origin = org_addr;
                    self.loc.1 += 1;
                    continue;
                }
                s if s.ends_with(':') => {
                    let label_name = &s[0..s.len()-1];

                    if self.section == Section::Text {

                        self.program.tokens.push(Token{
                            loc: self.loc.clone(),
                            typ: crate::common::TokenType::TextLabel(label_name.to_string()),
                            value: ValueType::Int(self.program.tokens.len() as u8)
                        });
                        self.program.labels.insert(label_name.to_string(), self.program.tokens.len() as u8);
                        if words.len() > 1 {
                            assembler_loc_warn(&self.loc, "Instructions on the same line as a label are not supported, please move the instruction to the next line or it will get ignored");
                        }
                        self.loc.1 += 1;
                        continue;
                    } else {
                        let data: Vec<i16> = if words.len() < 2 {
                            assembler_loc_error(&self.loc, "Label doesnt have any defined data");
                            self.errors += 1;
                            self.loc.1 += 1;
                            continue;
                        } else
                        if words.len() == 2 {
                            let int = parse_int::parse::<i16>(&words[1]);
                            match int {
                                Ok(i) => vec![i],
                                Err(_) => words[1].as_bytes().to_vec().into_iter().map(|f| f as i16).collect()
                            }
                        } else {
                            // words[1..]
                            todo!("Implement arrays of bytes")
                        };

                        self.program.data_labels.insert(label_name.to_string(), data);
                        self.loc.1 += 1;
                        continue;
                    }
                }
                _ => ()
            };

            let op_type = match OpType::from_str(&words[0]){
                Some(t) => t,
                None => {
                    assembler_loc_error(&self.loc, "Unknown operation");
                    self.errors += 1;
                    self.loc.1 += 1;
                    continue;
                }
            };


            let value = if words.len() < 2 {
                ValueType::None
            } else {
                let i = parse_int::parse::<u8>(&words[1]);
                match i {
                    Ok(i) => ValueType::Int(i),
                    Err(_) => ValueType::String(words[1].clone())
                }
            };
            self.program.tokens.push(Token { 
                loc: self.loc.clone(),
                typ: TokenType::Operation(op_type),
                value
            });

            self.loc.1 += 1;
        }
        Ok(())
    }


    pub fn get_program(&self) -> Program {
        self.program.clone()
    }

    pub fn split_comment(&mut self, line: String) -> String {
        
        let mut buf = String::new();
        
        let mut in_str = 0; // 1 == " 2 == '
        let mut last = '\0';
        for c in line.chars() {

            if c == '\'' {
                if in_str == 2 {
                    if last != '\\' {
                        in_str = 0;
                    }
                } else {
                    if in_str != 1 {
                        in_str = 2;
                    }
                }
            }

            if c == '\"' {
                if in_str == 1 {
                    if last != '\\' {
                        in_str = 0;
                    }
                } else {
                    if in_str != 2 {
                        in_str = 1;
                    }
                }
            }

            if in_str == 0 && c == ';' {
                break;
            }
            if c == '\\' {
                if last != '\\' {
                    buf.push(c);        
                } else {
                    buf.pop();
                }
            } else {
                
                buf.push(c);
            }
            last = c;
        }


        buf
    }


}


