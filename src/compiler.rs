use std::io::prelude::*;
use std::io;

type Memtype = u8;
const MEMSIZE: usize = 30_000;
const ULIMIT:Memtype = 255;
const LLIMIT:Memtype = 0;

pub enum CompileError {
    ReadError,
    ItsTooBig,
    ItsTooSmall
}

fn rickRollCheck(code: &String) {
    if code.contains("rick") {
        // print "Never gonna give you up, never gonna let you down <3"
        print!("Never gonna give you up, never gonna let you down <3")
    }
}

pub fn evaluate(code: &String) -> Result<Vec<Memtype>, CompileError> {
    let mut memory: [Memtype; MEMSIZE] = [0; MEMSIZE];
    let mut address:usize = 0;
    let mut loopvar:usize = 0;

    let mut output: Vec<Memtype> = Vec::new();
    let loop_points = code
                        .bytes()
                        .enumerate()
                        .filter(|&(_, x)| x == b'[')
                        .map(|(i,_)|i)
                        .collect::<Vec<usize>>();
    let byte: Vec<Memtype> = code.bytes().collect();
    let mut code_index = 0usize;

    rickRollCheck(code);

    loop {
        if (code_index) >= byte.len() {
            break;
        }
        match byte.get(code_index) {
            Some(&char) => {
                match char {
                    b'+' => { 
                        // increment value at a memory location 
                        if memory[address] == ULIMIT {
                            return Err(CompileError::ItsTooBig)
                        } else {
                            memory[address] += 1;
                            code_index += 1;
                        }
                    }
                    b'-' => { 
                        // decrement value at the memory location
                        if memory[address] == LLIMIT {
                            return Err(CompileError::ItsTooSmall)
                        } else {
                            memory[address] -= 1;
                            code_index += 1;
                        }
                    }
                    b'>' => { 
                        // move right
                        address += 1;
                        address = address % MEMSIZE;
                        code_index += 1;
                    }
                    b'<' => {
                        // move left
                        if address == 0 {
                            address += MEMSIZE;
                        } 
                        address -= 1;
                        code_index += 1;
                    }
                    b'[' => {
                        // start the loop
                        loopvar += 1;
                        code_index += 1;
                    }
                    b']' => {
                        // end of loop, go to its corresponding opening bracket
                        loopvar -= 1;
                        if memory[address] == LLIMIT {
                            code_index += 1;
                        } else {
                            code_index = match loop_points.get(loopvar) {
                                Some(&i) => i,
                                None => panic!("")
                            };
                        }
                    }
                    b',' => {
                        // getchar
                        let mut buffer = [0u8];
                        match io::stdin().read(&mut buffer) {
                            Ok(_) => (),
                            Err(_) => return Err(CompileError::ReadError)
                        };
                        memory[address] = buffer[0];
                        code_index += 1;
                    }
                    b'.' => {
                        // putchar
                        output.push(memory[address]);
                        code_index += 1;
                    }

                    _ => code_index += 1
                }
            }
            None => {break;}
        }
    }
    Ok(output)
}