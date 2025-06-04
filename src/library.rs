use crate::book::{self, Book};

pub struct Library {
    shelf: Vec<book::Book>,
    storage_value: u16
}

impl Library {
    // Function to append new book
    pub fn add_book() {
        let randomBook = Book::new(
        String::from("The name of the Wind"),
        String::from("Patrick Rothfuss"),
        2009
        );

        println!("{:#?}", randomBook);
    }
    

    // Function to return all books (display title and id)

    // Funtion to check out a book

    // Function to display a book (display title and id, user chooses id, gets details)
}