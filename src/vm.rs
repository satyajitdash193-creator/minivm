use std::fs;
fn trap(ip: usize, message: &str) -> ! {
    eprintln!("trap at ip=0x{:04X}: {}", ip, message);
    std::process::exit(1);
}


pub fn run(filename: &str, trace: bool) {

    println!("Loading {}", filename);

    let data = fs::read(filename)
        .expect("Failed to read bytecode file");

    if data.len() < 9 {
        panic!("Invalid bytecode file");
    }

    // Check Magic Number
    if &data[0..4] != b"MVM\0" {
        panic!("Invalid file format");
    }

    println!("Magic Number OK");

    let version = data[4];
    println!("Version: {}", version);

    let code_length = u32::from_le_bytes([
        data[5],
        data[6],
        data[7],
        data[8],
    ]);

    println!("Code Length: {}", code_length);

    // Program starts after 9-byte header
    let code = &data[9..];

    let mut pc: usize = 0;
    let mut stack: Vec<i64> = Vec::new();
    let mut memory = [0i64; 256];

    while pc < code.len() {

        let current_ip = pc;

        let opcode = code[pc];
        pc += 1;

        if trace {
            println!("ip={:04X} opcode={:02X} stack={:?}", current_ip, opcode, stack);
        }

        match opcode {

            // PUSH
            0x01 => {
                let value = i64::from_le_bytes([
                    code[pc],
                    code[pc + 1],
                    code[pc + 2],
                    code[pc + 3],
                    code[pc + 4],
                    code[pc + 5],
                    code[pc + 6],
                    code[pc + 7],
                ]);

                pc += 8;
                stack.push(value);
            }

            // POP
            0x02 => {
                stack.pop().expect("Stack underflow");
            }

            // DUP
            0x03 => {
                let value = *stack.last().expect("Stack underflow");
                stack.push(value);
            }

            // SWAP
            0x04 => {
                if stack.len() < 2 {
                    panic!("Stack underflow");
                }

                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();

                stack.push(a);
                stack.push(b);
            }

            // ADD
            0x10 => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                stack.push(a + b);
            }

            // SUB
            0x11 => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                stack.push(a - b);
            }

            // MUL
            0x12 => {
                let b = stack.pop().expect("Stack underflow");
                let a = stack.pop().expect("Stack underflow");

                stack.push(a * b);
            }

            // DIV
            0x13 => {
                let b = stack.pop().expect("Stack underflow");

                if b == 0 {
                    panic!("Division by zero");
                }

                let a = stack.pop().expect("Stack underflow");

                stack.push(a / b);
            }

            // MOD
            0x14 => {
                let b = stack.pop().expect("Stack underflow");

                if b == 0 {
                    panic!("Division by zero");
                }

                let a = stack.pop().expect("Stack underflow");

                stack.push(a % b);
            }

            // NEG
            0x15 => {
                let value = stack.pop().expect("Stack underflow");
                stack.push(-value);
            }

            // LOAD
            0x40 => {
                let slot = code[pc];
                pc += 1;

                stack.push(memory[slot as usize]);
            }

            // STORE
            0x41 => {
                let slot = code[pc];
                pc += 1;

                let value = stack.pop().expect("Stack underflow");

                memory[slot as usize] = value;
            }

            // PRINT
            0x60 => {
                let value = stack.pop().expect("Stack underflow");
                println!("{}", value);
            }

            // HALT
            0xFF => {
                println!("Program Finished");
                break;
            }

            _ => {
                panic!("Unknown opcode {:02X}", opcode);
            }
        }
    }
}