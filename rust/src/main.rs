extern crate edn;
extern crate rustyline;

use edn::parser::Parser;
use edn::Value;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;

//todo could be more idiomatic way to do this
fn rest<T: Clone>(v: &Vec<T>) -> Vec<T> {
    let mut new_vector = v.clone();
    new_vector.remove(0);
    new_vector
}

fn parse_line(s: String) -> edn::Value {
    let mut parser = Parser::new(&s);
    match parser.read() {
        Some(Ok(some)) => some,
        _ => edn::Value::Nil
    }
}

struct Function {
    name : String
}

fn apply(f: &Function, params: Vec<Value>) -> Value {
    println!("apply f{} ", f.name);
    Value::String(f.name.to_string())
}

fn eval_list(symbol_table: &HashMap<String,Function>, v : Vec<Value> ) -> Result<Value,String> {
    // hmm.  should we eval all the args?
    println!("Evaluating a list");
    match v[0] {
        edn::Value::Symbol(ref symname) => {
            println!("found a symbol {}", symname);
            match symbol_table.get(symname) {
                Some(f) => {
                    let params = rest(&v);
                    Ok (apply(f, params))
                }
                None => Err(format!("No such symbol defined: {}", symname))
            }
                
        }
       _ => Err("Typically we want a symbol".to_string())

    }

}

fn eval(symbol_table: &HashMap<String,Function>, v : edn::Value) -> Result<Value,String> {
    match v {
        edn::Value::List(l) => {
            eval_list(symbol_table, l)
        }
        _ => Ok(v)
    }
}

fn parse_eval_line(symbol_table: &HashMap<String,Function>, s: String) -> Result<Value,String> {
    let parsed = parse_line(s);
    eval(symbol_table, parsed)
}

fn to_string(v : Value) -> String {
    match v {
        Value::String(ref s) => format!("\"{}\"",s),
        Value::Symbol(name) => name,
        Value::List(v) => "vec".to_string(),
        _ => "nyi".to_string()
    }
}

fn main() {
    println!("Hello, world!");

    let symbol_table: HashMap<String,Function> = HashMap::new();

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("line ok: {}", line);
                match parse_eval_line(&symbol_table, line) {
                    Ok(retval) => {
                        println!("-> {}", to_string(retval));
                    }
                    Err(msg) => {
                        println!("Error {}", msg);
                    }
                }
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
