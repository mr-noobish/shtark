use std::io::{self, Write};
use environment::Environment;
use values::{ValueType, RuntimeVal, BooleanVal};
use ast::Stmt;
use interpreter::evaluate;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod values;
pub mod interpreter;
pub mod environment;
fn main() {
    let mut env = Environment::new(None);
    env.declare_var(String::from("true"), RuntimeVal::BooleanVal(BooleanVal {
        value_type: ValueType::Boolean,
        value: true,
    }), true);
    env.declare_var(String::from("false"), RuntimeVal::BooleanVal(BooleanVal {
        value_type: ValueType::Boolean,
        value: false,
    }), true);
    loop {
        print!(">>>");
        io::stdout().flush().unwrap();
        let mut input:String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: String = input.trim().to_owned();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }
        let mut program = parser::Parser::new(input);
        println!("{:#?}", evaluate(Stmt::Program(program.produce_ast()), &mut env));
    }
}