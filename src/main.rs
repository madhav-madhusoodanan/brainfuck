use std::io::prelude::*;
use std::io;
use std::env;
use std::fs;

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
    fn evaluate(&mut self, code: &str) {
        let mut loopPoints: Vec<usize> = Vec::new();
        let byte: Vec<u8> = code.bytes().collect();
        let mut index = 0usize;
        loop {
            self.index = self.index % 30_000;
            index = index % byte.len();
            match byte.get(index) {
                Some(&char) => {
                    match char {
                        b'+' => { 
                            // increment value at a memory location 
                            self.memory[self.index] += 1;
                        }
                        b'-' => { 
                            // decrement value at the memory location
                            self.memory[self.index] -= 1;
                        }
                        b'>' => { 
                            // move right
                            self.index += 1;
                        }
                        b'<' => { 
                            // move left
                            self.index -= 1
                        }
                        b'[' => {
                            // start the loop
                            loopPoints.push(self.loopvar);
                            self.loopvar += 1;
                        }
                        b']' => {
                            // end of loop, go to its corresponding opening bracket
                            self.loopvar -= 1;
                            if self.memory[self.index] == 0 {
                                index += 1;
                            } else {
                                index = match loopPoints.get(self.loopvar) {
                                    Some(&i) => i,
                                    None => panic!("")
                                }
                            }
                        }
                        b',' => {
                            // getchar
                            let mut buffer = [0u8];
                            io::stdin().read(&mut buffer);
                            self.memory[self.index] = buffer[0];
                        }
                        b'.' => {
                            // putchar
                            print!("{}", self.memory[self.index] as char);
                        }

                        _ => ()
                    }
                }
                None => ()
            }
        }
    }
    
}


fn main() {
    
}

// #[test]
// fn parse() {
//     let mut evaluator = Evaluator::new();
//     let code = "++++.";
//     evaluator.evaluate(code);
// }

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