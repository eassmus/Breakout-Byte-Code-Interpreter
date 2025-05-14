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
use std::time::SystemTime;
use vm::VM;

fn main() -> Result<(), String> {
    let mut vm = VM::new();
    let mut tokens = parse("test.bo").unwrap();

    let mut function_signatures = Vec::new();
    let mut constants = Vec::new();
    let (chunks, main_loc) = compile(&mut tokens, &mut function_signatures, &mut constants)?;
    println!("Compiled Bytecode:");
    for chunk in chunks {
        println!("{:?}", chunk);
        vm.give_data(chunk);
    }
    println!("Executing");
    vm.give_constants(constants);
    vm.set_main(main_loc.unwrap());
    let exec_start = SystemTime::now();
    vm.run()?;
    let exec_end = SystemTime::now();
    println!(
        "\nExecuted in: {}ms\n",
        exec_end.duration_since(exec_start).unwrap().as_millis()
    );

    Ok(())
}
