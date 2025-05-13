mod chunk;
mod common;
mod compiler;
mod parser;
mod tokenizer;
mod vm;

use parser::{parse, Token};
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let tokens = parse("test.bo");
    vm.run();
}
