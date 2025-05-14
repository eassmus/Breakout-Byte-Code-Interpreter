#![feature(thin_box)]
mod chunk;
mod common;
mod compiler;
mod parser;
mod tokenizer;
mod value;
mod vm;

use compiler::compile;
use parser::parse;
use std::env;
use std::time::SystemTime;
use vm::VM;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => path,
        None => return Err("No source path given".to_string()),
    };
    let mut tokens = parse(path).unwrap();

    let mut function_signatures = Vec::new();
    let mut constants = Vec::new();
    let (chunks, main_loc, main_type) =
        compile(&mut tokens, &mut function_signatures, &mut constants)?;

    let mut vm = VM::new();
    for chunk in chunks {
        vm.give_data(chunk);
    }
    vm.give_constants(constants);
    vm.set_main(main_loc.unwrap(), main_type.unwrap());

    println!("Executing");
    let exec_start = SystemTime::now();
    vm.run()?;
    let exec_end = SystemTime::now();
    println!(
        "\nExecuted in: {}ms\n",
        exec_end.duration_since(exec_start).unwrap().as_millis()
    );

    Ok(())
}
