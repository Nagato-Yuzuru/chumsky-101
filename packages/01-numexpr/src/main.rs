pub mod ast;
mod parser;

fn main() {
    parser::parser();
    println!("Hello, world!");
}
