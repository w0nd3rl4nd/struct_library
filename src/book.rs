#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    year: u16,
    is_checked_out: bool
}

impl Book {
    pub fn new(title: String, author: String, year: u16) -> Book {
        return Book {
            title,
            author,
            year,
            is_checked_out: false
        };
    }

    pub fn check_out(&mut self) {
        if self.is_checked_out {
            panic!("Book {} is already checked out", self.title)
        }
        self.is_checked_out = true;
        println!("Book {} is now checked out", self.title);
    }

    pub fn return_book(&mut self) {
        if !self.is_checked_out {
            panic!("Book {} is already in the library", self.title)
        }
        self.is_checked_out = false;
        println!("Book {} returned", self.title);
    }

    pub fn display(&self) -> String {
        return format!("Book: {}, Author: {}, Year: {}, is_checked_out: {}", 
            self.title,
            self.author,
            self.year,
            self.is_checked_out
        );
    }
}