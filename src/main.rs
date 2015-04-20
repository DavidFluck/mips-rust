extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;

enum Instruction {
    IType(u8, u8, u8, i16),
    JType(u8, i32),
    RType(u8, u8, u8, u8, u8, u8),
}

impl Instruction {
    fn new(instr: u32) -> Instruction {
        let op: u8 = ((instr >> 26) & 0x3F) as u8;
        match instr {
            0b000000 => {
                /* IType instruction. */
                let opcode: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
                let rs: u8 = ((instr.clone() >> 21) & 0x1F) as u8;
                let rt: u8 = ((instr.clone() >> 16) & 0x1F) as u8;
                let immediate: i16 = (instr.clone() & 0xFFFF) as i16;

                Instruction::IType(opcode, rs, rt, immediate)
            },
            _ => panic!("Unrecognized opcode."),
        }
    }
}

fn main() {
    /* Open file. */
    let path = Path::new("bin");
    let display = path.display();

    /* Open the path in read-only mode, returns `IoResult<File>`. */
    let mut file = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => panic!("Couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut bytes: [u8; 4] = [0; 4];
    let result = match File::read(&mut file, &mut bytes) {
        Err(why) => panic!("Couldn't read bytes: {}", Error::description(&why)),
        Ok(bytes) => bytes,
    };

    let mut buf = Cursor::new(&bytes[..]);
    let num = buf.read_u32::<LittleEndian>().unwrap();
    println!("{}", num);

    let instruction = Instruction::new(0b00000000100001010011000000100000);
}
