mod isa;
mod assembler;
mod vm;
mod disassembler;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_help();
        return;
    }

    match args[1].as_str() {

        // Assemble
        "asm" => {

            if args.len() == 5 && args[3] == "-o" {

                assembler::assemble(&args[2], &args[4]);

            } else {

                println!("Usage:");
                println!("cargo run -- asm input.tasm -o output.tbc");
            }
        }

        // Run VM
        "run" => {

            if args.len() == 3 {

                vm::run(&args[2], false);

            } else if args.len() == 4 && args[2] == "--trace" {

                vm::run(&args[3], true);

            } else {

                println!("Usage:");
                println!("cargo run -- run file.tbc");
                println!("cargo run -- run --trace file.tbc");
            }
        }

        // Disassemble
        "dis" => {

            disassembler::disassemble(&args[2]);

        }

        _ => {
            print_help();
        }
    }
}

fn print_help() {

    println!("MiniVM");

    println!();

    println!("Commands:");

    println!("  asm <input.tasm> -o <output.tbc>");

    println!("  run <program.tbc>");

    println!("  run --trace <program.tbc>");

    println!("  dis <program.tbc>");
}