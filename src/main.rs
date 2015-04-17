extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;

enum Opcode {
    ADD,
    ADDI,
}

/* I-type instruction struct. */
// struct IType {
//     opcode: Opcode,
//     rs: u8,
//     rt: u8,
//     immediate: i16,
// }
//
// /* J-type instruction struct. */
// struct JType {
//     opcode: Opcode,
//     target: i32,
// }
//
// /* R-type instruction struct. */
// struct RType {
//     opcode: Opcode,
//     rs: u8,
//     rt: u8,
//     rd: u8,
//     shamt: u8,
//     func: u8,
// }

struct IType {
    opcode: Opcode,
    rs: u8,
    rt: u8,
    immediate: i16,
}

impl IType {
    fn new(instr: u32) -> IType {
        /* IType instruction. */
    }
}

struct JType {
    opcode: Opcode,
    target: i32,
}

struct RType {
    opcode: Opcode,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    func: u8,
}

enum Instruction {
    IType,
    JType,
    RType,
}

fn decode(instr: u32) {
    /* Determine the type of instruction. */
    let op: u8 = ((instr >> 24) & 0x3F) as u8;
    let Instruction instr = match op {
        0 => println!("Add found."),
        _ => println!("Unrecognized opcode."),
    }
    println!("{}", op);
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

    decode(0b110110000000000000000000000000);

    // println!("{}", byte);
    //
    // for x in buf.iter() {
    //     print!("{:02x} ", x as i32);
    // }
    //
    // println!("");
}
