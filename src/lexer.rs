use nb;
use heapless::{ArrayLength, String};

use tokenizer;
use tokenizer::{Token, Tokenizer};

#[derive(Clone)]
enum MachineState {
    NewCommand,
    Key,
    Value,
}

pub struct Lexer<SLEN> where SLEN: ArrayLength<u8> {
    current_cmd: String<SLEN>,
    current_key: String<SLEN>,
    state: MachineState,
}

impl<SLEN> Lexer<SLEN> where SLEN: ArrayLength<u8> {
    pub fn new() -> Self {
        Self {
            current_cmd: String::new(),
            current_key: String::new(),
            state: MachineState::NewCommand,
        }
    }

    pub fn parse_data<CB>(&mut self, tokenizer: &mut Tokenizer<SLEN>, mut callback: CB) -> nb::Result<(), tokenizer::Error> 
        where CB: FnMut(&str, &str, &str) -> () {
        tokenizer.get_tokens(|token| {
            let new_state = match token {
                Token::NewLine => MachineState::NewCommand,
                Token::CarriageReturn => self.state.clone(), // ignore carriage returns
                Token::Value(s) => {
                    match self.state {
                        MachineState::NewCommand => {
                            self.current_cmd = String::from(s);
                            MachineState::Key
                        },
                        MachineState::Key => {
                            self.current_key = String::from(s);
                            MachineState::Value
                        },
                        MachineState::Value => {
                            callback(self.current_cmd.as_str(), self.current_key.as_str(), s);
                            MachineState::Key
                        },
                    }
                },
                Token::Space => {
                    match self.state {
                        MachineState::Value => {
                            callback(self.current_cmd.as_str(), self.current_key.as_str(), "");
                            MachineState::Key
                        },
                        MachineState::NewCommand => self.state.clone(),
                        MachineState::Key => self.state.clone()
                    }
                },
                Token::Equals => {
                    self.state.clone()
                }
            };

            self.state = new_state;
        })
    }
}