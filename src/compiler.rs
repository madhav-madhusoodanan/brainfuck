use std::io::prelude::*;
use std::io;

type Memtype = u8;
const MEMSIZE: usize = 30_000;
const ULIMIT:Memtype = 255;
const LLIMIT:Memtype = 0;

pub enum CompileError {
    ReadError,
    OppositesNotAttracted(usize)    // thats when the loop brackets are not properly matched
    // ItsTooBig(usize),
    // ItsTooSmall(usize),
}

/* thats just an easter egg */
fn rick_roll_check(code: &String) {
    if code.contains("rick") {
        println!("Never gonna give you up, never gonna let you down <3")
    }
}

pub fn evaluate(code: &String) -> Result<Vec<Memtype>, CompileError> {
    let bytes: Vec<Memtype> = code.bytes().collect();
    let mut memory = [LLIMIT; MEMSIZE];
    let mut address:usize = 0;
    let mut code_index = 0usize;
    let mut output: Vec<Memtype> = Vec::new();
    let mut loop_points:Vec<(usize, usize)> = Vec::new();

    /* checking for loop points, and verifying if the loop brackets are properly matched */
    {
        let mut loc:Vec<usize> = Vec::new();
        for (code_index, code) in bytes.iter().enumerate() {
            match code {
                b'[' => loc.push(code_index),
                    
                b']' => {
                        match loc.pop() {
                            Some(loc_start) => loop_points.push((loc_start, code_index)),
                            None => return Err(CompileError::OppositesNotAttracted(code_index))
                        };
                    }
                _ => continue
            }
        }
    }

    rick_roll_check(code);

    loop {
        match bytes.get(code_index) {
            Some(&char) => {
                match char {
                    b'+' => { 
                        // increment value at a memory location 
                        if memory[address] == ULIMIT {
                            memory[address] = LLIMIT;
                            code_index += 1;
                            // return Err(CompileError::ItsTooBig(code_index))
                        } else {
                            memory[address] += 1;
                            code_index += 1;
                        }
                    }
                    b'-' => { 
                        // decrement value at the memory location
                        if memory[address] == LLIMIT {
                            memory[address] = ULIMIT;
                            code_index += 1;
                            // return Err(CompileError::ItsTooSmall(address))
                        } else {
                            memory[address] -= 1;
                            code_index += 1;
                        }
                    }
                    b'>' => { 
                        // move right
                        if address + 1 == MEMSIZE {
                            address = 0;
                        } else {
                            address += 1
                        }
                        code_index += 1;
                    }
                    b'<' => {
                        // move left
                        if address == 0 {
                            address = MEMSIZE - 1;
                        } else {
                            address -= 1;
                        }
                        code_index += 1;
                    }
                    b'[' => {
                        // start the loop
                        if memory[address] > LLIMIT {
                            code_index += 1;
                        } else {
                            code_index = match loop_points.iter().find(|&&(start, _)| start == code_index){
                                Some(&(_, end)) => end,
                                None => panic!("")
                            };
                        }
                    }
                    b']' => {
                        // end of loop, go to its corresponding opening bracket
                        if memory[address] == LLIMIT {
                            code_index += 1;
                        } else {
                            code_index = match loop_points.iter().find(|&&(_, end)| end == code_index){
                                Some(&(start, _)) => start,
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