use clap::builder::Str;

pub struct Book {
    name : String,
    author: String,
    no_of_pages: String,
    file_type: String,
    publisher: String,
    year: String,
    language: String
}

impl  Book{
    pub fn new(
        name: String, 
        author: String, 
        no_of_pages: String, 
        file_type: String, 
        publisher: String, 
        year: String, 
        language: String
    ) -> Book {
        Book {
            name,
            author,
            no_of_pages,
            file_type,
            publisher,
            year,
            language
        }
    }

    pub fn print(&self) {
        println!("{}Name         :- {}", '\t', self.name);        
        println!("{}Author       :- {}", '\t', self.author);        
        println!("{}No. of Pages :- {}", '\t', self.no_of_pages);        
        println!("{}File Type    :- {}", '\t', self.file_type);        
        println!("{}Publisher    :- {}", '\t', self.publisher);        
        println!("{}Year         :- {}", '\t', self.year);        
        println!("{}Language     :- {}", '\t', self.language);        
    }

}