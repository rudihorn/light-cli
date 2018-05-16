use core;

use nb;
use hal::serial::Write;

use heapless::consts::*;
use heapless::RingBuffer;

pub struct LightCliOutput<'a, E: 'a> {
    rb: RingBuffer<u8, U128>,
    writer: &'a mut Write<u8, Error=E>
}

impl<'a, E> core::fmt::Write for LightCliOutput<'a, E> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            loop {
                if self.rb.enqueue(c.clone()).is_ok() {
                    break;
                } else {
                    match self.flush() {
                        Err(nb::Error::Other(_)) => return Err(core::fmt::Error),
                        _ => () // otherwise either non blocking or ok, so try to repeat
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a, E> LightCliOutput<'a, E> {
    /// Creates a now buffered console output instance. 
    /// 
    /// # Arguments
    /// * `writer`: The serial output instance, implementing the [`Write<u8>`] interface.
    /// 
    /// [`Write<u8>`]: ../embedded_hal/serial/trait.Write.html
    pub fn new(writer: &'a mut Write<u8, Error = E>) -> Self {
        Self {
            rb: RingBuffer::new(),
            writer: writer
        }
    }

    fn peek(&self) -> Option<u8> {
        match self.rb.iter().next() {
            None => None,
            Some(v) => Some(v.clone())
        }
    }

    /// Tries to send as many characters as it can until the interface
    /// starts blocking or there are no charactors to submit. 
    /// 
    /// # Remarks 
    /// 
    /// If the function returns Ok, then the buffer has succesfully been flushed
    /// whereas the error `WouldBlock` indicates that it is not empty
    /// but would have blocked if it tried to submit the character.
    /// 
    /// To completely empty the buffer, use `block!(cl_output.flush()).unwrap()`.
    pub fn flush(&mut self) -> nb::Result<(), E> {
        let mut co = self.peek();
        
        loop {
            match co {
                None => return Ok(()),
                Some(c) => {
                    let res = self.writer.write(c.clone());
                    match res {
                        Err(nb::Error::WouldBlock) => return Err(nb::Error::WouldBlock),
                        Err(nb::Error::Other(o)) => return Err(nb::Error::Other(o)),
                        Ok(()) => {
                            self.rb.dequeue().unwrap();
                            co = self.peek();
                        },
                    }
                }
            }
        }
    }
}