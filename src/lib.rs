//! Simple heapless command line interface parser for embedded devices
//! 
//! This crates makes use of a serial interface that implements the read trait
//! of the [`embedded-hal`] crate.
//! 
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
//! 
//! # Usage 
//! 
//! First define an instance of the CLI by initializing a [`LightCliInput`] and a 
//! [`LightCliOutput`]. The output instance requires the serial write instance 
//! which should implement the embedded-hal [`Write<u8>`] trait:
//! 
//! [`LightCliInput`]: struct.LightCliInput.html
//! [`LightCliOutput`]: struct.LightCliOutput.html
//! [`Write<u8>`]: ../embedded_hal/serial/trait.Write.html
//! 
//! ```
//! let mut cl_in : LightCliInput<U32> = LightCliInput::new();
//! let mut cl_out = LightCliOutput::new(tx);
//! ```
//! 
//! Periodically copy all contents of the serial device into the cli buffer by using 
//! the [`fill`] method, passing it the serial read instance `rx`, which implements
//! the embedded-hal [`Read<u8>`] trait. In addition it is necessary to try to empty
//! the output buffer, by calling the [`flush`] method on the console output instance:
//! 
//! [`fill`]: struct.LightCliInput.html#method.fill
//! [`flush`]: struct.LightCliOutput.html#method.flush
//! [`Read<u8>`]: ../embedded_hal/serial/trait.Read.html
//! 
//! ```
//! let _ = cl_in.fill(&mut rx);
//! let _ = cl_out.flush();
//! ```
//! 
//! Periodically parse the data in the buffer using the [`lightcli!`] macro:
//! 
//! [`lightcli!`]: macro.lightcli.html
//! 
//! ```
//! let mut name : String<U32> = String;:new();
//! 
//! loop {
//!     /* fill, flush, etc. */
//! 
//!     lightcli!(cl_in, cl_out, cmd, key, val, [
//!         "HELLO" => [
//!             "Name" => name = String::from(val)
//!         ] => { writeln!(cl_out, "Name set").unwrap(); };
//!         "EHLO" => [
//!         ] => { writeln!(cl_out, "EHLO Name={}", name.as_str()).unwrap(); }
//!     ]);
//! }
//! ```
//! 
//! A serial communication may then look like:
//! 
//! ```
//! >> EHLO
//! << EHLO Name=
//! >> HELLO Name=Johnson
//! << Name set
//! >> EHLO
//! << EHLO Name=Johnson
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
mod output;
mod input;

#[cfg(test)]
mod tests;

pub use lexer::CallbackCommand;

pub use output::LightCliOutput;
pub use input::LightCliInput;

