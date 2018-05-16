use nb;
use heapless::{ArrayLength, String};

use tokenizer;
use tokenizer::{Token, Tokenizer};

#[derive(Clone)]
#[derive(PartialEq)]
enum MachineState {
    NewCommand,
    NewCommandCR,
    Key,
    Value,
}

pub enum CallbackCommand<'a> {
    Attribute(&'a str, &'a str, &'a str),
    Command(&'a str),
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
        where CB: FnMut(CallbackCommand) -> () {
        tokenizer.get_tokens(|token| {
            let new_state = match token {
                Token::NewLine => {
                    if self.state != MachineState::NewCommandCR {
                        callback(CallbackCommand::Command(self.current_cmd.as_str()));
                        self.current_cmd.clear();
                        self.current_key.clear();
                    }
                    MachineState::NewCommand
                },
                Token::CarriageReturn => {
                    callback(CallbackCommand::Command(self.current_cmd.as_str()));
                    self.current_cmd.clear();
                    self.current_key.clear();
                    MachineState::NewCommandCR
                }, // ignore carriage returns
                Token::Value(s) => {
                    match self.state {
                        MachineState::NewCommandCR => {
                            self.current_cmd = String::from(s);
                            MachineState::Key
                        },
                        MachineState::NewCommand => {
                            self.current_cmd = String::from(s);
                            MachineState::Key
                        },
                        MachineState::Key => {
                            self.current_key = String::from(s);
                            MachineState::Value
                        },
                        MachineState::Value => {
                            callback(CallbackCommand::Attribute(self.current_cmd.as_str(), self.current_key.as_str(), s));
                            MachineState::Key
                        },
                    }
                },
                Token::Space => {
                    match self.state {
                        MachineState::Value => {
                            callback(CallbackCommand::Attribute(self.current_cmd.as_str(), self.current_key.as_str(), ""));
                            MachineState::Key
                        },
                        MachineState::NewCommand => self.state.clone(),
                        MachineState::NewCommandCR => self.state.clone(),
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