use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
// [program] [input_file] [extra flags]

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1])?;

    let mut code: Vec<u8> = Vec::new();

    let _ = file.read_to_end(&mut code)?;

    run(code);

    Ok(())
}

fn run(code: Vec<u8>) {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    let mut loop_stack: Vec<usize> = Vec::new();

    // brainfuck interpreter in rust by felix

    // > = increases memory pointer, or moves the pointer to the right 1 block.
    // < = decreases memory pointer, or moves the pointer to the left 1 block.
    // + = increases value stored at the block pointed to by the memory pointer
    // - = decreases value stored at the block pointed to by the memory pointer
    // [ = like c while(cur_block_value != 0) loop.
    // ] = if block currently pointed to's value is not zero, jump back to [
    // , = like c getchar(). input 1 character.
    // . = like c putchar(). print 1 character to the console
    let mut program_index = 0;

    while program_index < code.len() {
        let c = code[program_index] as char;
        match c {
            '>' => pointer = pointer.saturating_add(1).min(memory.len() - 1),
            '<' => pointer = pointer.saturating_sub(1),

            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),

            '[' => {
                if memory[pointer] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        program_index += 1;
                        if program_index >= code.len() {
                            panic!("Unmatched '['");
                        }
                        match code[program_index] as char {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                    }
                } else {
                    loop_stack.push(program_index);
                }
            }
            ']' => {
                if memory[pointer] != 0 {
                    if let Some(&start) = loop_stack.last() {
                        program_index = start;
                    } else {
                        panic!("Unmatched ']'");
                    }
                } else {
                    loop_stack.pop();
                }
            }

            '.' => {
                print!("{}", memory[pointer] as char);
                io::stdout().flush().unwrap();
            }

            ',' => {
                let mut buffer = [0];
                if io::stdin().read_exact(&mut buffer).is_ok() {
                    memory[pointer] = buffer[0];
                }
            }
            _ => {}
        }
        program_index += 1;
    }
}
