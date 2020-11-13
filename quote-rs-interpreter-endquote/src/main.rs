/*
language specs:
    run by running this interpreter with the file path to the program as the first argument






*/



//shuddup rustc
#![allow(unused_variables)]
#![allow(unused_mut)]

//packages
use std::env;
use std::fs;
use std::io;

fn read_input() -> String {
    // supposed to read and return stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed reading input");
    return input;
}
fn reg_to_int(str: &str) -> usize {
    match str {
        "#0" => {
            return 0;
        }
        "#1" => {
            return 1;
        }
        "#2" => {
            return 2;
        }
        "#3" => {
            return 3;
        }
        "#4" => {
            return 4;
        }
        "#5" => {
            return 5;
        }
        "#6" => {
            return 6;
        }
        "#7" => {
            return 7;
        }
        _ => {
            println!("{}", "Registry must be #0 .. #7!");
            return 999;
        }
    }
}

fn main() {
    // Tack Isak för den fina inputkoden!
    // get input file
    let args: Vec<String> = env::args().collect();
    let lines = fs::read_to_string(&args[1]).expect("Failed to read file contents");
    
    // clean input 
    let mut lines: Vec<&str> = lines.split("\r\n").collect();
    lines.retain(|&s| !s.starts_with("//") && s != " ");

    // two bit registers
    // #0 is I/O
    let mut register = 0;
    // gee bill your mom lets you have seven instructions
    let instructions = vec!("read", "write","lookup", "exit"); //currently unused, may add validity checking later
    let mut i = 0;
    let mut jumper = 0;
    'main: while i < lines.len(){
        let mut components: Vec<&str> = lines[i].split(";").collect();
        //if it's empty we don't need to run all of this, optimization!
        if lines[i] != ""{
        match components[0] {
            //reads to #0
            "read" => {
                register = read_input().trim().parse::<i32>().unwrap();
            }
            //prints #0
            "write" => {
                println!("{}", register);
            }
         
            //exits program
            "exit" => {
                break 'main;
            }
            "lookup" => {
                match register{
                    0 => {
                        register = 1;
                    }
                    1 => {
                        register = 1;
                    }
                    2 => {
                        register = 2;
                    }
                    3 => {
                        register = 6;
                    }
                    4 => {
                        register = 24;
                    }
                    5 => {
                        register = 120;
                    }
                    6 => {
                        register = 720;
                    }
                    7 => {
                        register = 5040;
                    }
                    8 => {
                        register = 40320;
                    }
                    9 => {
                        register = 362880;
                    }
                    10 => {
                        register = 3628800;
                    }
                    11 => {
                        register = 39916800;
                    }
                    12 => {
                        register = 479001600;
                    }
                }
            }
            _ => {
                println!("{}", "Failed to read instruction");
                break 'main;
            } 
        }
        i += 1;
    }
    }
}