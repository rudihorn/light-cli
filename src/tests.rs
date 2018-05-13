
use hal::serial::Read;
use heapless::consts::*;
use heapless::RingBuffer;
use heapless::String;
use nb;

use LightCli;
use CallbackCommand;

pub struct SerialBufferDevice {
    pub rb: RingBuffer<u8, U512>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    None
}

impl Read<u8> for SerialBufferDevice {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
       match self.rb.dequeue(){
           Some(a) => Ok(a),
           None => Err(nb::Error::WouldBlock)
       }
    }
} 

impl SerialBufferDevice {
    pub fn write_str(&mut self, s: &str) {
        let s : String<U128> = String::from(s);
        self.write(s.as_bytes());
    }

    #[allow(dead_code)]
    pub fn write_one(&mut self, b: u8) {
        self.rb.enqueue(b).unwrap()
    }

    pub fn write(&mut self, dat: &[u8]) {
        for b in dat {
            self.rb.enqueue(*b).unwrap()
        }
    }
}

#[test]
pub fn test1() {
    let mut sb = SerialBufferDevice {
        rb: RingBuffer::new()
    };
    let mut cli : LightCli<U32> = LightCli::new();

    sb.write_str("HELLO TE❤ST=50\n");
    cli.fill(&mut sb).unwrap();

    let mut ran = false;
    let mut done = false;

    let _ = cli.parse_data(|cbcmd| {
        match cbcmd {
            CallbackCommand::Attribute(cmd, key, val) => {
                assert!(cmd == "HELLO", "cmd={}", cmd);
                assert!(key == "TE❤ST", "key={}", key);
                assert!(val == "50", "val={}", val);
                ran = true;
            },
            CallbackCommand::Command(cmd) => {
                assert!(cmd == "HELLO", "cmd={}", cmd);
                assert!(!done);
                done = true;
            }
        }
    });
    
    assert!(ran);
    assert!(done);
}


#[test]
pub fn test_partial() {
    let mut sb = SerialBufferDevice {
        rb: RingBuffer::new()
    };
    let mut cli : LightCli<U32> = LightCli::new();

    // fill the buffer with some data
    sb.write_str("HELLO TE❤ST=50 Five=A");
    cli.fill(&mut sb).unwrap();

    let mut ran = false;
    let mut done = false;

    // should only parse first argument, second is enver called
    let _ = cli.parse_data(|cbcmd| {
        match cbcmd {
            CallbackCommand::Attribute(cmd, key, val) => {
                assert!(cmd == "HELLO", "cmd={}", cmd);
                assert!(key == "TE❤ST", "key={}", key);
                assert!(val == "50", "val={}", val);
                ran = true;
            },
            CallbackCommand::Command(_cmd) => {
                assert!(false, "Command isn't finished.");
            }
        }
    });

    assert!(ran);
    
    // finish command
    sb.write_str("\n");
    cli.fill(&mut sb).unwrap();

    ran = false;

    let _ = cli.parse_data(|cbcmd| {
        match cbcmd {
            CallbackCommand::Attribute(cmd, key, val) => {
                assert!(cmd == "HELLO", "cmd={}", cmd);
                assert!(key == "Five", "key={}", key);
                assert!(val == "A", "val={}", val);
                ran = true;
            },
            CallbackCommand::Command(cmd) => {
                assert!(cmd == "HELLO", "cmd={}", cmd);
                done = true;
            }
        }
    });

    assert!(ran);
    assert!(done);
}

#[test]
pub fn test_macro() {

    let mut sb = SerialBufferDevice {
        rb: RingBuffer::new()
    };
    let mut cli : LightCli<U32> = LightCli::new();

    sb.write_str("HELLO Name=Foo\n");
    cli.fill(&mut sb).unwrap();

    let mut ran = false;
    let mut done = false;

    let mut name : String<U32> = String::new();

    lightcli!(cli, cmd, key, val, [
            "HELLO" => [
                "Name" => {
                    ran = true;
                    name = String::from(val)
                }
            ] => {assert!(ran); done = true};
            "EHLO" => [
            ] => assert!(name == "Foo", "name = {}", name.as_str())
        ],
        assert!(false, "Unknown cmd {}", cmd),
        assert!(false, "Unknown key {} for cmd {}", key, cmd), 
        assert!(false, "Unknown cmd done {}", cmd)
    );
    
    assert!(ran);
    assert!(done);
}
