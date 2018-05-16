use heapless;
use nb;
use tokenizer;

use tokenizer::{Tokenizer};
use lexer::{Lexer, CallbackCommand};
use hal::serial::Read;

pub struct LightCliInput<SLEN> where SLEN: heapless::ArrayLength<u8> {
    tokenizer: Tokenizer<SLEN>,
    lexer: Lexer<SLEN>
}

impl<SLEN : heapless::ArrayLength<u8>> LightCliInput<SLEN> {
    /// Create a new LightCLI instance.
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            lexer: Lexer::new(),
        }
    }

    /// Try to parse as much data from the internal ring buffer as possible.
    /// 
    /// # Arguments
    /// * `callback` - This is the callback that will receive all parsing events.
    /// 
    /// # Remarks
    /// All commands are in the form "COMMAND KEY=VALUE". For every parsed key/value 
    /// pair the callback will be triggered with the current command string, current
    /// key and the corresponding value. When a newline is read the callback is 
    /// triggered with a command event.
    pub fn parse_data<CB>(&mut self, callback: CB) -> nb::Result<(), tokenizer::Error> 
        where CB: FnMut(CallbackCommand) -> () {
        self.lexer.parse_data(&mut self.tokenizer, callback)
    }

    /// Copy as many available bytes from `ser` into the buffer as possible.
    /// 
    /// # Arguments
    /// * `ser` - The serial interface to read from.
    /// 
    /// # Remarks
    /// 
    /// This will continue to try to read a byte from the serial device until the
    /// device returns `nb::Error::WouldBlock`.
    pub fn fill<E>(&mut self, ser: &mut Read<u8, Error=E>) -> nb::Result<(), E> {
        self.tokenizer.fill(ser)
    }
}