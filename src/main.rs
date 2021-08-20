use std::io::prelude::*;
use std::io;
use std::env;
use std::fs;

type Memtype = u8;
const MEMSIZE: usize = 30_000;
const ULIMIT:Memtype = 255;
const LLIMIT:Memtype = 0;

enum CompileError {
    ReadError,
    ItsTooBig,
    ItsTooSmall
}

struct Evaluator {
    memory: [Memtype; MEMSIZE],
    index: usize,
    loopvar: usize
}


impl Evaluator {
    fn new() -> Evaluator {
        Evaluator {
            memory: [0; MEMSIZE],
            index: 0usize,
            loopvar: 0usize,
        }
    }
    fn evaluate(&mut self, code: &String) -> Result<Vec<Memtype>, CompileError> {
        let mut output: Vec<Memtype> = Vec::new();
        let loop_points = code
                            .bytes()
                            .enumerate()
                            .filter(|&(_, x)| x == b'[')
                            .map(|(i,_)|i)
                            .collect::<Vec<usize>>();
        let byte: Vec<Memtype> = code.bytes().collect();
        let mut index = 0usize;
        loop {
            if (index) >= byte.len() {
                break;
            }
            match byte.get(index) {
                Some(&char) => {
                    match char {
                        b'+' => { 
                            // increment value at a memory location 
                            if self.memory[self.index] == ULIMIT {
                                return Err(CompileError::ItsTooBig)
                            } else {
                                self.memory[self.index] += 1;
                                index+= 1;
                            }
                        }
                        b'-' => { 
                            // decrement value at the memory location
                            if self.memory[self.index] == LLIMIT {
                                return Err(CompileError::ItsTooSmall)
                            } else {
                                self.memory[self.index] -= 1;
                                index+= 1;
                            }
                        }
                        b'>' => { 
                            // move right
                            self.index += 1;
                            self.index = self.index % MEMSIZE;
                            index+= 1;
                        }
                        b'<' => {
                            // move left
                            if self.index == 0 {
                                self.index += MEMSIZE;
                            } 
                            self.index -= 1;
                            index+= 1;
                        }
                        b'[' => {
                            // start the loop
                            self.loopvar += 1;
                            index+= 1;
                        }
                        b']' => {
                            // end of loop, go to its corresponding opening bracket
                            self.loopvar -= 1;
                            if self.memory[self.index] == LLIMIT {
                                index += 1;
                            } else {
                                index = match loop_points.get(self.loopvar) {
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
                            self.memory[self.index] = buffer[0];
                            index+= 1;
                        }
                        b'.' => {
                            // putchar
                            output.push(self.memory[self.index]);
                            index+= 1;
                        }

                        _ => index += 1
                    }
                }
                None => {break;}
            }
        }
        Ok(output)
    }
    
}
fn process(code: &String) -> Result<(), CompileError> {
    let mut evaluator = Evaluator::new();
    match evaluator.evaluate(code) {
        Ok(value) => {
            for ascii in value {
                print!("{}", ascii as char);
            };
            println!("");
            Ok(())
        },
        Err(e) => Err(e)
    }
}

fn main() {
    let args: Vec<String> = env::args()
                                .enumerate()
                                .filter(|&(index, _)| index != 0)
                                .map(|(_, elem)| elem)
                                .collect();
    if args.len() != 0 {
        for file in args{
                // read file
                let input = match fs::read_to_string(&file) {
                    Ok(input) => input,
                    Err(_) => {
                        println!("Dude i cant read {} file idk why :(", file);
                        continue;
                    }
                };

                // process
                match process(&input) {
                    Ok(_) => (), 
                    Err(e) => {
                        match e {
                            CompileError::ItsTooSmall => println!("Dude in {}, one memory cell just became negative", file),
                            CompileError::ItsTooBig => println!("Dude in {}, one memory cell just became soo big", file),
                            _ => println!("Dude i cant process {} file idk why :(", file)
                        }
                        continue;
                    }
                }

                
        }
    } else if args.len() == 0 {
        let mut input = String::new();

        // read line from stdin
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Dude i cant understand you :(");
            }
        };

        // process
        match process(&input) {
            Ok(_) => (), 
            Err(e) => {
                match e {
                    CompileError::ItsTooSmall => println!("Dude one memory cell just became negative"),
                    CompileError::ItsTooBig => println!("Dude one memory cell just became soo big"),
                    _ => println!("Dude i cant process this idk why :(")
                }
            }
        }
    }

}

#[test]
fn parse() {
    let mut evaluator = Evaluator::new();
    let code = "++++++++++[>++++++++++<-]>.";
    assert_eq!(evaluator.evaluate(&code.to_string()), Vec::from([100]));
}

#[test]
fn string () {
    for i in b"hehe" {
        print!("{} ", i)
    }
    println!("h: {} ", b'h');
}

#[test]
fn cycle() {
    let arr = [4, 2, 3];
    let mut iter = arr.iter().cycle();

    assert_eq!(iter.nth(0), Some(&4));
    assert_eq!(iter.nth(0), Some(&2));
    assert_eq!(iter.nth(0), Some(&3));
    assert_eq!(iter.nth(0), Some(&4));

}