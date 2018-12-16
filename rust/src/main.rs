extern crate edn;
use std::io::{self, Read};

use edn::parser::Parser;

fn parse_line(s: String) {
    let mut parser = Parser::new(&s);
    println!("{:?}", parser.read());
}

fn main() {
    println!("Hello, world!");
    let instream = io::stdin();
    let mut repl_continue = true;
    let mut handle = instream.lock();
    while repl_continue {
        let mut buffer = String::new();
        let res = handle.read_to_string(&mut buffer);
        match res {
            Ok(line) => parse_line(buffer),
            Err(e) => println!("error: {}",e),
        }
        
    } ;
}
