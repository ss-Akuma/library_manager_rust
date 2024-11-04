use std::collections::HashMap;
use crate::books::Book;

pub struct Book_db {
    Books:HashMap<String,Book>
}

impl Book_db {
    fn new() -> Self{
        Book_db{
            Books:HashMap::new()
        }
    }

    pub fn book_add(&mut self,name:&str,year:u32,author:&str){
        let book = Book::new(name.to_string(),year,author.to_string());
        self.Books.insert(name.to_string(), book);
    }

    pub fn book_delete(){
        
    }
}