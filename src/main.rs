#![allow(dead_code)]
#[macro_use] extern crate enum_ordinalize;

use std::io;

mod graphics;
mod chess;
mod bitboard_generator;

fn main() {
    println!("Game | Gen | Exit (1|2|3)");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<u32>() {
            Ok(1) => graphics::init(),
            Ok(2) => bitboard_generator::init(),
            Ok(3) => break,
            _ => println!("Not a valid input!"),
        }
    }
}