use hal::serial::Read;
use heapless::consts::*;
use heapless::RingBuffer;
use heapless::String;
use nb;

use LightCli;

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

    let cmd  = "HELLO TE❤ST=50\n";
    sb.write_str(&cmd);

    let mut cli : LightCli<U32> = LightCli::new();
    cli.fill(&mut sb).unwrap();

    let mut ran = false;

    let _ = cli.parse_data(|cmd,key,val| {
        assert!(cmd == "HELLO", "cmd={}", cmd);
        assert!(key == "TE❤ST", "key={}", key);
        assert!(val == "50", "val={}", val);
        ran = true;
    });
    
    assert!(ran);
}


#[test]
pub fn test_partial() {
    let mut sb = SerialBufferDevice {
        rb: RingBuffer::new()
    };

    let cmd  = "HELLO TE❤ST=50 Five=A";
    sb.write_str(&cmd);

    let mut cli : LightCli<U32> = LightCli::new();
    cli.fill(&mut sb).unwrap();

    let mut ran = false;

    // should only parse first argument, second is enver called
    let _ = cli.parse_data(|cmd,key,val| {
        assert!(cmd == "HELLO", "cmd={}", cmd);
        assert!(key == "TE❤ST", "key={}", key);
        assert!(val == "50", "val={}", val);
        ran = true;
    });
    
    assert!(ran);
}
