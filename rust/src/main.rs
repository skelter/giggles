extern crate edn;
extern crate rustyline;

use edn::parser::Parser;
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn parse_line(s: String) {
    let mut parser = Parser::new(&s);
    println!("{:?}", parser.read());
}

fn main() {
    println!("Hello, world!");

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("line ok: {}", line);
                parse_line(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted.");
                break
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(err) => {
                println!("error {}", err);
            }
        }
    }

}
