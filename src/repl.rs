use crate::ast;
use crate::lexer;
use crate::token;
use std::io;
use std::io::Write;

pub fn start(reader: &mut io::Stdin) {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        reader
            .read_line(&mut buf)
            .expect("Couldn't read from reader");

        let mut l = lexer::new(buf);

        let mut out_buf = String::new();

        loop {
            let tok = l.next_token();
            if tok.token_type == token::EOF {
                break;
            }
            out_buf.push_str(format!("{} ", tok.literal).as_str());
        }

        println!("{out_buf}");
    }
}
