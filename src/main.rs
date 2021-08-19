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

        for (index, char) in code.bytes().enumerate() {
            self.index = self.index % 30_000;
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
                    self.loopvar += 1;
                    loopPoints.push(index);
                }
                b']' => {
                    // end of loop, go to its corresponding opening bracket
                }
                b',' => {
                    // getchar
                }
                b'.' => {
                    // putchar
                }

                _ => ()
            }
        }
    }
    
}


fn main() {
    
}

#[test]
fn parse() {
    let mut evaluator = Evaluator::new();
    let code = "++++.";
    evaluator.evaluate(code);
}

#[test]
fn string () {
    for i in b"hehe" {
        print!("{} ", i)
    }
    println!("h: {} ", b'h');
}