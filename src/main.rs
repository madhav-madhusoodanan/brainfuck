use std::io::prelude::*;
use std::io;
use std::env;
use std::fs;

enum CompileError {
    ParseError
}

struct Evaluator {
    memory: [u8; 30_000],
    index: usize,
    loopvar: usize
}


impl Evaluator {
    fn new() -> Evaluator {
        Evaluator {
            memory: [0u8; 30_000],
            index: 0usize,
            loopvar: 0usize,
        }
    }
    fn evaluate(&mut self, code: &String) -> Result<Vec<u8>, CompileError> {
        let mut output: Vec<u8> = Vec::new();
        let loop_points = code
                            .bytes()
                            .enumerate()
                            .filter(|&(_, x)| x == b'[')
                            .map(|(i,_)|i)
                            .collect::<Vec<usize>>();
        let byte: Vec<u8> = code.bytes().collect();
        let mut index = 0usize;
        loop {
            self.index = self.index % 30_000;
            if (index) >= byte.len() {
                break;
            }
            match byte.get(index) {
                Some(&char) => {
                    match char {
                        b'+' => { 
                            // increment value at a memory location 
                            self.memory[self.index] += 1;
                            index+= 1;
                        }
                        b'-' => { 
                            // decrement value at the memory location
                            self.memory[self.index] -= 1;
                            index+= 1;

                        }
                        b'>' => { 
                            // move right
                            self.index += 1;
                            index+= 1;
                        }
                        b'<' => { 
                            // move left
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
                            if self.memory[self.index] == 0 {
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
                            io::stdin().read(&mut buffer);
                            self.memory[self.index] = buffer[0];
                            index+= 1;
                        }
                        b'.' => {
                            // putchar
                            output.push(self.memory[self.index]);
                            index+= 1;
                        }

                        _ => ()
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
                let input = fs::read_to_string(&file).expect("ouch!");
                match process(&input) {
                    Ok(_) => (), 
                    Err(_) => panic!("Error")
                }

                
        }
    } else if args.len() == 0 {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => panic!("Error")
        };
        match process(&input) {
            Ok(_) => (), 
            Err(_) => panic!("Error")
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