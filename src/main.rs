use std::{io};

struct Book {
    title: String,
    author: String,
    year: u16,
    is_checked_out: bool
}

impl Book {
    fn new(title: String, author: String, year: u16) -> Book {
        return Book {
            title,
            author,
            year,
            is_checked_out: false
        };
    }

    fn check_out(&mut self) {
        if self.is_checked_out {
            panic!("Book {} is already checked out", self.title)
        }
        self.is_checked_out = true;
        println!("Book {} is now checked out", self.title);
    }

    fn return_book(&mut self) {
        if !self.is_checked_out {
            panic!("Book {} is already in the library", self.title)
        }
        self.is_checked_out = false;
        println!("Book {} returned", self.title);
    }

    fn display(&self) -> String {
        return format!("Book: {}, Author: {}, Year: {}, is_checked_out: {}", 
            self.title,
            self.author,
            self.year,
            self.is_checked_out
        );
    }
}

fn main() {
    let mut library: Vec<Book> = Vec::new();
    let mut choice: u8 = 0;

    loop {
        choice = menu();
        match choice {
            1=>println!("Add book "),
            2=>println!("Show all books"),
            3=>println!("Borrow book"),
            4=>println!("Return book"),
            _=>println!("Wildcard pressed")
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
        Err(e) => panic!("{}", e),
    }
}