use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

use rlox::Scanner;
use crate::rlox::StdErrErrorHandler;

mod rlox;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_name: &str) {
    println!("Reading {}", file_name);
    let data = fs::read_to_string(file_name).unwrap();
    run(&data);
}

fn run_prompt() {
    let input = io::stdin();
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        input.read_line(&mut line).unwrap();
        run(&line);
    }
}

fn run(program: &str) {
    let mut scanner = Scanner::new(program, &StdErrErrorHandler {});
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{:?}", token);
    }
}
