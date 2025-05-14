#![feature(thin_box)]
mod chunk;
mod common;
mod compiler;
mod parser;
mod tokenizer;
mod value;
mod vm;

use crate::parser::Token;
use compiler::compile;
use parser::parse;
use parser::parse_line;
use std::env;
use std::io::stdin;
use std::time::SystemTime;
use vm::VM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);

    let mut function_signatures = Vec::new();
    let mut constants = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut vm = VM::new();

    if path.is_some() {
        // file read mode
        parse(path.unwrap(), &mut tokens)?;
        let (chunks, main_loc, main_type) =
            compile(&mut tokens, &mut function_signatures, &mut constants)?;

        for chunk in chunks {
            vm.give_data(chunk);
        }
        vm.update_constants(&constants);
        vm.set_main(main_loc.unwrap(), main_type.unwrap());

        println!("Executing");
        let exec_start = SystemTime::now();
        vm.run()?;
        let exec_end = SystemTime::now();
        println!(
            "\nExecuted in: {}ms\n",
            exec_end.duration_since(exec_start).unwrap().as_millis()
        );
    } else {
        let mut buffer = String::new();
        let mut next = String::new();
        loop {
            if next.trim().is_empty() && !buffer.trim().is_empty() {
                parse_line(&buffer, &mut tokens)?;
                tokens.reverse();
                buffer = String::new();
                next = String::new();
                let (chunks, main_loc, main_type) =
                    compile(&mut tokens, &mut function_signatures, &mut constants)?;

                for chunk in chunks {
                    vm.give_data(chunk);
                }
                vm.update_constants(&constants);
                if main_loc.is_some() {
                    vm.set_main(main_loc.unwrap(), main_type.unwrap());
                    vm.run()?;
                }
            } else {
                buffer.push_str(&next);
                next = String::new();
                stdin().read_line(&mut next).unwrap();
                if next == "exit\n" {
                    break;
                }
            }
        }
    }

    Ok(())
}
