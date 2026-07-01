use std::fs;

pub fn disassemble(filename: &str) {

    let data = fs::read(filename)
        .expect("Failed to read bytecode file");

    if data.len() < 9 {
        panic!("Invalid bytecode file");
    }

    if &data[0..4] != b"MVM\0" {
        panic!("Invalid file format");
    }

    println!("Disassembly:\n");

    let code = &data[9..];

    let mut pc: usize = 0;

    while pc < code.len() {

        let opcode = code[pc];
        pc += 1;

        match opcode {

            // PUSH
            0x01 => {

                let value = i64::from_le_bytes([
                    code[pc],
                    code[pc+1],
                    code[pc+2],
                    code[pc+3],
                    code[pc+4],
                    code[pc+5],
                    code[pc+6],
                    code[pc+7],
                ]);

                pc += 8;

                println!("PUSH {}", value);
            }

            0x02 => println!("POP"),

            0x03 => println!("DUP"),

            0x04 => println!("SWAP"),

            0x10 => println!("ADD"),

            0x11 => println!("SUB"),

            0x12 => println!("MUL"),

            0x13 => println!("DIV"),

            0x14 => println!("MOD"),

            0x15 => println!("NEG"),

            0x40 => {

                let slot = code[pc];
                pc += 1;

                println!("LOAD {}", slot);
            }

            0x41 => {

                let slot = code[pc];
                pc += 1;

                println!("STORE {}", slot);
            }

            0x60 => println!("PRINT"),

            0xFF => {
                println!("HALT");
                break;
            }

            _ => {
                panic!("Unknown opcode {:02X}", opcode);
            }
        }
    }
}