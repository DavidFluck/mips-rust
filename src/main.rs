extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Cursor;

/* Funct codes. */
const ADD: u8 = 0x20;
const ADDU: u8 = 0x21;
const AND: u8 = 0x24;
const BREAK: u8 = 0xD;
const DIV: u8 = 0x1A;
const DIVU: u8 = 0x1B;
const JALR: u8 = 0x9;
const JR: u8 = 0x8;
const MFHI: u8 = 0x10;
const MFLO: u8 = 0x12;
const MTHI: u8 = 0x11;
const MTLO: u8 = 0x13;
const MULT: u8 = 0x18;
const MULTU: u8 = 0x19;
const NOR: u8 = 0x27;
const OR: u8 = 0x25;
const SLL: u8 = 0x0;
const SLLV: u8 = 0x4;
const SLT: u8 = 0xA;
const SRA: u8 = 0x3;
const SRAV: u8 = 0x7;
const SRL: u8 = 0x2;
const SRLV: u8 = 0x6;
const SUB: u8 = 0x22;
const SUBU: u8 = 0x23;
const SYSCALL: u8 = 0xC;
const XOR: u8 = 0x26;

/* Opcodes. */
const ADDI: u8 = 0x8;
const ADDIU: u8 = 0x9;
const ANDI: u8 = 0xC;
const BEQ: u8 = 0x4;
const BGTZ: u8 = 0x7;
const BLEZ: u8 = 0x6;
const BNE: u8 = 0x5;
const REGIMM: u8 = 0x1;
const J: u8 = 0x2;
const JAL: u8 = 0x3;
const LB: u8 = 0x20;
const LBU: u8 = 0x24;
const LH: u8 = 0x21;
const LHU: u8 = 0x25;
const LUI: u8 = 0xF;
const LW: u8 = 0x23;
const LWL: u8 = 0x22;
const LWR: u8 = 0x26;
const ORI: u8 = 0xD;
const SB: u8 = 0x28;
const SH: u8 = 0x29;
const SLTI: u8 = 0xA;
const SLTIU: u8 = 0xB;
const SLTU: u8 = 0x2B;
const SPECIAL: u8 = 0x0;
const SW: u8 = 0x2B;
const SWL: u8 = 0x2A;
const SWR: u8 = 0x2E;
const XORI: u8 = 0xE;

/* Branch codes. */
const BGEZ: u8 = 0x1;
const BGEZAL: u8 = 0x11;
const BLTZ: u8 = 0x0;
const BLTZAL: u8 = 0x10;

/* TODO: Add function for bit shifting and masking. */

fn opcodeToString(opcode: u8) -> &'static str {
    match opcode {
        ADD => "ADD",
        ADDU => "ADDU",
        AND => "AND",
        DIV => "DIV",
        DIVU => "DIVU",
        _ => panic!("Unrecognized opcode."),
    }
}

fn functToString(funct: u8) -> &'static str {
    match funct {
        ADDI => "ADDI",
        ADDIU => "ADDIU",
        _ => panic!("Unrecognized funct code."),
    }
}

enum Instruction {
    IType(u8, u8, u8, i16),
    JType(u8, i32),
    RType(u8, u8, u8, u8, u8, u8),
}

impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            /* We need to check for the special opcode, since then we have to use funct to decide which instruction we have. */
            Instruction::RType(opcode, rs, rt, rd, shamt, funct) if opcode == SPECIAL => {
                write!(fmt, "{} ${}, ${}, ${}", functToString(funct), rs, rt, rd)
            },
            _ => panic!("BLAM"),
        }
    }
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
        let op: u8 = ((instr.clone() >> 26) & 0x3F) as u8;
        match op {
            /* Special opcode. */
            SPECIAL => {
                let funct: u8 = (instr.clone() & 0x3F) as u8;
                match funct {
                    /* ADD */
                    ADD | ADDU | AND => { intToRType(instr) },
                    _ => panic!("Unrecognized funct."),
                }
            },
            ADDI | ADDIU => { intToIType(instr) },

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

    let mut regFile: [i32; 32] = [0; 32];

    /* Should print ADD $4, $5, $6. */
    let instruction = Instruction::new(0b00000000100001010011000000100000 as u32);
    println!("{}", instruction);
}
