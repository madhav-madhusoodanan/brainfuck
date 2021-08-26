use std::io;
use std::env;
use std::fs;
use std::time::{ Instant };
mod compiler;

fn main() {
    let args: Vec<String> = env::args()
                                .enumerate()
                                .filter(|&(index, _)| index != 0)
                                .map(|(_, elem)| elem)
                                .collect();

    let start = Instant::now();
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
                match compiler::evaluate(&input) {
                    Ok(chars) => {
                        for ch in chars {
                            print!("{}", ch as char);
                        }
                    }, 
                    Err(e) => {
                        match e {
                            compiler::CompileError::ItsTooSmall(_) => println!("Dude in {}, one memory cell just became negative", file),
                            compiler::CompileError::ItsTooBig(_) => println!("Dude in {}, one memory cell just became soo big", file),
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
        match compiler::evaluate(&input) {
            Ok(chars) => {
                for ch in chars {
                    print!("{}", ch as char);
                }
            }, 
            Err(e) => {
                match e {
                    compiler::CompileError::ItsTooSmall(_) => println!("Dude one memory cell just became negative"),
                    compiler::CompileError::ItsTooBig(_) => println!("Dude one memory cell just became soo big"),
                    _ => println!("Dude i cant process this idk why :(")
                }
            }
        }
    }
    let duration = start.elapsed().as_micros();
    println!("\n\nI burnt this code in {} microseconds :)", duration);
}

#[test]
fn parse() {
    let code = "++++++++++[>++++++++++<-]>.";
    assert_eq!(compiler::evaluate(&code.to_string()), Vec::from([100]));
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