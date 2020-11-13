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
fn main() {
    // Tack Isak för den fina inputkoden!
    // get input file
    let args: Vec<String> = env::args().collect();
    let lines = fs::read_to_string(&args[1]).expect("Failed to read file contents");
    
    // clean input 
    let mut lines: Vec<&str> = lines.split("\r\n").collect();
    lines.retain(|&s| !s.starts_with("//") && s != " ");

    // three bit registers
    // #0 is I/O
    let mut registers = vec!(0,0,0,0,0,0,0,0);
    // we only have four instructions for the same reason, and we don't even need one of them
    let instructions = vec!("add", "read", "write","dont", "jump", "exit"); //currently unused, may add validity checking later
    let mut i = 0;
    let mut jumper = 0;
    'main: while i < lines.len(){
        let mut components: Vec<&str> = lines[i].split(";").collect();
        match components[0] {
            //adds second given registry to first given registry
            "add" => {
                
            }
            //reads to #0
            "read" => {
                registers[0] = read_input().trim().parse::<i32>().unwrap();
            }
            //prints #0
            "write" => {
                println!("{}", registers[0]);
            }
            //ignores next jump if rs > rt condition is filled
            "dont" => {
                if components[1] > components[2]{
                    jumper = 1;
                }
            }
            //always jumps ten lines up
            "jump" => {
                if jumper == 0 {
                    if (i - 10) >= 1{
                        i -= 10;
                    }
                    else {
                        println!("{}", "Tried to jump out of bounds");
                        break 'main;
                    }
                }
                else {
                    jumper = 0;
                }
            }
            //exits program
            "exit" => {
                break 'main;
            }
            _ => {
                println!("{}", "Failed to read instruction");
                break 'main;
            } 
        }
        i += 1;
    }
}

