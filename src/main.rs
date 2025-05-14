mod chunk;
mod common;
mod compiler;
mod parser;
mod tokenizer;
mod value;
mod vm;

use compiler::compile;
use parser::parse;
use vm::VM;

fn main() -> Result<(), String> {
    let mut vm = VM::new();
    let mut tokens = parse("test.bo").unwrap();

    let mut function_signatures = Vec::new();
    let mut constants = Vec::new();
    let (chunks, main_loc) = compile(&mut tokens, &mut function_signatures, &mut constants)?;
    println!("{:?}", function_signatures);
    println!("{:?}", constants);
    for chunk in chunks {
        vm.give_data(chunk);
    }
    vm.give_constants(constants);
    vm.set_main(main_loc.unwrap());
    vm.run()?;

    Ok(())
}
