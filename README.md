# mips-rust
A MIPS R3000 emulator written in Rust.

What it says on the tin. I'm trying to learn Rust and figured a MIPS emulator would be a fun project to try.

## Current Functionality
At the moment, the R3000 emulator supports reading one instruction at a time from a binary file and disassembling it until EOF (for R-Type instructions only, at least). But all instructions will be loaded and turned into the appropriate
`Instruction` enum variant.

## Near-Future Functionality
My current goal is to begin working on some of the basic arithmetic instructions, followed by the load/store instructions. This will also involve modeling the entire CPU state, including a register file, main memory, and a timer (although I'll probably hold off on the timer for a little bit). I'll also need to eventually deal with branch and load delays.
