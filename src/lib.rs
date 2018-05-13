//! Simple heapless command line interface parser for embedded devices
//! 
//! This crates makes use of a serial interface that implements the read trait
//! of the [`embedded-hal`] crate.
//! 
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
//! 
//! # Usage 
//! 
//! First define an instance of the CLI by initializing a [`LightCli`] as follows:
//! 
//! [`LightCli`]: struct.LightCli.html
//! 
//! ```
//! let mut cli : LightCli<U32> = LightCli::new();
//! ```
//! 
//! Periodically copy all contents of the serial device into the cli buffer by using 
//! the `fill` method, passing it the serial read instance `rx`, which implements
//! the embedded-hal Read<u8> interface:
//! 
//! ```
//! cli.fill(&mut rx).unwrap();
//! ```
//! 
//! Periodically parse the data in the buffer using the [`lightcli!`] macro:
//! 
//! [`lightcli!`]: macro.lightcli.html
//! 
//! ```
//! let mut name : String<U32> = String;:new();
//! 
//! lightcli!(cli, cmd, key, val, [
//!        "HELLO" => [
//!             "Name" => name = String::from(val)
//!         ] => {};
//!         "EHLO" => [
//!             ] => { writeln!(tx, "name: {}", name); }
//!         ],
//!         {}, {}, {}
//!     );
//! ```
//! 
//! # Examples
//! 
//! See the [examples] module.
//! 
//! [examples]: examples/index.html

#![no_std]

pub extern crate embedded_hal as hal;
pub extern crate nb;
pub extern crate heapless;

#[macro_use]
mod macros;
mod tokenizer;
mod lexer;

#[cfg(test)]
mod tests;

use tokenizer::{Tokenizer};
use lexer::Lexer;
use hal::serial::Read;
pub use lexer::CallbackCommand;
pub use heapless::consts::*;

pub struct LightCli<SLEN> where SLEN: heapless::ArrayLength<u8> {
    tokenizer: Tokenizer<SLEN>,
    lexer: Lexer<SLEN>
}

impl<SLEN : heapless::ArrayLength<u8>> LightCli<SLEN> {
    /// Create a new LightCLI instance.
    pub fn new() -> Self {
        LightCli {
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
