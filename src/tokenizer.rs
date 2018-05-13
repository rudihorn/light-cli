use core;

use nb;
use heapless;

use hal::serial::Read;
use heapless::consts::*;
use heapless::RingBuffer;
use heapless::String;

#[derive(Debug)]
pub enum Error{
    InvalidUTF8,
    InvalidCount,
    Overflow
}

pub struct Tokenizer<SLEN> where SLEN: heapless::ArrayLength<u8> {
    rb: RingBuffer<u8, U64>,
    nextstr: String<SLEN>,
}

pub enum Token<'a> {
    NewLine,
    CarriageReturn,
    Equals,
    Space,
    Value(&'a str),
}

impl<SLEN> Tokenizer<SLEN>
where SLEN: heapless::ArrayLength<u8> {
    pub fn new() -> Self {
        Self {
            rb: RingBuffer::new(),
            nextstr: String::new(),
        }
    }

    fn get_amount(&mut self, count : usize) -> nb::Result<char, Error> {
        if count > 4 {
            Err(nb::Error::Other(Error::InvalidCount))
        }
        else if self.rb.len() < count {
            Err(nb::Error::WouldBlock)
        } else {
            let mut v : u32 = 0;

            for _ in 0..count {
                let b = self.rb.dequeue().unwrap() as u32;

                if b >> 3 == 0b11110 {
                    v = (v << 3) | (b & 0b111)
                } else if b >> 4 == 0b1110 {
                    v = (v << 4) | (b & 0b1111)
                } else if b >> 5 == 0b110 {
                    v = (v << 5) | (b & 0b11111)
                } else if b >> 6 == 0b10 {
                    v = (v << 6) | (b & 0b111111)
                } else {
                    v = b
                }
            }
            Ok(core::char::from_u32(v).unwrap())
        }
    }

    fn get_char(&mut self) -> nb::Result<char, Error> {
        if self.rb.is_empty() {
            Err(nb::Error::WouldBlock)
        } else {
            let b = *self.rb.iter().next().unwrap();

            if b >> 3 == 0b11110 as u8 {
                Ok(self.get_amount(4).unwrap())
            } else if b >> 4 == 0b1110 as u8 {
                Ok(self.get_amount(3).unwrap())
            } else if b >> 5 == 0b110 as u8 {
                Ok(self.get_amount(2).unwrap())
            } else if b >> 6 == 0b10 as u8 {
                Err(nb::Error::Other(Error::InvalidUTF8))
            } else {
                Ok(self.rb.dequeue().unwrap() as char)
            }
        }
    } 

    pub fn get_tokens<CB>(&mut self, mut callback : CB) -> nb::Result<(), Error> 
        where CB: FnMut(Token) -> () {


        loop {
            let c = match self.get_char() { 
                Err(e) => {
                    return Err(e)
                },
                Ok(c) => c
            };

            let send_val = |callback: &mut CB, s: &mut String<SLEN>| {
                if !s.is_empty() {
                    callback(Token::Value(s))
                }

                s.clear();
            };

            match c {
                ' ' => {
                    send_val(&mut callback, &mut self.nextstr);
                    callback(Token::Space)
                },
                '=' => {
                    send_val(&mut callback, &mut self.nextstr);
                    callback(Token::Equals)
                },
                '\r' => {
                    send_val(&mut callback, &mut self.nextstr);
                    callback(Token::CarriageReturn)
                }
                '\n' => {
                    send_val(&mut callback, &mut self.nextstr);
                    callback(Token::NewLine)
                },
                _ => {
                    match self.nextstr.push(c) {
                        // if we aren't able to push a char onto the string
                        // it probably means it is full 
                        Err(_) => return Err(nb::Error::Other(Error::Overflow)),
                        _ => ()
                    }
                }
            };

        }
    }

    pub fn fill<E>(&mut self, ser: &mut Read<u8, Error=E>) -> nb::Result<(), E> {
        loop {
            let r = ser.read();
            match r {
                Err(nb::Error::WouldBlock) => return Ok(()),
                Err(e) => return Err(e),
                Ok(c) => self.rb.enqueue(c).unwrap()
            }
        }
    }
}
