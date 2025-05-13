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

fn main() {
    let mut vm = VM::new();
    let tokens = parse("test.bo");
    //let compiled = compile(tokens);
    //vm.give_data(compiled);
    //vm.run();
}
