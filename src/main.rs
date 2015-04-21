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

fn intToIType(instr: u32) -> Instruction {
    /* IType instruction. */
    let opcode: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
    let rs: u8 = ((instr.clone() >> 21) & 0x1F) as u8;
    let rt: u8 = ((instr.clone() >> 16) & 0x1F) as u8;
    let immediate: i16 = (instr.clone() & 0xFFFF) as i16;

    Instruction::IType(opcode, rs, rt, immediate)
}

fn intToJType(instr: u32) -> Instruction {
    /* JType instruction. */
    let opcode: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
    let target: i32 = (instr.clone() & 0x3FFFFFF) as i32;

    Instruction::JType(opcode, target)
}

fn intToRType(instr: u32) -> Instruction {
    /* RType instruction. */
    let opcode: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
    let rs: u8 = ((instr.clone() >> 21) & 0x1F) as u8;
    let rt: u8 = ((instr.clone() >> 16) & 0x1F) as u8;
    let rd: u8 = ((instr.clone() >> 11) & 0x1F) as u8;
    let shamt: u8 = ((instr.clone() >> 6) & 0x1F) as u8;
    let funct: u8 = (instr.clone() & 0x3F) as u8;

    Instruction::RType(opcode, rs, rt, rd, shamt, funct)
}

impl Instruction {
    fn new(instr: u32) -> Instruction {
        /* Now we can pass the unmodified instruction around. */
        let op: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
        println!("Op: {:08b}", op);
        match op {
            /* Special opcode. */
            0b000000 => {
                let funct: u8 = (instr.clone() & 0x3F) as u8;
                match funct {
                    /* ADD */
                    0b100000 => {
                        intToRType(instr)
                    },
                    _ => panic!("Unrecognized funct."),
                }
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

    let instruction = Instruction::new(0b00000000100001010011000000100000 as u32);
    println!("{:06b}", match instruction { Instruction::RType(_, _, _, _, _, funct) => funct, _ => panic!("Unrecognized taco."), });
}
