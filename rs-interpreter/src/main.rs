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

    // three bit registers
    // #0 is I/O, #1 is 0 (haha)
    let mut registers = vec!(0,0,0,0,0,0,0,0);
    // gee bill your mom lets you have six instructions
    let instructions = vec!("add","set", "read", "write","dont", "jump", "exit"); //currently unused, may add validity checking later
    let mut i = 0;
    let mut jumper = 0;
    'main: while i < lines.len(){
        let mut components: Vec<&str> = lines[i].split(";").collect();
        //if it's empty we don't need to run all of this, optimization!
        if lines[i] != ""{
        match components[0] {
            //sets r1 to r2 + r3
            "add" => {
                let mut regs = vec!(0, 0, 0);
                for i in 1..3{
                    regs [i-2] = reg_to_int(components[i]);
                }
                registers[regs[0]] = registers[regs[1]] + registers [regs[2]];
            }
            // li
            "set" => {
                let mut regs = reg_to_int(components[1]);
                registers[regs] = components[2].parse::<i32>().unwrap()
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
                let mut regs = vec!(0,0);
                for i in 1..2 {
                    regs[i-1] = reg_to_int(components[i])
                }
                if registers[regs[0]] > registers[regs[1]]{
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
}

