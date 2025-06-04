use std::{io, process};
mod library;
mod book;
use crate::library::Library;
use crate::book::Book;

fn main() {
    let mut choice: u8 = 0;

    loop {
        choice = menu();
        match choice {
            1=>Library::add_book(),
            2=>println!("Show all books"),
            3=>println!("Borrow book"),
            4=>println!("Return book"),
            5=> {
                println!("Bye bye!");
                process::exit(0);
            }
            _=>println!("Haha, very funny. Now choose.")
        }
    }
}

fn menu () -> u8 {
    println!("Choose an option!");
    println!("1. Add new book");
    println!("2. View all books");
    println!("3. Borrow book");
    println!("4. Return book");
    println!("5. Exit library");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); 

    return parse_string(&input);
}

fn parse_string(s: &str) -> u8 {
    match s.trim().parse::<u8>() {
        Ok(n) => return n,
        Err(e) => if (s == "\n") {
            return 0;
        } else {
            panic!("Invalid string: {}", e)
        }
    }
}