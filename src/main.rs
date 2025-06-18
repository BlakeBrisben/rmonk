mod lexer;
mod token;

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let s = "Thing thingy thing";
    let slice = s.get(1..4).unwrap();
    let slice2 = &s[1..4];

    let map = HashMap::from([("Thing", "thing"), ("one", "one"), ("TWO", "two")]);

    let search = "one";
    match map.get(search) {
        Some(&t) => println!("{t}"),
        None => println!("NONE"),
    };

    println!("{slice}: {slice2}");

    let tok = token::lookup_ident(String::from("thing"));

    println!("{tok}");

    let tok = token::STRING;

    let mut l = lexer::new(String::from("= 5;"));

    let tok = l.next_token();

    println!("{}", tok.literal);
    println!("{}", tok.token_type);
}
