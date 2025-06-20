mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    repl::start(&mut stdin);
}
