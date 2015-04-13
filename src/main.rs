extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;

enum Opcode {
    Add = 0,
}

/* I-type instruction struct. */
struct IType {
    Opcode opcode,
    u8 rs,
    u8 rt,
    i16 immediate,
}

/* J-type instruction struct. */
struct  JType {
    Opcode opcode,
    i32 target,
}

/* R-type instruction struct. */
struct RType {
    Opcode opcode,
    u8 rs,
    u8 rt,
    u8 rd,
    u8 shamt,
    u8 func,
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

    // println!("{}", byte);
    //
    // for x in buf.iter() {
    //     print!("{:02x} ", x as i32);
    // }
    //
    // println!("");
}
